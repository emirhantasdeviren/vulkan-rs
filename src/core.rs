use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use crate::ffi;
use crate::linker::DynamicLibrary;

pub const KHR_SURFACE_EXTENSION_NAME: &str = "VK_KHR_surface";
#[cfg(target_os = "linux")]
pub const KHR_XCB_SURFACE_EXTENSION_NAME: &str = "VK_KHR_xcb_surface";

pub struct Instance {
    handle: NonNull<ffi::OpaqueInstance>,
    dispatch_loader: DispatchLoaderInstance,
    _lib: DynamicLibrary,
    _marker: PhantomData<ffi::OpaqueInstance>,
}

pub struct PhysicalDevice<'a> {
    handle: NonNull<ffi::OpaquePhysicalDevice>,
    dispatch_loader: DispatchLoaderPhysicalDevice,
    _marker: PhantomData<(ffi::OpaquePhysicalDevice, &'a Instance)>,
}

pub struct Device<'a> {
    handle: NonNull<ffi::OpaqueDevice>,
    dispatch_loader: DispatchLoaderDevice,
    _marker: PhantomData<(ffi::OpaqueDevice, &'a Instance)>,
}

pub struct Queue<'a> {
    handle: NonNull<ffi::OpaqueQueue>,
    _marker: PhantomData<(ffi::OpaqueQueue, &'a Device<'a>)>,
}

#[derive(Default)]
struct DispatchLoaderInstance {
    vk_get_instance_proc_addr: Option<ffi::PFN_vkGetInstanceProcAddr>,
    vk_enumerate_instance_version: Option<ffi::PFN_vkEnumerateInstanceVersion>,
    vk_create_instance: Option<ffi::PFN_vkCreateInstance>,
    vk_destroy_instance: Option<ffi::PFN_vkDestroyInstance>,
    vk_enumerate_physical_devices: Option<ffi::PFN_vkEnumeratePhysicalDevices>,
}

struct DispatchLoaderPhysicalDevice {
    vk_get_physical_device_properties: ffi::PFN_vkGetPhysicalDeviceProperties,
    vk_get_physical_device_queue_family_properties:
        ffi::PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    vk_create_device: ffi::PFN_vkCreateDevice,
    vk_get_device_proc_addr: ffi::PFN_vkGetDeviceProcAddr,
}

struct DispatchLoaderDevice {
    vk_destroy_device: ffi::PFN_vkDestroyDevice,
    vk_get_device_queue: ffi::PFN_vkGetDeviceQueue,
}

#[derive(PartialEq, Eq)]
pub enum PhysicalDeviceType {
    Other,
    IntegratedGpu,
    DiscreteGpu,
    VirtualGpu,
    Cpu,
}

#[derive(Default)]
pub struct ApplicationInfo {
    pub application_name: Option<String>,
    pub application_version: ApiVersion,
    pub engine_name: Option<String>,
    pub engine_version: ApiVersion,
    pub api_version: ApiVersion,
}

pub struct PhysicalDeviceProperties {
    pub api_version: ApiVersion,
    pub device_type: PhysicalDeviceType,
    pub device_name: String,
}

pub struct QueueFamilyProperties {
    pub queue_flags: ffi::QueueFlags,
    pub queue_count: u32,
}

#[derive(Default, Clone, Copy)]
pub struct ApiVersion(u32);

