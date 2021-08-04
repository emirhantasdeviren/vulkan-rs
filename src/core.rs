use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

#[cfg(not(target_pointer_width = "64"))]
use std::num::NonZeroU64;

use crate::ffi;
use crate::linker::DynamicLibrary;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub const KHR_SURFACE_EXTENSION_NAME: &str = "VK_KHR_surface";
pub const KHR_SWAPCHAIN_EXTENSION_NAME: &str = "VK_KHR_swapchain";
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub const KHR_XCB_SURFACE_EXTENSION_NAME: &str = "VK_KHR_xcb_surface";
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub const KHR_XLIB_SURFACE_EXTENSION_NAME: &str = "VK_KHR_xlib_surface";
#[cfg(target_os = "windows")]
pub const KHR_WIN32_SURFACE_EXTENSION_NAME: &str = "VK_KHR_win32_surface";

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

pub struct Semaphore<'a> {
    #[cfg(target_pointer_width = "64")]
    handle: NonNull<ffi::OpaqueSemaphore>,
    #[cfg(not(target_pointer_width = "64"))]
    handle: NonZeroU64,
    device: &'a Device<'a>,
    #[cfg(target_pointer_width = "64")]
    _marker: PhantomData<ffi::OpaqueSemaphore>,
}

pub struct CommandBuffer<'a> {
    handle: NonNull<ffi::OpaqueCommandBuffer>,
    _marker: PhantomData<(ffi::OpaqueCommandBuffer, &'a CommandPool<'a>)>,
}

pub struct CommandPool<'a> {
    #[cfg(target_pointer_width = "64")]
    handle: NonNull<ffi::OpaqueCommandPool>,
    #[cfg(not(target_pointer_width = "64"))]
    handle: NonZeroU64,
    device: &'a Device<'a>,
    #[cfg(target_pointer_width = "64")]
    _marker: PhantomData<ffi::OpaqueCommandPool>,
}

pub struct SurfaceKhr<'a> {
    #[cfg(target_pointer_width = "64")]
    handle: NonNull<ffi::OpaqueSurfaceKhr>,
    #[cfg(not(target_pointer_width = "64"))]
    handle: NonZeroU64,
    instance: &'a Instance,
    #[cfg(target_pointer_width = "64")]
    _marker: PhantomData<ffi::OpaqueSurfaceKhr>,
}

pub struct SwapchainKhr<'a> {
    #[cfg(target_pointer_width = "64")]
    handle: NonNull<ffi::OpaqueSwapchainKhr>,
    #[cfg(not(target_pointer_width = "64"))]
    handle: NonZeroU64,
    device: &'a Device<'a>,
    #[cfg(target_pointer_width = "64")]
    _marker: PhantomData<ffi::OpaqueSwapchainKhr>,
}

