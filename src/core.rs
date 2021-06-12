use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use crate::ffi;
use crate::linker::DynamicLibrary;

struct DispatchLoaderInstance {
    vk_get_instance_proc_addr: ffi::PFN_vkGetInstanceProcAddr,
    vk_enumerate_instance_version: Option<ffi::PFN_vkEnumerateInstanceVersion>,
    vk_create_instance: ffi::PFN_vkCreateInstance,
    vk_destroy_instance: ffi::PFN_vkDestroyInstance,
}

pub struct Instance {
    handle: NonNull<ffi::OpaqueInstance>,
    dispatch_loader: DispatchLoaderInstance,
    _lib: DynamicLibrary,
    _marker: PhantomData<ffi::OpaqueInstance>,
}

#[derive(Default)]
pub struct ApplicationInfo {
    pub application_name: Option<String>,
    pub application_version: ApiVersion,
    pub engine_name: Option<String>,
    pub engine_version: ApiVersion,
    pub api_version: ApiVersion,
}

#[derive(Default)]
pub struct ApiVersion(u32);

impl Instance {
    pub fn new() -> Self {
        let lib = DynamicLibrary::new("vulkan-1.dll");
        let vk_get_instance_proc_addr: ffi::PFN_vkGetInstanceProcAddr =
            unsafe { std::mem::transmute(lib.get_proc_addr("vkGetInstanceProcAddr")) };
        let vk_create_instance: ffi::PFN_vkCreateInstance = unsafe {
            vk_get_instance_proc_addr(std::ptr::null_mut(), "vkCreateInstance\0".as_ptr().cast())
                .map(|pfn| std::mem::transmute(pfn))
                .expect("Could not get address of Vulkan command.")
        };

        let create_info = ffi::InstanceCreateInfo {
            s_type: ffi::StructureType::InstanceCreateInfo,
            p_next: std::ptr::null(),
            flags: 0,
            p_application_info: std::ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: std::ptr::null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: std::ptr::null(),
        };

        let mut handle = MaybeUninit::uninit();
        let result =
            unsafe { vk_create_instance(&create_info, std::ptr::null(), handle.as_mut_ptr()) };

        if result == 0 {
            let handle = unsafe { handle.assume_init() };
            let vk_enumerate_instance_version: Option<ffi::PFN_vkEnumerateInstanceVersion> = unsafe {
                vk_get_instance_proc_addr(handle, "vkEnumerateInstanceVersion\0".as_ptr().cast())
                    .map(|pfn| std::mem::transmute(pfn))
            };
            let vk_destroy_instance: ffi::PFN_vkDestroyInstance = unsafe {
                vk_get_instance_proc_addr(handle, "vkDestroyInstance\0".as_ptr().cast())
                    .map(|pfn| std::mem::transmute(pfn))
                    .expect("Could not get address of Vulkan command.")
            };
            println!("Successfully created instance: {:p}", handle);
            let dispatch_loader = DispatchLoaderInstance {
                vk_get_instance_proc_addr,
                vk_enumerate_instance_version,
                vk_create_instance,
                vk_destroy_instance,
            };

            Self {
                handle: unsafe { NonNull::new_unchecked(handle) },
                dispatch_loader,
                _lib: lib,
                _marker: PhantomData,
            }
        } else {
            panic!("Returned {}", result);
        }
    }

    pub fn version(&self) -> Option<ApiVersion> {
        self.dispatch_loader
            .vk_enumerate_instance_version
            .map(|func| {
                let mut version = MaybeUninit::uninit();
                unsafe {
                    func(version.as_mut_ptr());
                    ApiVersion(version.assume_init())
                }
            })
    }

    fn _get_proc_addr(&self, name: &str) -> Option<ffi::PFN_vkVoidFunction> {
        let name_c = CString::new(name).unwrap();
        unsafe {
            (self.dispatch_loader.vk_get_instance_proc_addr)(self.handle.as_ptr(), name_c.as_ptr())
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        println!("Dropped Instance");
        unsafe {
            (self.dispatch_loader.vk_destroy_instance)(self.handle.as_ptr(), std::ptr::null());
        }
    }
}

impl ApiVersion {
    pub const V1_0: Self = Self::new(0, 1, 0, 0);
    pub const V1_1: Self = Self::new(0, 1, 1, 0);
    pub const V1_2: Self = Self::new(0, 1, 2, 0);

    pub const fn new(variant: u32, major: u32, minor: u32, patch: u32) -> Self {
        Self((variant << 29) | (major << 22) | (minor << 12) | patch)
    }

    pub fn major(&self) -> u32 {
        (self.0 >> 22) & 0x7F
    }

    pub fn minor(&self) -> u32 {
        (self.0 >> 12) & 0x3FF
    }

    pub fn patch(&self) -> u32 {
        self.0 & 0xFFF
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}