impl Instance {
    pub fn new(
        application_info: Option<&ApplicationInfo>,
        layers: Option<&[&str]>,
        extensions: Option<&[&str]>,
    ) -> Self {
        let file_name = if cfg!(unix) {
            "libvulkan.so"
        } else if cfg!(windows) {
            "vulkan-1.dll"
        } else {
            ""
        };
        let lib = DynamicLibrary::new(file_name);
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

        let layers_c: Option<Vec<CString>> = layers.map(|l| {
            l.iter()
                .map(|name| CString::new(name.as_bytes()).unwrap())
                .collect()
        });
        let extensions_c: Option<Vec<CString>> = extensions.map(|e| {
            e.iter()
                .map(|name| CString::new(name.as_bytes()).unwrap())
                .collect()
        });

        let layer_ptrs: Option<Vec<*const i8>> = layers_c
            .as_ref()
            .map(|l| l.iter().map(|name| name.as_ptr()).collect());
        let extension_ptrs: Option<Vec<*const i8>> = extensions_c
            .as_ref()
            .map(|e| e.iter().map(|name| name.as_ptr()).collect());

        let enabled_layer_count = layer_ptrs.as_ref().map_or(0, |ptrs| ptrs.len() as u32);
        let enabled_extension_count = extension_ptrs.as_ref().map_or(0, |ptrs| ptrs.len() as u32);

        let pp_enabled_layer_names = layer_ptrs
            .as_ref()
            .map_or(std::ptr::null(), |ptrs| ptrs.as_ptr());
        let pp_enabled_extension_names = extension_ptrs
            .as_ref()
            .map_or(std::ptr::null(), |ptrs| ptrs.as_ptr());

        let create_info = ffi::InstanceCreateInfo {
            s_type: ffi::StructureType::InstanceCreateInfo,
            p_next: std::ptr::null(),
            flags: 0,
            p_application_info,
            enabled_layer_count,
            pp_enabled_layer_names,
            enabled_extension_count,
            pp_enabled_extension_names,
        };

        let mut handle = MaybeUninit::uninit();
        let result = unsafe {
            (dispatch_loader.vk_create_instance.unwrap())(
                &create_info,
                std::ptr::null(),
                handle.as_mut_ptr(),
            )
        };

        if result == ffi::Result::Success {
            let handle = unsafe { handle.assume_init() };
            unsafe { dispatch_loader.load(handle) };

            Self {
                handle: unsafe { NonNull::new_unchecked(handle) },
                dispatch_loader,
                _lib: lib,
                _marker: PhantomData,
            }
        } else {
            panic!("Returned {:?}", result);
        }
    }