#[derive(Default)]
struct DispatchLoaderInstance {
    vk_get_instance_proc_addr: Option<ffi::PFN_vkGetInstanceProcAddr>,
    vk_enumerate_instance_version: Option<ffi::PFN_vkEnumerateInstanceVersion>,
    vk_create_instance: Option<ffi::PFN_vkCreateInstance>,
    vk_destroy_instance: Option<ffi::PFN_vkDestroyInstance>,
    vk_enumerate_physical_devices: Option<ffi::PFN_vkEnumeratePhysicalDevices>,
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    vk_create_xcb_surface_khr: Option<ffi::PFN_vkCreateXcbSurfaceKHR>,
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    vk_create_xlib_surface_khr: Option<ffi::PFN_vkCreateXlibSurfaceKHR>,
    #[cfg(target_os = "windows")]
    vk_create_win32_surface_khr: Option<ffi::PFN_vkCreateWin32SurfaceKHR>,
    vk_destroy_surface_khr: Option<ffi::PFN_vkDestroySurfaceKHR>,
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
    vk_create_command_pool: ffi::PFN_vkCreateCommandPool,
    vk_destroy_command_pool: ffi::PFN_vkDestroyCommandPool,
    vk_allocate_command_buffers: ffi::PFN_vkAllocateCommandBuffers,
    vk_create_semaphore: ffi::PFN_vkCreateSemaphore,
    vk_destroy_semaphore: ffi::PFN_vkDestroySemaphore,
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

pub enum Format {
    Undefined,
    R4g4UnormPack8,
    R4g4b4a4UnormPack16,
    B4g4r4a4UnormPack16,
    R5g6b5UnormPack16,
    B5g6r5UnormPack16,
    R5g5b5a1UnormPack16,
    B5g5r5a1UnormPack16,
    A1r5g5b5UnormPack16,
    R8Unorm,
    R8Snorm,
    R8Uscaled,
    R8Sscaled,
    R8Uint,
    R8Sint,
    R8Srgb,
    R8g8Unorm,
    R8g8Snorm,
    R8g8Uscaled,
    R8g8Sscaled,
    R8g8Uint,
    R8g8Sint,
    R8g8Srgb,
    R8g8b8Unorm,
    R8g8b8Snorm,
    R8g8b8Uscaled,
    R8g8b8Sscaled,
    R8g8b8Uint,
    R8g8b8Sint,
    R8g8b8Srgb,
    B8g8r8Unorm,
    B8g8r8Snorm,
    B8g8r8Uscaled,
    B8g8r8Sscaled,
    B8g8r8Uint,
    B8g8r8Sint,
    B8g8r8Srgb,
    R8g8b8a8Unorm,
    R8g8b8a8Snorm,
    R8g8b8a8Uscaled,
    R8g8b8a8Sscaled,
    R8g8b8a8Uint,
    R8g8b8a8Sint,
    R8g8b8a8Srgb,
    B8g8r8a8Unorm,
    B8g8r8a8Snorm,
    B8g8r8a8Uscaled,
    B8g8r8a8Sscaled,
    B8g8r8a8Uint,
    B8g8r8a8Sint,
    B8g8r8a8Srgb,
    A8b8g8r8UnormPack32,
    A8b8g8r8SnormPack32,
    A8b8g8r8UscaledPack32,
    A8b8g8r8SscaledPack32,
    A8b8g8r8UintPack32,
    A8b8g8r8SintPack32,
    A8b8g8r8SrgbPack32,
    A2r10g10b10UnormPack32,
    A2r10g10b10SnormPack32,
    A2r10g10b10UscaledPack32,
    A2r10g10b10SscaledPack32,
    A2r10g10b10UintPack32,
    A2r10g10b10SintPack32,
    A2b10g10r10UnormPack32,
    A2b10g10r10SnormPack32,
    A2b10g10r10UscaledPack32,
    A2b10g10r10SscaledPack32,
    A2b10g10r10UintPack32,
    A2b10g10r10SintPack32,
    R16Unorm,
    R16Snorm,
    R16Uscaled,
    R16Sscaled,
    R16Uint,
    R16Sint,
    R16Sfloat,
    R16g16Unorm,
    R16g16Snorm,
    R16g16Uscaled,
    R16g16Sscaled,
    R16g16Uint,
    R16g16Sint,
    R16g16Sfloat,
    R16g16b16Unorm,
    R16g16b16Snorm,
    R16g16b16Uscaled,
    R16g16b16Sscaled,
    R16g16b16Uint,
    R16g16b16Sint,
    R16g16b16Sfloat,
    R16g16b16a16Unorm,
    R16g16b16a16Snorm,
    R16g16b16a16Uscaled,
    R16g16b16a16Sscaled,
    R16g16b16a16Uint,
    R16g16b16a16Sint,
    R16g16b16a16Sfloat,
    R32Uint,
    R32Sint,
    R32Sfloat,
    R32g32Uint,
    R32g32Sint,
    R32g32Sfloat,
    R32g32b32Uint,
    R32g32b32Sint,
    R32g32b32Sfloat,
    R32g32b32a32Uint,
    R32g32b32a32Sint,
    R32g32b32a32Sfloat,
    R64Uint,
    R64Sint,
    R64Sfloat,
    R64g64Uint,
    R64g64Sint,
    R64g64Sfloat,
    R64g64b64Uint,
    R64g64b64Sint,
    R64g64b64Sfloat,
    R64g64b64a64Uint,
    R64g64b64a64Sint,
    R64g64b64a64Sfloat,
    B10g11r11UfloatPack32,
    E5b9g9r9UfloatPack32,
    D16Unorm,
    X8D24UnormPack32,
    D32Sfloat,
    S8Uint,
    D16UnormS8Uint,
    D24UnormS8Uint,
    D32SfloatS8Uint,
    Bc1RgbUnormBlock,
    Bc1RgbSrgbBlock,
    Bc1RgbaUnormBlock,
    Bc1RgbaSrgbBlock,
    Bc2UnormBlock,
    Bc2SrgbBlock,
    Bc3UnormBlock,
    Bc3SrgbBlock,
    Bc4UnormBlock,
    Bc4SnormBlock,
    Bc5UnormBlock,
    Bc5SnormBlock,
    Bc6hUfloatBlock,
    Bc6hSfloatBlock,
    Bc7UnormBlock,
    Bc7SrgbBlock,
    Etc2R8g8b8UnormBlock,
    Etc2R8g8b8SrgbBlock,
    Etc2R8g8b8a1UnormBlock,
    Etc2R8g8b8a1SrgbBlock,
    Etc2R8g8b8a8UnormBlock,
    Etc2R8g8b8a8SrgbBlock,
    EacR11UnormBlock,
    EacR11SnormBlock,
    EacR11g11UnormBlock,
    EacR11g11SnormBlock,
    Astc4x4UnormBlock,
    Astc4x4SrgbBlock,
    Astc5x4UnormBlock,
    Astc5x4SrgbBlock,
    Astc5x5UnormBlock,
    Astc5x5SrgbBlock,
    Astc6x5UnormBlock,
    Astc6x5SrgbBlock,
    Astc6x6UnormBlock,
    Astc6x6SrgbBlock,
    Astc8x5UnormBlock,
    Astc8x5SrgbBlock,
    Astc8x6UnormBlock,
    Astc8x6SrgbBlock,
    Astc8x8UnormBlock,
    Astc8x8SrgbBlock,
    Astc10x5UnormBlock,
    Astc10x5SrgbBlock,
    Astc10x6UnormBlock,
    Astc10x6SrgbBlock,
    Astc10x8UnormBlock,
    Astc10x8SrgbBlock,
    Astc10x10UnormBlock,
    Astc10x10SrgbBlock,
    Astc12x10UnormBlock,
    Astc12x10SrgbBlock,
    Astc12x12UnormBlock,
    Astc12x12SrgbBlock,
    G8b8g8r8_422Unorm,
    B8g8r8g8_422Unorm,
    G8B8R8_3plane420Unorm,
    G8B8r8_2plane420Unorm,
    G8B8R8_3plane422Unorm,
    G8B8r8_2plane422Unorm,
    G8B8R8_3plane444Unorm,
    R10x6UnormPack16,
    R10x6g10x6Unorm2pack16,
    R10x6g10x6b10x6a10x6Unorm4pack16,
    G10x6b10x6g10x6r10x6_422Unorm4pack16,
    B10x6g10x6r10x6g10x6_422Unorm4pack16,
    G10x6B10x6R10x6_3plane420Unorm3pack16,
    G10x6B10x6r10x6_2plane420Unorm3pack16,
    G10x6B10x6R10x6_3plane422Unorm3pack16,
    G10x6B10x6r10x6_2plane422Unorm3pack16,
    G10x6B10x6R10x6_3plane444Unorm3pack16,
    R12x4UnormPack16,
    R12x4g12x4Unorm2pack16,
    R12x4g12x4b12x4a12x4Unorm4pack16,
    G12x4b12x4g12x4r12x4_422Unorm4pack16,
    B12x4g12x4r12x4g12x4_422Unorm4pack16,
    G12x4B12x4R12x4_3plane420Unorm3pack16,
    G12x4B12x4r12x4_2plane420Unorm3pack16,
    G12x4B12x4R12x4_3plane422Unorm3pack16,
    G12x4B12x4r12x4_2plane422Unorm3pack16,
    G12x4B12x4R12x4_3plane444Unorm3pack16,
    G16b16g16r16_422Unorm,
    B16g16r16g16_422Unorm,
    G16B16R16_3plane420Unorm,
    G16B16r16_2plane420Unorm,
    G16B16R16_3plane422Unorm,
    G16B16r16_2plane422Unorm,
    G16B16R16_3plane444Unorm,
    Pvrtc1_2bppUnormBlockImg,
    Pvrtc1_4bppUnormBlockImg,
    Pvrtc2_2bppUnormBlockImg,
    Pvrtc2_4bppUnormBlockImg,
    Pvrtc1_2bppSrgbBlockImg,
    Pvrtc1_4bppSrgbBlockImg,
    Pvrtc2_2bppSrgbBlockImg,
    Pvrtc2_4bppSrgbBlockImg,
    Astc4x4SfloatBlockExt,
    Astc5x4SfloatBlockExt,
    Astc5x5SfloatBlockExt,
    Astc6x5SfloatBlockExt,
    Astc6x6SfloatBlockExt,
    Astc8x5SfloatBlockExt,
    Astc8x6SfloatBlockExt,
    Astc8x8SfloatBlockExt,
    Astc10x5SfloatBlockExt,
    Astc10x6SfloatBlockExt,
    Astc10x8SfloatBlockExt,
    Astc10x10SfloatBlockExt,
    Astc12x10SfloatBlockExt,
    Astc12x12SfloatBlockExt,
    G8B8r8_2plane444UnormExt,
    G10x6B10x6r10x6_2plane444Unorm3pack16Ext,
    G12x4B12x4r12x4_2plane444Unorm3pack16Ext,
    G16B16r16_2plane444UnormExt,
    A4r4g4b4UnormPack16Ext,
    A4b4g4r4UnormPack16Ext,
}

#[derive(Default, Clone, Copy)]
pub struct ApiVersion(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    OutOfHostMemory,
    OutOfDeviceMemory,
    InitializationFailed,
    DeviceLost,
    MemoryMapFailed,
    LayerNotPresent,
    ExtensionNotPresent,
    FeatureNotPresent,
    IncompatibleDriver,
    TooManyObjects,
    FormatNotSupported,
    FragmentedPool,
    Unknown,
    OutOfPoolMemory,
    InvalidExternalHandle,
    Fragmentation,
    InvalidOpaqueCaptureAddress,
    SurfaceLostKhr,
    NativeWindowInUseKhr,
    OutOfDateKhr,
    IncompatibleDisplayKhr,
    ValidationFailedExt,
    InvalidShaderNv,
    InvalidDrmFormatModifierPlaneLayoutExt,
    NotPermittedExt,
    FullScreenExclusiveModeLostExt,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vulkan run-time error")
    }
}

