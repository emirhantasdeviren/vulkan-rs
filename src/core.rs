use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use crate::ffi;
use crate::linker::DynamicLibrary;

pub struct Instance {
    handle: NonNull<ffi::OpaqueInstance>,
    dispatch_loader: DispatchLoaderInstance,
    _lib: DynamicLibrary,
    _marker: PhantomData<ffi::OpaqueInstance>,
}

#[derive(Default)]
struct DispatchLoaderInstance {
    vk_get_instance_proc_addr: Option<ffi::PFN_vkGetInstanceProcAddr>,
    vk_enumerate_instance_version: Option<ffi::PFN_vkEnumerateInstanceVersion>,
    vk_create_instance: Option<ffi::PFN_vkCreateInstance>,
    vk_destroy_instance: Option<ffi::PFN_vkDestroyInstance>,
}

#[derive(Default)]
pub struct ApplicationInfo {
    pub application_name: Option<String>,
    pub application_version: ApiVersion,
    pub engine_name: Option<String>,
    pub engine_version: ApiVersion,
    pub api_version: ApiVersion,
}

#[derive(Default, Clone, Copy)]
pub struct ApiVersion(u32);

impl Instance {
    pub fn new(
        application_info: Option<&ApplicationInfo>,
        _layers: Option<&[&str]>,
        _extensions: Option<&[&str]>,
    ) -> Self {
        let lib = DynamicLibrary::new("libvulkan.so");
        let vk_get_instance_proc_addr: ffi::PFN_vkGetInstanceProcAddr =
            unsafe { std::mem::transmute(lib.get_proc_addr("vkGetInstanceProcAddr")) };
        let mut dispatch_loader = DispatchLoaderInstance::new(vk_get_instance_proc_addr);

        let names_c = application_info.map(|i| {
            (
                i.application_name
                    .as_ref()
                    .map(|name| CString::new(name.as_bytes()).unwrap()),
                i.engine_name
                    .as_ref()
                    .map(|name| CString::new(name.as_bytes()).unwrap()),
            )
        });

        let app_info_c =
            application_info
                .zip(names_c.as_ref())
                .map(|(i, (app_name, engine_name))| {
                    let p_application_name = app_name
                        .as_ref()
                        .map_or(std::ptr::null(), |name| name.as_ptr());
                    let p_engine_name = engine_name
                        .as_ref()
                        .map_or(std::ptr::null(), |name| name.as_ptr());

                    ffi::ApplicationInfo {
                        s_type: ffi::StructureType::ApplicationInfo,
                        p_next: std::ptr::null(),
                        p_application_name,
                        application_version: i.application_version.0,
                        p_engine_name,
                        engine_version: i.engine_version.0,
                        api_version: i.api_version.0,
                    }
                });

        let p_application_info = app_info_c.as_ref().map_or(std::ptr::null(), |i| i);

        let create_info = ffi::InstanceCreateInfo {
            s_type: ffi::StructureType::InstanceCreateInfo,
            p_next: std::ptr::null(),
            flags: 0,
            p_application_info,
            enabled_layer_count: 0,
            pp_enabled_layer_names: std::ptr::null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: std::ptr::null(),
        };

        let mut handle = MaybeUninit::uninit();
        let result = unsafe {
            (dispatch_loader.vk_create_instance.unwrap())(
                &create_info,
                std::ptr::null(),
                handle.as_mut_ptr(),
            )
        };

        if result == 0 {
            let handle = unsafe { handle.assume_init() };
            unsafe { dispatch_loader.load(handle) };
            println!("Successfully created instance: {:p}", handle);

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
}

impl Drop for Instance {
    fn drop(&mut self) {
        println!("Dropped Instance");
        unsafe {
            (self.dispatch_loader.vk_destroy_instance.unwrap())(
                self.handle.as_ptr(),
                std::ptr::null(),
            );
        }
    }
}

impl DispatchLoaderInstance {
    fn new(vk_get_instance_proc_addr: ffi::PFN_vkGetInstanceProcAddr) -> Self {
        Self {
            vk_get_instance_proc_addr: Some(vk_get_instance_proc_addr),
            vk_enumerate_instance_version: unsafe {
                vk_get_instance_proc_addr(
                    std::ptr::null_mut(),
                    "vkEnumerateInstanceVersion\0".as_ptr().cast(),
                )
                .map(|pfn| std::mem::transmute(pfn))
            },
            vk_create_instance: unsafe {
                vk_get_instance_proc_addr(
                    std::ptr::null_mut(),
                    "vkCreateInstance\0".as_ptr().cast(),
                )
                .map(|pfn| std::mem::transmute(pfn))
            },
            ..Default::default()
        }
    }

    unsafe fn load(&mut self, instance: *mut ffi::OpaqueInstance) {
        let vk_get_instance_proc_addr = self.vk_get_instance_proc_addr.unwrap();
        self.vk_destroy_instance =
            vk_get_instance_proc_addr(instance, "vkDestroyInstance\0".as_ptr().cast())
                .map(|pfn| std::mem::transmute(pfn));
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