    pub fn enumerate_physical_devices(&self) -> Vec<PhysicalDevice<'_>> {
        let mut physical_device_count = MaybeUninit::uninit();
        let vk_enumerate_physical_devices =
            self.dispatch_loader.vk_enumerate_physical_devices.unwrap();
        let result = unsafe {
            vk_enumerate_physical_devices(
                self.handle.as_ptr(),
                physical_device_count.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };

        if result == ffi::Result::Success {
            let capacity = unsafe { physical_device_count.assume_init() as usize };
            let mut physical_devices = Vec::with_capacity(capacity);
            let result = unsafe {
                vk_enumerate_physical_devices(
                    self.handle.as_ptr(),
                    physical_device_count.as_mut_ptr(),
                    physical_devices.as_mut_ptr(),
                )
            };
            if result == ffi::Result::Success {
                let new_len = unsafe { physical_device_count.assume_init() as usize };
                unsafe { physical_devices.set_len(new_len) };
                physical_devices
                    .into_iter()
                    .map(|p| PhysicalDevice {
                        handle: unsafe { NonNull::new_unchecked(p) },
                        dispatch_loader: DispatchLoaderPhysicalDevice::new(self),
                        _marker: PhantomData,
                    })
                    .collect()
            } else {
                panic!("Could not write phsyical devices to Vec. {:?}", result);
            }
        } else {
            panic!("Could not get physical device count.");
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

impl<'a> PhysicalDevice<'a> {
    pub fn properties(&self) -> PhysicalDeviceProperties {
        let mut props = MaybeUninit::uninit();
        unsafe {
            (self.dispatch_loader.vk_get_physical_device_properties)(
                self.handle.as_ptr(),
                props.as_mut_ptr(),
            )
        };
        let props = unsafe { props.assume_init() };
        let device_type = match props.device_type {
            ffi::PhysicalDeviceType::Other => self::PhysicalDeviceType::Other,
            ffi::PhysicalDeviceType::IntegratedGpu => self::PhysicalDeviceType::IntegratedGpu,
            ffi::PhysicalDeviceType::DiscreteGpu => self::PhysicalDeviceType::DiscreteGpu,
            ffi::PhysicalDeviceType::VirtualGpu => self::PhysicalDeviceType::VirtualGpu,
            ffi::PhysicalDeviceType::Cpu => self::PhysicalDeviceType::Cpu,
            ffi::PhysicalDeviceType::MaxEnum => panic!("MAX ENUM?"),
        };
        // NOTE: Since `device_name` is UTF-8 null-terminated string according to Vulkan manual.
        // We don't have to check the validation of UTF-8. Find a better way.
        let device_name_cstr = unsafe { CStr::from_ptr(props.device_name.as_ptr()) };
        let device_name = String::from(device_name_cstr.to_str().unwrap());

        PhysicalDeviceProperties {
            api_version: ApiVersion(props.api_version),
            device_type,
            device_name,
        }
    }

    pub fn queue_family_properties(&self) -> Vec<QueueFamilyProperties> {
        let mut queue_family_count = MaybeUninit::uninit();
        unsafe {
            (self
                .dispatch_loader
                .vk_get_physical_device_queue_family_properties)(
                self.handle.as_ptr(),
                queue_family_count.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };
        let capacity = unsafe { queue_family_count.assume_init() };
        let mut props_ffi = Vec::with_capacity(capacity as usize);
        unsafe {
            (self
                .dispatch_loader
                .vk_get_physical_device_queue_family_properties)(
                self.handle.as_ptr(),
                queue_family_count.as_mut_ptr(),
                props_ffi.as_mut_ptr(),
            )
        };
        let new_len = unsafe { queue_family_count.assume_init() };
        unsafe { props_ffi.set_len(new_len as usize) };

        props_ffi
            .into_iter()
            .map(|p| QueueFamilyProperties {
                queue_flags: p.queue_flags,
                queue_count: p.queue_count,
            })
            .collect()
    }

    pub fn create_device(
        &self,
        queue_family_indices: &[usize],
        priorities: &[&[f32]],
    ) -> Device<'a> {
        let queue_create_infos: Vec<ffi::DeviceQueueCreateInfo> = queue_family_indices
            .iter()
            .zip(priorities.iter())
            .map(|(index, queue_priorities)| ffi::DeviceQueueCreateInfo {
                s_type: ffi::StructureType::DeviceQueueCreateInfo,
                p_next: std::ptr::null(),
                flags: 0,
                queue_family_index: *index as u32,
                queue_count: queue_priorities.len() as u32,
                p_queue_priorities: queue_priorities.as_ptr(),
            })
            .collect();

        let create_info = ffi::DeviceCreateInfo {
            s_type: ffi::StructureType::DeviceCreateInfo,
            p_next: std::ptr::null(),
            flags: 0,
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: std::ptr::null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: std::ptr::null(),
            p_enabled_features: std::ptr::null(),
        };

        let mut device_handle = MaybeUninit::uninit();
        let result = unsafe {
            (self.dispatch_loader.vk_create_device)(
                self.handle.as_ptr(),
                &create_info,
                std::ptr::null(),
                device_handle.as_mut_ptr(),
            )
        };

        if result == ffi::Result::Success {
            let device_handle = unsafe { device_handle.assume_init() };
            Device {
                handle: unsafe { NonNull::new_unchecked(device_handle) },
                dispatch_loader: unsafe {
                    DispatchLoaderDevice::new(
                        self.dispatch_loader.vk_get_device_proc_addr,
                        device_handle,
                    )
                },
                _marker: PhantomData,
            }
        } else {
            panic!("Could not create logical device. {:?}", result);
        }
    }
}

impl<'a> Device<'a> {
    pub fn get_device_queue(&self, queue_family_index: usize, queue_index: usize) -> Queue<'_> {
        let mut handle = MaybeUninit::uninit();
        unsafe {
            (self.dispatch_loader.vk_get_device_queue)(
                self.handle.as_ptr(),
                queue_family_index as u32,
                queue_index as u32,
                handle.as_mut_ptr(),
            );
        }

        Queue {
            handle: unsafe { NonNull::new_unchecked(handle.assume_init()) },
            _marker: PhantomData,
        }
    }
}

impl<'a> Drop for Device<'a> {
    fn drop(&mut self) {
        println!("Dropped Device");
        unsafe { (self.dispatch_loader.vk_destroy_device)(self.handle.as_ptr(), std::ptr::null()) }
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
        self.vk_enumerate_physical_devices =
            vk_get_instance_proc_addr(instance, "vkEnumeratePhysicalDevices\0".as_ptr().cast())
                .map(|pfn| std::mem::transmute(pfn));
    }
}

impl DispatchLoaderPhysicalDevice {
    fn new(instance: &Instance) -> Self {
        let vk_get_instance_proc_addr = instance.dispatch_loader.vk_get_instance_proc_addr.unwrap();

        // SAFETY: Since we have reference of `self::Instance`, which has a valid instance handle
        // We can safely receive function addresses.
        // String typos are my responsibility.
        unsafe {
            Self {
                vk_get_physical_device_properties: vk_get_instance_proc_addr(
                    instance.handle.as_ptr(),
                    "vkGetPhysicalDeviceProperties\0".as_ptr().cast(),
                )
                .map(|pfn| std::mem::transmute(pfn))
                .unwrap(),
                vk_get_physical_device_queue_family_properties: vk_get_instance_proc_addr(
                    instance.handle.as_ptr(),
                    "vkGetPhysicalDeviceQueueFamilyProperties\0".as_ptr().cast(),
                )
                .map(|pfn| std::mem::transmute(pfn))
                .unwrap(),
                vk_create_device: vk_get_instance_proc_addr(
                    instance.handle.as_ptr(),
                    "vkCreateDevice\0".as_ptr().cast(),
                )
                .map(|pfn| std::mem::transmute(pfn))
                .unwrap(),
                vk_get_device_proc_addr: vk_get_instance_proc_addr(
                    instance.handle.as_ptr(),
                    "vkGetDeviceProcAddr\0".as_ptr().cast(),
                )
                .map(|pfn| std::mem::transmute(pfn))
                .unwrap(),
            }
        }
    }
}

impl DispatchLoaderDevice {
    // SAFETY: Caller must ensure that device handle is non-null valid VkDevice
    unsafe fn new(
        vk_get_device_proc_addr: ffi::PFN_vkGetDeviceProcAddr,
        device_handle: *mut ffi::OpaqueDevice,
    ) -> Self {
        Self {
            vk_destroy_device: vk_get_device_proc_addr(
                device_handle,
                "vkDestroyDevice\0".as_ptr().cast(),
            )
            .map(|pfn| std::mem::transmute(pfn))
            .unwrap(),
            vk_get_device_queue: vk_get_device_proc_addr(
                device_handle,
                "vkGetDeviceQueue\0".as_ptr().cast(),
            )
            .map(|pfn| std::mem::transmute(pfn))
            .unwrap(),
        }
    }
}

impl QueueFamilyProperties {
    pub fn graphics(&self) -> bool {
        (self.queue_flags & 0x00000001) != 0
    }

    pub fn compute(&self) -> bool {
        (self.queue_flags & 0x00000002) != 0
    }

    pub fn transfer(&self) -> bool {
        (self.queue_flags & 0x00000004) != 0
    }

    pub fn sparse_binding(&self) -> bool {
        (self.queue_flags & 0x00000008) != 0
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