impl std::error::Error for Error {}

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
        // We don't have to check that the string contains valid UTF-8.
        let device_name_cstr = unsafe { CStr::from_ptr(props.device_name.as_ptr()) };
        let device_name =
            unsafe { String::from_utf8_unchecked(device_name_cstr.to_bytes().to_vec()) };

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

    pub fn create_command_pool(&self, queue_family_index: usize) -> CommandPool<'_> {
        let create_info = ffi::CommandPoolCreateInfo {
            s_type: ffi::StructureType::CommandPoolCreateInfo,
            p_next: std::ptr::null(),
            flags: 0,
            queue_family_index: queue_family_index as u32,
        };

        let mut handle = MaybeUninit::uninit();
        let result = unsafe {
            (self.dispatch_loader.vk_create_command_pool)(
                self.handle.as_ptr(),
                &create_info,
                std::ptr::null(),
                handle.as_mut_ptr(),
            )
        };

        if result == ffi::Result::Success {
            let handle = unsafe { handle.assume_init() };
            CommandPool {
                #[cfg(target_pointer_width = "64")]
                handle: unsafe { NonNull::new_unchecked(handle) },
                #[cfg(not(target_pointer_width = "64"))]
                handle: unsafe { NonZeroU64::new_unchecked(handle) },
                device: self,
                #[cfg(target_pointer_width = "64")]
                _marker: PhantomData,
            }
        } else {
            panic!("Could not create VkCommandPool: {:?}", result);
        }
    }

    pub fn create_semaphore(&self) -> Semaphore<'_> {
        let create_info = ffi::SemaphoreCreateInfo {
            s_type: ffi::StructureType::SemaphoreCreateInfo,
            p_next: std::ptr::null(),
            flags: 0,
        };

        let mut handle = MaybeUninit::uninit();
        let result = unsafe {
            (self.dispatch_loader.vk_create_semaphore)(
                self.handle.as_ptr(),
                &create_info,
                std::ptr::null(),
                handle.as_mut_ptr(),
            )
        };

        if result == ffi::Result::Success {
            Semaphore {
                #[cfg(target_pointer_width = "64")]
                handle: unsafe { NonNull::new_unchecked(handle.assume_init()) },
                #[cfg(not(target_pointer_width = "64"))]
                handle: unsafe { NonZeroU64::new_unchecked(handle.assume_init()) },
                device: self,
                #[cfg(target_pointer_width = "64")]
                _marker: PhantomData,
            }
        } else {
            panic!("Could not create Semaphore: {:?}", result)
        }
    }
}

