use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use crate::core::{Error, Result};
use crate::device::PhysicalDevice;
use crate::ffi;
use crate::linker::DynamicLibrary;
use crate::wsi::SurfaceKhr;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub struct Instance {
    pub(super) handle: NonNull<ffi::OpaqueInstance>,
    pub(super) dispatch_loader: DispatchLoaderInstance,
    _lib: DynamicLibrary,
    _marker: PhantomData<ffi::OpaqueInstance>,
}

#[derive(Default)]
pub struct InstanceBuilder<'a> {
    application_info: Option<&'a ApplicationInfo>,
    layers: Option<&'a [&'a str]>,
    extensions: Option<&'a [&'a str]>,
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

#[derive(Default)]
pub(crate) struct DispatchLoaderInstance {
    pub(crate) vk_get_instance_proc_addr: Option<ffi::PFN_vkGetInstanceProcAddr>,
    pub(crate) vk_enumerate_instance_version: Option<ffi::PFN_vkEnumerateInstanceVersion>,
    pub(crate) vk_create_instance: Option<ffi::PFN_vkCreateInstance>,
    pub(crate) vk_destroy_instance: Option<ffi::PFN_vkDestroyInstance>,
    pub(crate) vk_enumerate_physical_devices: Option<ffi::PFN_vkEnumeratePhysicalDevices>,
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub(crate) vk_create_xcb_surface_khr: Option<ffi::PFN_vkCreateXcbSurfaceKHR>,
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub(crate) vk_create_xlib_surface_khr: Option<ffi::PFN_vkCreateXlibSurfaceKHR>,
    #[cfg(target_os = "windows")]
    pub(crate) vk_create_win32_surface_khr: Option<ffi::PFN_vkCreateWin32SurfaceKHR>,
    pub(crate) vk_destroy_surface_khr: Option<ffi::PFN_vkDestroySurfaceKHR>,
}

pub(crate) struct DispatchLoaderPhysicalDevice {
    pub(crate) vk_get_physical_device_properties: ffi::PFN_vkGetPhysicalDeviceProperties,
    pub(crate) vk_get_physical_device_queue_family_properties:
        ffi::PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    pub(crate) vk_create_device: ffi::PFN_vkCreateDevice,
    pub(crate) vk_get_device_proc_addr: ffi::PFN_vkGetDeviceProcAddr,
    pub(crate) vk_get_physical_device_surface_capabilities_khr:
        Option<ffi::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>,
    pub(crate) vk_get_physical_device_surface_formats_khr:
        Option<ffi::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>,
    pub(crate) vk_get_physical_device_surface_present_modes_khr:
        Option<ffi::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>,
}

pub(crate) struct DispatchLoaderDevice {
    pub(crate) vk_destroy_device: ffi::PFN_vkDestroyDevice,
    pub(crate) vk_get_device_queue: ffi::PFN_vkGetDeviceQueue,
    pub(crate) vk_create_command_pool: ffi::PFN_vkCreateCommandPool,
    pub(crate) vk_destroy_command_pool: ffi::PFN_vkDestroyCommandPool,
    pub(crate) vk_allocate_command_buffers: ffi::PFN_vkAllocateCommandBuffers,
    pub(crate) vk_create_semaphore: ffi::PFN_vkCreateSemaphore,
    pub(crate) vk_destroy_semaphore: ffi::PFN_vkDestroySemaphore,
    pub(crate) vk_create_swapchain_khr: Option<ffi::PFN_vkCreateSwapchainKHR>,
    pub(crate) vk_destroy_swapchain_khr: Option<ffi::PFN_vkDestroySwapchainKHR>,
    pub(crate) vk_get_swapchain_images_khr: Option<ffi::PFN_vkGetSwapchainImagesKHR>,
    pub(crate) vk_create_image_view: ffi::PFN_vkCreateImageView,
    pub(crate) vk_destroy_image_view: ffi::PFN_vkDestroyImageView,
}

impl Instance {
    pub fn new() -> Result<Self> {
        InstanceBuilder::new().build()
    }

    pub fn builder<'a>() -> InstanceBuilder<'a> {
        Default::default()
    }