impl<'a> Drop for Device<'a> {
    fn drop(&mut self) {
        println!("Dropped Device");
        unsafe { (self.dispatch_loader.vk_destroy_device)(self.handle.as_ptr(), std::ptr::null()) }
    }
}

impl<'a> CommandPool<'a> {
    pub fn allocate_command_buffers(&self, buffer_count: usize) -> Vec<CommandBuffer<'_>> {
        let create_info = ffi::CommandBufferAllocateInfo {
            s_type: ffi::StructureType::CommandBufferAllocateInfo,
            p_next: std::ptr::null(),
            #[cfg(target_pointer_width = "64")]
            command_pool: self.handle.as_ptr(),
            #[cfg(not(target_pointer_width = "64"))]
            command_pool: self.handle.get(),
            level: ffi::CommandBufferLevel::Primary,
            command_buffer_count: buffer_count as u32,
        };

        let mut command_buffers = Vec::with_capacity(buffer_count);

        let result = unsafe {
            (self.device.dispatch_loader.vk_allocate_command_buffers)(
                self.device.handle.as_ptr(),
                &create_info,
                command_buffers.as_mut_ptr(),
            )
        };

        if result == ffi::Result::Success {
            unsafe { command_buffers.set_len(buffer_count) };
            command_buffers
                .into_iter()
                .map(|buffer| CommandBuffer {
                    handle: unsafe { NonNull::new_unchecked(buffer) },
                    _marker: PhantomData,
                })
                .collect()
        } else {
            panic!("Could not create CommandBuffer")
        }
    }
}

impl<'a> Drop for CommandPool<'a> {
    fn drop(&mut self) {
        println!("Dropped CommandPool");
        unsafe {
            (self.device.dispatch_loader.vk_destroy_command_pool)(
                self.device.handle.as_ptr(),
                #[cfg(target_pointer_width = "64")]
                self.handle.as_ptr(),
                #[cfg(not(target_pointer_width = "64"))]
                self.handle.get(),
                std::ptr::null(),
            );
        }
    }
}

impl<'a> Drop for SurfaceKhr<'a> {
    fn drop(&mut self) {
        println!("Dropped SurfaceKHR");
        unsafe {
            (self
                .instance
                .dispatch_loader
                .vk_destroy_surface_khr
                .unwrap())(
                self.instance.handle.as_ptr(),
                #[cfg(target_pointer_width = "64")]
                self.handle.as_ptr(),
                #[cfg(not(target_pointer_width = "64"))]
                self.handle.get(),
                std::ptr::null(),
            );
        }
    }
}

impl<'a> Drop for Semaphore<'a> {
    fn drop(&mut self) {
        println!("Dropped Semaphore");
        unsafe {
            (self.device.dispatch_loader.vk_destroy_semaphore)(
                self.device.handle.as_ptr(),
                #[cfg(target_pointer_width = "64")]
                self.handle.as_ptr(),
                #[cfg(not(target_pointer_width = "64"))]
                self.handle.get(),
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