    pub fn enumerate_physical_devices(&self) -> impl ExactSizeIterator<Item = PhysicalDevice<'_>> {
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
                physical_devices.into_iter().map(move |p| PhysicalDevice {
                    handle: unsafe { NonNull::new_unchecked(p) },
                    dispatch_loader: DispatchLoaderPhysicalDevice::new(self),
                    _marker: PhantomData,
                })
            } else {
                panic!("Could not write phsyical devices to Vec. {:?}", result);
            }
        } else {
            panic!("Could not get physical device count.");
        }
    }

    pub fn create_surface_khr(&self, window: &impl HasRawWindowHandle) -> SurfaceKhr<'_> {
        match window.raw_window_handle() {
            #[cfg(target_os = "windows")]
            RawWindowHandle::Windows(window_handle) => {
                let create_info = ffi::Win32SurfaceCreateInfoKhr {
                    s_type: ffi::StructureType::Win32SurfaceCreateInfoKhr,
                    p_next: std::ptr::null(),
                    flags: 0,
                    hinstance: window_handle.hinstance.cast(),
                    hwnd: window_handle.hwnd.cast(),
                };

                let mut handle = MaybeUninit::uninit();
                let result = unsafe {
                    (self.dispatch_loader.vk_create_win32_surface_khr.unwrap())(
                        self.handle.as_ptr(),
                        &create_info,
                        std::ptr::null(),
                        handle.as_mut_ptr(),
                    )
                };

                if result == ffi::Result::Success {
                    SurfaceKhr {
                        #[cfg(target_pointer_width = "64")]
                        handle: unsafe { NonNull::new_unchecked(handle.assume_init()) },
                        #[cfg(not(target_pointer_width = "64"))]
                        handle: unsafe { NonZeroU64::new_unchecked(handle.assume_init()) },
                        instance: self,
                        #[cfg(target_pointer_width = "64")]
                        _marker: PhantomData,
                    }
                } else {
                    panic!("Could not create VkSurfaceKHR")
                }
            }
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            RawWindowHandle::Xcb(window_handle) => {
                let create_info = ffi::XcbSurfaceCreateInfoKhr {
                    s_type: ffi::StructureType::XcbSurfaceCreateInfoKhr,
                    p_next: std::ptr::null(),
                    flags: 0,
                    connection: window_handle.connection.cast(),
                    window: window_handle.window,
                };

                let mut handle = MaybeUninit::uninit();
                let result = unsafe {
                    (self.dispatch_loader.vk_create_xcb_surface_khr.unwrap())(
                        self.handle.as_ptr(),
                        &create_info,
                        std::ptr::null(),
                        handle.as_mut_ptr(),
                    )
                };

                if result == ffi::Result::Success {
                    SurfaceKhr {
                        #[cfg(target_pointer_width = "64")]
                        handle: unsafe { NonNull::new_unchecked(handle.assume_init()) },
                        #[cfg(not(target_pointer_width = "64"))]
                        handle: unsafe { NonZeroU64::new_unchecked(handle.assume_init()) },
                        instance: self,
                        #[cfg(target_pointer_width = "64")]
                        _marker: PhantomData,
                    }
                } else {
                    panic!("Could not create VkSurfaceKHR")
                }
            }
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            RawWindowHandle::Xlib(window_handle) => {
                let create_info = ffi::XlibSurfaceCreateInfoKhr {
                    s_type: ffi::StructureType::XlibSurfaceCreateInfoKhr,
                    p_next: std::ptr::null(),
                    flags: 0,
                    dpy: window_handle.display.cast(),
                    window: window_handle.window,
                };

                let mut handle = MaybeUninit::uninit();
                let result = unsafe {
                    (self.dispatch_loader.vk_create_xlib_surface_khr.unwrap())(
                        self.handle.as_ptr(),
                        &create_info,
                        std::ptr::null(),
                        handle.as_mut_ptr(),
                    )
                };

                if result == ffi::Result::Success {
                    SurfaceKhr {
                        #[cfg(target_pointer_width = "64")]
                        handle: unsafe { NonNull::new_unchecked(handle.assume_init()) },
                        #[cfg(not(target_pointer_width = "64"))]
                        handle: unsafe { NonZeroU64::new_unchecked(handle.assume_init()) },
                        instance: self,
                        #[cfg(target_pointer_width = "64")]
                        _marker: PhantomData,
                    }
                } else {
                    panic!("Could not create VkSurfaceKHR")
                }
            }
            _ => unimplemented!(),
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

impl<'a> InstanceBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_application_info(&mut self, application_info: &'a ApplicationInfo) -> &mut Self {
        self.application_info = Some(application_info);
        self
    }

    pub fn with_layers(&mut self, layers: &'a [&'a str]) -> &mut Self {
        self.layers = Some(layers);
        self
    }

    pub fn with_extensions(&mut self, extensions: &'a [&'a str]) -> &mut Self {
        self.extensions = Some(extensions);
        self
    }

    pub fn build(&self) -> Result<Instance> {
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

        let names_c = self.application_info.map(|i| {
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
            self.application_info
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

        let layers_c: Option<Vec<CString>> = self.layers.map(|l| {
            l.iter()
                .map(|name| CString::new(name.as_bytes()).unwrap())
                .collect()
        });
        let extensions_c: Option<Vec<CString>> = self.extensions.map(|e| {
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

        match result {
            ffi::Result::Success => {
                let handle = unsafe { handle.assume_init() };
                unsafe { dispatch_loader.load(handle) };

                Ok(Instance {
                    handle: unsafe { NonNull::new_unchecked(handle) },
                    dispatch_loader,
                    _lib: lib,
                    _marker: PhantomData,
                })
            }
            ffi::Result::ErrorOutOfHostMemory => Err(Error::OutOfHostMemory),
            ffi::Result::ErrorOutOfDeviceMemory => Err(Error::OutOfDeviceMemory),
            ffi::Result::ErrorInitializationFailed => Err(Error::InitializationFailed),
            ffi::Result::ErrorLayerNotPresent => Err(Error::LayerNotPresent),
            ffi::Result::ErrorExtensionNotPresent => Err(Error::ExtensionNotPresent),
            ffi::Result::ErrorIncompatibleDriver => Err(Error::IncompatibleDriver),
            _ => unreachable!(),
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

impl From<u32> for ApiVersion {
    fn from(version: u32) -> Self {
        Self(version)
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

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        {
            self.vk_create_xcb_surface_khr =
                vk_get_instance_proc_addr(instance, "vkCreateXcbSurfaceKHR\0".as_ptr().cast())
                    .map(|pfn| std::mem::transmute(pfn));
            self.vk_create_xlib_surface_khr =
                vk_get_instance_proc_addr(instance, "vkCreateXlibSurfaceKHR\0".as_ptr().cast())
                    .map(|pfn| std::mem::transmute(pfn));
        }
        #[cfg(target_os = "windows")]
        {
            self.vk_create_win32_surface_khr =
                vk_get_instance_proc_addr(instance, "vkCreateWin32SurfaceKHR\0".as_ptr().cast())
                    .map(|pfn| std::mem::transmute(pfn));
        }

        self.vk_destroy_surface_khr =
            vk_get_instance_proc_addr(instance, "vkDestroySurfaceKHR\0".as_ptr().cast())
                .map(|pfn| std::mem::transmute(pfn));
    }
}

impl DispatchLoaderPhysicalDevice {
    pub(crate) fn new(instance: &Instance) -> Self {
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
                vk_get_physical_device_surface_capabilities_khr: vk_get_instance_proc_addr(
                    instance.handle.as_ptr(),
                    "vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0"
                        .as_ptr()
                        .cast(),
                )
                .map(|pfn| std::mem::transmute(pfn)),
                vk_get_physical_device_surface_formats_khr: vk_get_instance_proc_addr(
                    instance.handle.as_ptr(),
                    "vkGetPhysicalDeviceSurfaceFormatsKHR\0".as_ptr().cast(),
                )
                .map(|pfn| std::mem::transmute(pfn)),
                vk_get_physical_device_surface_present_modes_khr: vk_get_instance_proc_addr(
                    instance.handle.as_ptr(),
                    "vkGetPhysicalDeviceSurfacePresentModesKHR\0"
                        .as_ptr()
                        .cast(),
                )
                .map(|pfn| std::mem::transmute(pfn)),
            }
        }
    }
}

impl DispatchLoaderDevice {
    // SAFETY: Caller must ensure that device handle is non-null valid VkDevice
    pub(crate) unsafe fn new(
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
            vk_create_command_pool: vk_get_device_proc_addr(
                device_handle,
                "vkCreateCommandPool\0".as_ptr().cast(),
            )
            .map(|pfn| std::mem::transmute(pfn))
            .unwrap(),
            vk_destroy_command_pool: vk_get_device_proc_addr(
                device_handle,
                "vkDestroyCommandPool\0".as_ptr().cast(),
            )
            .map(|pfn| std::mem::transmute(pfn))
            .unwrap(),
            vk_allocate_command_buffers: vk_get_device_proc_addr(
                device_handle,
                "vkAllocateCommandBuffers\0".as_ptr().cast(),
            )
            .map(|pfn| std::mem::transmute(pfn))
            .unwrap(),
            vk_create_semaphore: vk_get_device_proc_addr(
                device_handle,
                "vkCreateSemaphore\0".as_ptr().cast(),
            )
            .map(|pfn| std::mem::transmute(pfn))
            .unwrap(),
            vk_destroy_semaphore: vk_get_device_proc_addr(
                device_handle,
                "vkDestroySemaphore\0".as_ptr().cast(),
            )
            .map(|pfn| std::mem::transmute(pfn))
            .unwrap(),
            vk_create_swapchain_khr: vk_get_device_proc_addr(
                device_handle,
                "vkCreateSwapchainKHR\0".as_ptr().cast(),
            )
            .map(|pfn| std::mem::transmute(pfn)),
            vk_destroy_swapchain_khr: vk_get_device_proc_addr(
                device_handle,
                "vkDestroySwapchainKHR\0".as_ptr().cast(),
            )
            .map(|pfn| std::mem::transmute(pfn)),
            vk_get_swapchain_images_khr: vk_get_device_proc_addr(
                device_handle,
                "vkGetSwapchainImagesKHR\0".as_ptr().cast(),
            )
            .map(|pfn| std::mem::transmute(pfn)),
            vk_create_image_view: vk_get_device_proc_addr(
                device_handle,
                "vkCreateImageView\0".as_ptr().cast(),
            )
            .map(|pfn| std::mem::transmute(pfn))
            .unwrap(),
            vk_destroy_image_view: vk_get_device_proc_addr(
                device_handle,
                "vkDestroyImageView\0".as_ptr().cast(),
            )
            .map(|pfn| std::mem::transmute(pfn))
            .unwrap(),
        }
    }
}
