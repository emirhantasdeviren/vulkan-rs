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

pub type Result<T> = std::result::Result<T, Error>;

pub struct Instance {
    handle: NonNull<ffi::OpaqueInstance>,
    dispatch_loader: DispatchLoaderInstance,
    _lib: DynamicLibrary,
    _marker: PhantomData<ffi::OpaqueInstance>,
}

#[derive(Default)]
pub struct InstanceBuilder<'a> {
    application_info: Option<&'a ApplicationInfo>,
    layers: Option<&'a [&'a str]>,
    extensions: Option<&'a [&'a str]>,
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

pub struct SwapchainBuilderKhr<'a, 'b> {
    flags: SwapchainCreateFlagsKhr,
    surface: &'a SurfaceKhr<'a>,
    min_image_count: u32,
    image_format: Format,
    image_color_space: ColorSpaceKhr,
    image_extent: Extent2D,
    image_array_layers: u32,
    image_usage: ImageUsageFlags,
    image_sharing_mode: SharingMode<'b>,
    pre_transform: SurfaceTransformKhr,
    composite_alpha: CompositeAlphaKhr,
    present_mode: PresentModeKhr,
    clipped: bool,
    _old_swapchain: Option<()>,
}

pub struct Image<'a> {
    #[cfg(target_pointer_width = "64")]
    handle: NonNull<ffi::OpaqueImage>,
    #[cfg(not(target_pointer_width = "64"))]
    handle: NonZeroU64,
    device: &'a Device<'a>,
    #[cfg(target_pointer_width = "64")]
    _marker: PhantomData<ffi::OpaqueImage>,
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
    vk_get_physical_device_surface_capabilities_khr:
        Option<ffi::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>,
    vk_get_physical_device_surface_formats_khr:
        Option<ffi::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>,
    vk_get_physical_device_surface_present_modes_khr:
        Option<ffi::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>,
}

struct DispatchLoaderDevice {
    vk_destroy_device: ffi::PFN_vkDestroyDevice,
    vk_get_device_queue: ffi::PFN_vkGetDeviceQueue,
    vk_create_command_pool: ffi::PFN_vkCreateCommandPool,
    vk_destroy_command_pool: ffi::PFN_vkDestroyCommandPool,
    vk_allocate_command_buffers: ffi::PFN_vkAllocateCommandBuffers,
    vk_create_semaphore: ffi::PFN_vkCreateSemaphore,
    vk_destroy_semaphore: ffi::PFN_vkDestroySemaphore,
    vk_create_swapchain_khr: Option<ffi::PFN_vkCreateSwapchainKHR>,
    vk_destroy_swapchain_khr: Option<ffi::PFN_vkDestroySwapchainKHR>,
    vk_get_swapchain_images_khr: Option<ffi::PFN_vkGetSwapchainImagesKHR>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpaceKhr {
    SrgbNonlinearKhr,
    DisplayP3NonlinearExt,
    ExtendedSrgbLinearExt,
    DisplayP3LinearExt,
    DciP3NonlinearExt,
    Bt709LinearExt,
    Bt709NonlinearExt,
    Bt2020LinearExt,
    Hdr10St2084Ext,
    DolbyvisionExt,
    Hdr10HlgExt,
    AdobergbLinearExt,
    AdobergbNonlinearExt,
    PassThroughExt,
    ExtendedSrgbNonlinearExt,
    DisplayNativeAmd,
}

pub enum SharingMode<'a> {
    Exclusive,
    Concurrent(&'a [usize]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceTransformKhr {
    IdentityKhr,
    Rotate90Khr,
    Rotate180Khr,
    Rotate270Khr,
    HorizontalMirrorKhr,
    HorizontalMirrorRotate90Khr,
    HorizontalMirrorRotate180Khr,
    HorizontalMirrorRotate270Khr,
    InheritKhr,
}

pub struct SurfaceTransformFlagsKhr(u32);

pub enum CompositeAlphaKhr {
    OpaqueKhr,
    PreMultipliedKhr,
    PostMultipliedKhr,
    InheritKhr,
}

pub struct CompositeAlphaFlagsKhr(u32);

pub struct SurfaceCapabilitiesKhr {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKhr,
    pub current_transform: SurfaceTransformKhr,
    pub supported_composite_alpha: CompositeAlphaFlagsKhr,
    pub supported_usage_flags: ImageUsageFlags,
}

#[derive(Debug, Clone, Copy)]
pub struct SurfaceFormatKhr {
    pub format: Format,
    pub color_space: ColorSpaceKhr,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Extent2D {
    width: u32,
    height: u32,
}

pub enum ImageUsage {
    TransferSrc,
    TransferDst,
    Sampled,
    Storage,
    ColorAttachment,
    DepthStencilAttachment,
    TransientAttachment,
    InputAttachment,
    ShadingRateImageNv,
    FragmentDensityMapExt,
}
#[derive(Debug, Default)]
pub struct ImageUsageFlags(u32);
#[derive(Default)]
pub struct ImageUsageFlagsBuilder(u32);

pub enum SwapchainCreateKhr {
    SplitInstanceBindRegionsKhr,
    ProtectedKhr,
    MutableFormatKhr,
}
#[derive(Debug, Default)]
pub struct SwapchainCreateFlagsKhr(u32);
#[derive(Default)]
pub struct SwapchainCreateFlagsBuilderKhr(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentModeKhr {
    ImmediateKhr,
    MailboxKhr,
    FifoKhr,
    FifoRelaxedKhr,
    SharedDemandRefreshKhr,
    SharedContinuousRefreshKhr,
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
    pub fn new() -> Result<Self> {
        InstanceBuilder::new().build()
    }

    pub fn builder<'a>() -> InstanceBuilder<'a> {
        Default::default()
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
        extensions: Option<&[&str]>,
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

        let extensions_c: Option<Vec<CString>> = extensions.map(|e| {
            e.iter()
                .map(|name| CString::new(name.as_bytes()).unwrap())
                .collect()
        });
        let extension_ptrs: Option<Vec<*const i8>> = extensions_c
            .as_ref()
            .map(|e| e.iter().map(|name| name.as_ptr()).collect());

        let enabled_extension_count = extension_ptrs.as_ref().map_or(0, |ptrs| ptrs.len() as u32);
        let pp_enabled_extension_names = extension_ptrs
            .as_ref()
            .map_or(std::ptr::null(), |ptrs| ptrs.as_ptr());

        let create_info = ffi::DeviceCreateInfo {
            s_type: ffi::StructureType::DeviceCreateInfo,
            p_next: std::ptr::null(),
            flags: 0,
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: std::ptr::null(),
            enabled_extension_count,
            pp_enabled_extension_names,
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

    pub fn get_surface_formats_khr(
        &self,
        surface: &SurfaceKhr,
    ) -> Option<Result<Vec<SurfaceFormatKhr>>> {
        let mut capacity = 0;
        let vk_get_physical_device_surface_formats_khr = self
            .dispatch_loader
            .vk_get_physical_device_surface_formats_khr?;

        let result = unsafe {
            (vk_get_physical_device_surface_formats_khr)(
                self.handle.as_ptr(),
                #[cfg(target_pointer_width = "64")]
                surface.handle.as_ptr(),
                #[cfg(not(target_pointer_width = "64"))]
                surface.handle.get(),
                &mut capacity,
                std::ptr::null_mut(),
            )
        };

        Some(match result {
            ffi::Result::Success => {
                let mut surface_formats = Vec::with_capacity(capacity as usize);

                let result = unsafe {
                    vk_get_physical_device_surface_formats_khr(
                        self.handle.as_ptr(),
                        #[cfg(target_pointer_width = "64")]
                        surface.handle.as_ptr(),
                        #[cfg(not(target_pointer_width = "64"))]
                        surface.handle.get(),
                        &mut capacity,
                        surface_formats.as_mut_ptr(),
                    )
                };

                match result {
                    ffi::Result::Success => {
                        unsafe { surface_formats.set_len(capacity as usize) };
                        Ok(surface_formats
                            .into_iter()
                            .map(|sf| SurfaceFormatKhr {
                                format: sf.format.into(),
                                color_space: sf.color_space.into(),
                            })
                            .collect())
                    }
                    ffi::Result::Incomplete => todo!(),
                    ffi::Result::ErrorOutOfHostMemory => Err(Error::OutOfHostMemory),
                    ffi::Result::ErrorOutOfDeviceMemory => Err(Error::OutOfDeviceMemory),
                    ffi::Result::ErrorSurfaceLostKhr => Err(Error::SurfaceLostKhr),
                    _ => unreachable!(),
                }
            }
            ffi::Result::Incomplete => todo!(),
            ffi::Result::ErrorOutOfHostMemory => Err(Error::OutOfHostMemory),
            ffi::Result::ErrorOutOfDeviceMemory => Err(Error::OutOfDeviceMemory),
            ffi::Result::ErrorSurfaceLostKhr => Err(Error::SurfaceLostKhr),
            _ => unreachable!(),
        })
    }

    pub fn get_surface_capabilities_khr(
        &self,
        surface: &SurfaceKhr,
    ) -> Option<Result<SurfaceCapabilitiesKhr>> {
        let mut surface_capabilities = MaybeUninit::uninit();
        let result = unsafe {
            (self
                .dispatch_loader
                .vk_get_physical_device_surface_capabilities_khr?)(
                #[cfg(target_pointer_width = "64")]
                self.handle.as_ptr(),
                #[cfg(not(target_pointer_width = "64"))]
                self.handle.get(),
                surface.handle.as_ptr(),
                surface_capabilities.as_mut_ptr(),
            )
        };

        match result {
            ffi::Result::Success => {
                let ffi::SurfaceCapabilitiesKhr {
                    min_image_count,
                    max_image_count,
                    current_extent,
                    min_image_extent,
                    max_image_extent,
                    max_image_array_layers,
                    supported_transforms,
                    current_transform,
                    supported_composite_alpha,
                    supported_usage_flags,
                } = unsafe { surface_capabilities.assume_init() };

                Some(Ok(SurfaceCapabilitiesKhr {
                    min_image_count,
                    max_image_count,
                    current_extent: current_extent.into(),
                    min_image_extent: min_image_extent.into(),
                    max_image_extent: max_image_extent.into(),
                    max_image_array_layers,
                    supported_transforms: SurfaceTransformFlagsKhr(supported_transforms),
                    current_transform: current_transform.into(),
                    supported_composite_alpha: CompositeAlphaFlagsKhr(supported_composite_alpha),
                    supported_usage_flags: ImageUsageFlags(supported_usage_flags),
                }))
            }
            ffi::Result::ErrorOutOfHostMemory => Some(Err(Error::OutOfHostMemory)),
            ffi::Result::ErrorOutOfDeviceMemory => Some(Err(Error::OutOfDeviceMemory)),
            ffi::Result::ErrorSurfaceLostKhr => Some(Err(Error::SurfaceLostKhr)),
            _ => unreachable!(),
        }
    }

    pub fn get_surface_present_modes_khr(
        &self,
        surface: &SurfaceKhr,
    ) -> Option<Result<Vec<PresentModeKhr>>> {
        let mut capacity = 0;
        let vk_get_physical_device_surface_present_modes_khr = self
            .dispatch_loader
            .vk_get_physical_device_surface_present_modes_khr?;
        let result = unsafe {
            vk_get_physical_device_surface_present_modes_khr(
                self.handle.as_ptr(),
                #[cfg(target_pointer_width = "64")]
                surface.handle.as_ptr(),
                #[cfg(not(target_pointer_width = "64"))]
                surface.handle.get(),
                &mut capacity,
                std::ptr::null_mut(),
            )
        };

        Some(match result {
            ffi::Result::Success => {
                let mut present_modes: Vec<ffi::PresentModeKhr> =
                    Vec::with_capacity(capacity as usize);
                let result = unsafe {
                    vk_get_physical_device_surface_present_modes_khr(
                        self.handle.as_ptr(),
                        #[cfg(target_pointer_width = "64")]
                        surface.handle.as_ptr(),
                        #[cfg(not(target_pointer_width = "64"))]
                        surface.handle.get(),
                        &mut capacity,
                        present_modes.as_mut_ptr(),
                    )
                };

                match result {
                    ffi::Result::Success => {
                        unsafe { present_modes.set_len(capacity as usize) };

                        Ok(present_modes.into_iter().map(|pm| pm.into()).collect())
                    }
                    ffi::Result::Incomplete => todo!(),
                    ffi::Result::ErrorOutOfHostMemory => Err(Error::OutOfHostMemory),
                    ffi::Result::ErrorOutOfDeviceMemory => Err(Error::OutOfDeviceMemory),
                    ffi::Result::ErrorSurfaceLostKhr => Err(Error::SurfaceLostKhr),
                    _ => unreachable!(),
                }
            }
            ffi::Result::Incomplete => todo!(),
            ffi::Result::ErrorOutOfHostMemory => Err(Error::OutOfHostMemory),
            ffi::Result::ErrorOutOfDeviceMemory => Err(Error::OutOfDeviceMemory),
            ffi::Result::ErrorSurfaceLostKhr => Err(Error::SurfaceLostKhr),
            _ => unreachable!(),
        })
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

    pub fn get_swapchain_images_khr<'b: 'a, 'c: 'b>(
        &'b self,
        swapchain: &'c SwapchainKhr<'b>,
    ) -> Option<Result<Vec<Image<'c>>>> {
        self.dispatch_loader.vk_get_swapchain_images_khr.map(|pfn| {
            let mut p_swapchain_image_count = MaybeUninit::uninit();
            let result = unsafe {
                pfn(
                    self.handle.as_ptr(),
                    #[cfg(target_pointer_width = "64")]
                    swapchain.handle.as_ptr(),
                    #[cfg(not(target_pointer_width = "64"))]
                    swapchain.handle.get(),
                    p_swapchain_image_count.as_mut_ptr(),
                    std::ptr::null_mut(),
                )
            };

            match result {
                ffi::Result::Success => {
                    let capacity = unsafe { p_swapchain_image_count.assume_init() as usize };
                    let mut swapchain_images = Vec::with_capacity(capacity);
                    let result = unsafe {
                        pfn(
                            self.handle.as_ptr(),
                            #[cfg(target_pointer_width = "64")]
                            swapchain.handle.as_ptr(),
                            #[cfg(not(target_pointer_width = "64"))]
                            swapchain.handle.get(),
                            p_swapchain_image_count.as_mut_ptr(),
                            swapchain_images.as_mut_ptr(),
                        )
                    };

                    match result {
                        ffi::Result::Success => {
                            let new_len = unsafe { p_swapchain_image_count.assume_init() as usize };
                            unsafe { swapchain_images.set_len(new_len) };
                            Ok(swapchain_images.into_iter().map(|image| {
                                Image {
                                    #[cfg(target_pointer_width = "64")]
                                    handle: unsafe { NonNull::new_unchecked(image) },
                                    #[cfg(not(target_pointer_width = "64"))]
                                    handle: unsafe { NonZeroU64::new_unchecked(image) },
                                    device: self,
                                    #[cfg(target_pointer_width = "64")]
                                    _marker: PhantomData,
                                }
                            })
                            .collect())
                        }
                        ffi::Result::Incomplete => todo!(),
                        ffi::Result::ErrorOutOfHostMemory => Err(Error::OutOfHostMemory),
                        ffi::Result::ErrorOutOfDeviceMemory => Err(Error::OutOfDeviceMemory),
                        _ => unreachable!(),
                    }
                }
                ffi::Result::Incomplete => todo!(),
                ffi::Result::ErrorOutOfHostMemory => Err(Error::OutOfHostMemory),
                ffi::Result::ErrorOutOfDeviceMemory => Err(Error::OutOfDeviceMemory),
                _ => unreachable!(),
            }
        })
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

impl<'a, 'b> SwapchainBuilderKhr<'a, 'b> {
    pub fn new(
        surface: &'a SurfaceKhr<'a>,
        min_image_count: u32,
        image_format: Format,
        image_color_space: ColorSpaceKhr,
        image_extent: Extent2D,
        image_usage: ImageUsageFlags,
        image_sharing_mode: SharingMode<'b>,
        pre_transform: SurfaceTransformKhr,
        composite_alpha: CompositeAlphaKhr,
        present_mode: PresentModeKhr,
        clipped: bool,
    ) -> Self {
        Self {
            flags: Default::default(),
            surface,
            min_image_count,
            image_format,
            image_color_space,
            image_extent,
            image_array_layers: 1,
            image_usage,
            image_sharing_mode,
            pre_transform,
            composite_alpha,
            present_mode,
            clipped,
            _old_swapchain: Default::default(),
        }
    }

    pub fn with_flags(mut self, flags: SwapchainCreateFlagsKhr) -> Self {
        self.flags = flags;
        self
    }

    pub fn with_image_array_layers(mut self, image_array_layers: u32) -> Self {
        self.image_array_layers = image_array_layers;
        self
    }

    pub fn build(self, device: &'a Device<'_>) -> Result<SwapchainKhr<'a>> {
        let create_info = ffi::SwapchainCreateInfoKhr {
            s_type: ffi::StructureType::SwapchainCreateInfoKhr,
            p_next: std::ptr::null(),
            flags: self.flags.0,
            #[cfg(target_pointer_width = "64")]
            surface: self.surface.handle.as_ptr(),
            #[cfg(not(target_pointer_width = "64"))]
            surface: self.surface.handle.get(),
            min_image_count: self.min_image_count,
            image_format: self.image_format.into(),
            image_color_space: self.image_color_space.into(),
            image_extent: self.image_extent.into(),
            image_array_layers: self.image_array_layers,
            image_usage: self.image_usage.0,
            image_sharing_mode: (&self.image_sharing_mode).into(),
            queue_family_index_count: match self.image_sharing_mode {
                SharingMode::Exclusive => 0,
                SharingMode::Concurrent(s) => s.len() as u32,
            },
            queue_family_indices: match self.image_sharing_mode {
                SharingMode::Exclusive => std::ptr::null(),
                SharingMode::Concurrent(s) => s.as_ptr().cast(),
            },
            pre_transform: self.pre_transform.into(),
            composite_alpha: self.composite_alpha.into(),
            present_mode: self.present_mode.into(),
            clipped: self.clipped.into(),
            old_swapchain: std::ptr::null_mut(),
        };

        let mut handle = MaybeUninit::uninit();

        let result = unsafe {
            (device.dispatch_loader.vk_create_swapchain_khr.unwrap())(
                device.handle.as_ptr(),
                &create_info,
                std::ptr::null(),
                handle.as_mut_ptr(),
            )
        };

        match result {
            ffi::Result::Success => Ok(SwapchainKhr {
                #[cfg(target_pointer_width = "64")]
                handle: unsafe { NonNull::new_unchecked(handle.assume_init()) },
                #[cfg(not(target_pointer_width = "64"))]
                handle: unsafe { NonZeroU64::new_unchecked(handle.assume_init()) },
                device,
                #[cfg(target_pointer_width = "64")]
                _marker: PhantomData,
            }),
            ffi::Result::ErrorOutOfHostMemory => Err(Error::OutOfHostMemory),
            ffi::Result::ErrorOutOfDeviceMemory => Err(Error::OutOfDeviceMemory),
            ffi::Result::ErrorDeviceLost => Err(Error::DeviceLost),
            ffi::Result::ErrorSurfaceLostKhr => Err(Error::SurfaceLostKhr),
            ffi::Result::ErrorNativeWindowInUseKhr => Err(Error::NativeWindowInUseKhr),
            ffi::Result::ErrorInitializationFailed => Err(Error::InitializationFailed),
            _ => unreachable!(),
        }
    }
}

impl<'a> Drop for SwapchainKhr<'a> {
    fn drop(&mut self) {
        println!("Dropped SwapchainKHR");
        unsafe {
            (self
                .device
                .dispatch_loader
                .vk_destroy_swapchain_khr
                .unwrap())(
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
        }
    }
}

impl QueueFamilyProperties {
    pub fn supports_graphics(&self) -> bool {
        self.queue_flags & ffi::QueueFlagBits::GraphicsBit as u32 != 0
    }

    pub fn supports_compute(&self) -> bool {
        self.queue_flags & ffi::QueueFlagBits::ComputeBit as u32 != 0
    }

    pub fn supports_transfer(&self) -> bool {
        self.queue_flags & ffi::QueueFlagBits::TransferBit as u32 != 0
    }

    pub fn supports_sparse_binding(&self) -> bool {
        self.queue_flags & ffi::QueueFlagBits::SparseBindingBit as u32 != 0
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

impl Extent2D {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl From<ffi::Extent2D> for Extent2D {
    fn from(extent: ffi::Extent2D) -> Self {
        Self {
            width: extent.width,
            height: extent.height,
        }
    }
}

impl From<Extent2D> for ffi::Extent2D {
    fn from(extent: Extent2D) -> Self {
        Self {
            width: extent.width,
            height: extent.height,
        }
    }
}

impl From<ffi::Format> for Format {
    fn from(format: ffi::Format) -> Self {
        match format {
            ffi::Format::Undefined => Self::Undefined,
            ffi::Format::R4g4UnormPack8 => Self::R4g4UnormPack8,
            ffi::Format::R4g4b4a4UnormPack16 => Self::R4g4b4a4UnormPack16,
            ffi::Format::B4g4r4a4UnormPack16 => Self::B4g4r4a4UnormPack16,
            ffi::Format::R5g6b5UnormPack16 => Self::R5g6b5UnormPack16,
            ffi::Format::B5g6r5UnormPack16 => Self::B5g6r5UnormPack16,
            ffi::Format::R5g5b5a1UnormPack16 => Self::R5g5b5a1UnormPack16,
            ffi::Format::B5g5r5a1UnormPack16 => Self::B5g5r5a1UnormPack16,
            ffi::Format::A1r5g5b5UnormPack16 => Self::A1r5g5b5UnormPack16,
            ffi::Format::R8Unorm => Self::R8Unorm,
            ffi::Format::R8Snorm => Self::R8Snorm,
            ffi::Format::R8Uscaled => Self::R8Uscaled,
            ffi::Format::R8Sscaled => Self::R8Sscaled,
            ffi::Format::R8Uint => Self::R8Uint,
            ffi::Format::R8Sint => Self::R8Sint,
            ffi::Format::R8Srgb => Self::R8Srgb,
            ffi::Format::R8g8Unorm => Self::R8g8Unorm,
            ffi::Format::R8g8Snorm => Self::R8g8Snorm,
            ffi::Format::R8g8Uscaled => Self::R8g8Uscaled,
            ffi::Format::R8g8Sscaled => Self::R8g8Sscaled,
            ffi::Format::R8g8Uint => Self::R8g8Uint,
            ffi::Format::R8g8Sint => Self::R8g8Sint,
            ffi::Format::R8g8Srgb => Self::R8g8Srgb,
            ffi::Format::R8g8b8Unorm => Self::R8g8b8Unorm,
            ffi::Format::R8g8b8Snorm => Self::R8g8b8Snorm,
            ffi::Format::R8g8b8Uscaled => Self::R8g8b8Uscaled,
            ffi::Format::R8g8b8Sscaled => Self::R8g8b8Sscaled,
            ffi::Format::R8g8b8Uint => Self::R8g8b8Uint,
            ffi::Format::R8g8b8Sint => Self::R8g8b8Sint,
            ffi::Format::R8g8b8Srgb => Self::R8g8b8Srgb,
            ffi::Format::B8g8r8Unorm => Self::B8g8r8Unorm,
            ffi::Format::B8g8r8Snorm => Self::B8g8r8Snorm,
            ffi::Format::B8g8r8Uscaled => Self::B8g8r8Uscaled,
            ffi::Format::B8g8r8Sscaled => Self::B8g8r8Sscaled,
            ffi::Format::B8g8r8Uint => Self::B8g8r8Uint,
            ffi::Format::B8g8r8Sint => Self::B8g8r8Sint,
            ffi::Format::B8g8r8Srgb => Self::B8g8r8Srgb,
            ffi::Format::R8g8b8a8Unorm => Self::R8g8b8a8Unorm,
            ffi::Format::R8g8b8a8Snorm => Self::R8g8b8a8Snorm,
            ffi::Format::R8g8b8a8Uscaled => Self::R8g8b8a8Uscaled,
            ffi::Format::R8g8b8a8Sscaled => Self::R8g8b8a8Sscaled,
            ffi::Format::R8g8b8a8Uint => Self::R8g8b8a8Uint,
            ffi::Format::R8g8b8a8Sint => Self::R8g8b8a8Sint,
            ffi::Format::R8g8b8a8Srgb => Self::R8g8b8a8Srgb,
            ffi::Format::B8g8r8a8Unorm => Self::B8g8r8a8Unorm,
            ffi::Format::B8g8r8a8Snorm => Self::B8g8r8a8Snorm,
            ffi::Format::B8g8r8a8Uscaled => Self::B8g8r8a8Uscaled,
            ffi::Format::B8g8r8a8Sscaled => Self::B8g8r8a8Sscaled,
            ffi::Format::B8g8r8a8Uint => Self::B8g8r8a8Uint,
            ffi::Format::B8g8r8a8Sint => Self::B8g8r8a8Sint,
            ffi::Format::B8g8r8a8Srgb => Self::B8g8r8a8Srgb,
            ffi::Format::A8b8g8r8UnormPack32 => Self::A8b8g8r8UnormPack32,
            ffi::Format::A8b8g8r8SnormPack32 => Self::A8b8g8r8SnormPack32,
            ffi::Format::A8b8g8r8UscaledPack32 => Self::A8b8g8r8UscaledPack32,
            ffi::Format::A8b8g8r8SscaledPack32 => Self::A8b8g8r8SscaledPack32,
            ffi::Format::A8b8g8r8UintPack32 => Self::A8b8g8r8UintPack32,
            ffi::Format::A8b8g8r8SintPack32 => Self::A8b8g8r8SintPack32,
            ffi::Format::A8b8g8r8SrgbPack32 => Self::A8b8g8r8SrgbPack32,
            ffi::Format::A2r10g10b10UnormPack32 => Self::A2r10g10b10UnormPack32,
            ffi::Format::A2r10g10b10SnormPack32 => Self::A2r10g10b10SnormPack32,
            ffi::Format::A2r10g10b10UscaledPack32 => Self::A2r10g10b10UscaledPack32,
            ffi::Format::A2r10g10b10SscaledPack32 => Self::A2r10g10b10SscaledPack32,
            ffi::Format::A2r10g10b10UintPack32 => Self::A2r10g10b10UintPack32,
            ffi::Format::A2r10g10b10SintPack32 => Self::A2r10g10b10SintPack32,
            ffi::Format::A2b10g10r10UnormPack32 => Self::A2b10g10r10UnormPack32,
            ffi::Format::A2b10g10r10SnormPack32 => Self::A2b10g10r10SnormPack32,
            ffi::Format::A2b10g10r10UscaledPack32 => Self::A2b10g10r10UscaledPack32,
            ffi::Format::A2b10g10r10SscaledPack32 => Self::A2b10g10r10SscaledPack32,
            ffi::Format::A2b10g10r10UintPack32 => Self::A2b10g10r10UintPack32,
            ffi::Format::A2b10g10r10SintPack32 => Self::A2b10g10r10SintPack32,
            ffi::Format::R16Unorm => Self::R16Unorm,
            ffi::Format::R16Snorm => Self::R16Snorm,
            ffi::Format::R16Uscaled => Self::R16Uscaled,
            ffi::Format::R16Sscaled => Self::R16Sscaled,
            ffi::Format::R16Uint => Self::R16Uint,
            ffi::Format::R16Sint => Self::R16Sint,
            ffi::Format::R16Sfloat => Self::R16Sfloat,
            ffi::Format::R16g16Unorm => Self::R16g16Unorm,
            ffi::Format::R16g16Snorm => Self::R16g16Snorm,
            ffi::Format::R16g16Uscaled => Self::R16g16Uscaled,
            ffi::Format::R16g16Sscaled => Self::R16g16Sscaled,
            ffi::Format::R16g16Uint => Self::R16g16Uint,
            ffi::Format::R16g16Sint => Self::R16g16Sint,
            ffi::Format::R16g16Sfloat => Self::R16g16Sfloat,
            ffi::Format::R16g16b16Unorm => Self::R16g16b16Unorm,
            ffi::Format::R16g16b16Snorm => Self::R16g16b16Snorm,
            ffi::Format::R16g16b16Uscaled => Self::R16g16b16Uscaled,
            ffi::Format::R16g16b16Sscaled => Self::R16g16b16Sscaled,
            ffi::Format::R16g16b16Uint => Self::R16g16b16Uint,
            ffi::Format::R16g16b16Sint => Self::R16g16b16Sint,
            ffi::Format::R16g16b16Sfloat => Self::R16g16b16Sfloat,
            ffi::Format::R16g16b16a16Unorm => Self::R16g16b16a16Unorm,
            ffi::Format::R16g16b16a16Snorm => Self::R16g16b16a16Snorm,
            ffi::Format::R16g16b16a16Uscaled => Self::R16g16b16a16Uscaled,
            ffi::Format::R16g16b16a16Sscaled => Self::R16g16b16a16Sscaled,
            ffi::Format::R16g16b16a16Uint => Self::R16g16b16a16Uint,
            ffi::Format::R16g16b16a16Sint => Self::R16g16b16a16Sint,
            ffi::Format::R16g16b16a16Sfloat => Self::R16g16b16a16Sfloat,
            ffi::Format::R32Uint => Self::R32Uint,
            ffi::Format::R32Sint => Self::R32Sint,
            ffi::Format::R32Sfloat => Self::R32Sfloat,
            ffi::Format::R32g32Uint => Self::R32g32Uint,
            ffi::Format::R32g32Sint => Self::R32g32Sint,
            ffi::Format::R32g32Sfloat => Self::R32g32Sfloat,
            ffi::Format::R32g32b32Uint => Self::R32g32b32Uint,
            ffi::Format::R32g32b32Sint => Self::R32g32b32Sint,
            ffi::Format::R32g32b32Sfloat => Self::R32g32b32Sfloat,
            ffi::Format::R32g32b32a32Uint => Self::R32g32b32a32Uint,
            ffi::Format::R32g32b32a32Sint => Self::R32g32b32a32Sint,
            ffi::Format::R32g32b32a32Sfloat => Self::R32g32b32a32Sfloat,
            ffi::Format::R64Uint => Self::R64Uint,
            ffi::Format::R64Sint => Self::R64Sint,
            ffi::Format::R64Sfloat => Self::R64Sfloat,
            ffi::Format::R64g64Uint => Self::R64g64Uint,
            ffi::Format::R64g64Sint => Self::R64g64Sint,
            ffi::Format::R64g64Sfloat => Self::R64g64Sfloat,
            ffi::Format::R64g64b64Uint => Self::R64g64b64Uint,
            ffi::Format::R64g64b64Sint => Self::R64g64b64Sint,
            ffi::Format::R64g64b64Sfloat => Self::R64g64b64Sfloat,
            ffi::Format::R64g64b64a64Uint => Self::R64g64b64a64Uint,
            ffi::Format::R64g64b64a64Sint => Self::R64g64b64a64Sint,
            ffi::Format::R64g64b64a64Sfloat => Self::R64g64b64a64Sfloat,
            ffi::Format::B10g11r11UfloatPack32 => Self::B10g11r11UfloatPack32,
            ffi::Format::E5b9g9r9UfloatPack32 => Self::E5b9g9r9UfloatPack32,
            ffi::Format::D16Unorm => Self::D16Unorm,
            ffi::Format::X8D24UnormPack32 => Self::X8D24UnormPack32,
            ffi::Format::D32Sfloat => Self::D32Sfloat,
            ffi::Format::S8Uint => Self::S8Uint,
            ffi::Format::D16UnormS8Uint => Self::D16UnormS8Uint,
            ffi::Format::D24UnormS8Uint => Self::D24UnormS8Uint,
            ffi::Format::D32SfloatS8Uint => Self::D32SfloatS8Uint,
            ffi::Format::Bc1RgbUnormBlock => Self::Bc1RgbUnormBlock,
            ffi::Format::Bc1RgbSrgbBlock => Self::Bc1RgbSrgbBlock,
            ffi::Format::Bc1RgbaUnormBlock => Self::Bc1RgbaUnormBlock,
            ffi::Format::Bc1RgbaSrgbBlock => Self::Bc1RgbaSrgbBlock,
            ffi::Format::Bc2UnormBlock => Self::Bc2UnormBlock,
            ffi::Format::Bc2SrgbBlock => Self::Bc2SrgbBlock,
            ffi::Format::Bc3UnormBlock => Self::Bc3UnormBlock,
            ffi::Format::Bc3SrgbBlock => Self::Bc3SrgbBlock,
            ffi::Format::Bc4UnormBlock => Self::Bc4UnormBlock,
            ffi::Format::Bc4SnormBlock => Self::Bc4SnormBlock,
            ffi::Format::Bc5UnormBlock => Self::Bc5UnormBlock,
            ffi::Format::Bc5SnormBlock => Self::Bc5SnormBlock,
            ffi::Format::Bc6hUfloatBlock => Self::Bc6hUfloatBlock,
            ffi::Format::Bc6hSfloatBlock => Self::Bc6hSfloatBlock,
            ffi::Format::Bc7UnormBlock => Self::Bc7UnormBlock,
            ffi::Format::Bc7SrgbBlock => Self::Bc7SrgbBlock,
            ffi::Format::Etc2R8g8b8UnormBlock => Self::Etc2R8g8b8UnormBlock,
            ffi::Format::Etc2R8g8b8SrgbBlock => Self::Etc2R8g8b8SrgbBlock,
            ffi::Format::Etc2R8g8b8a1UnormBlock => Self::Etc2R8g8b8a1UnormBlock,
            ffi::Format::Etc2R8g8b8a1SrgbBlock => Self::Etc2R8g8b8a1SrgbBlock,
            ffi::Format::Etc2R8g8b8a8UnormBlock => Self::Etc2R8g8b8a8UnormBlock,
            ffi::Format::Etc2R8g8b8a8SrgbBlock => Self::Etc2R8g8b8a8SrgbBlock,
            ffi::Format::EacR11UnormBlock => Self::EacR11UnormBlock,
            ffi::Format::EacR11SnormBlock => Self::EacR11SnormBlock,
            ffi::Format::EacR11g11UnormBlock => Self::EacR11g11UnormBlock,
            ffi::Format::EacR11g11SnormBlock => Self::EacR11g11SnormBlock,
            ffi::Format::Astc4x4UnormBlock => Self::Astc4x4UnormBlock,
            ffi::Format::Astc4x4SrgbBlock => Self::Astc4x4SrgbBlock,
            ffi::Format::Astc5x4UnormBlock => Self::Astc5x4UnormBlock,
            ffi::Format::Astc5x4SrgbBlock => Self::Astc5x4SrgbBlock,
            ffi::Format::Astc5x5UnormBlock => Self::Astc5x5UnormBlock,
            ffi::Format::Astc5x5SrgbBlock => Self::Astc5x5SrgbBlock,
            ffi::Format::Astc6x5UnormBlock => Self::Astc6x5UnormBlock,
            ffi::Format::Astc6x5SrgbBlock => Self::Astc6x5SrgbBlock,
            ffi::Format::Astc6x6UnormBlock => Self::Astc6x6UnormBlock,
            ffi::Format::Astc6x6SrgbBlock => Self::Astc6x6SrgbBlock,
            ffi::Format::Astc8x5UnormBlock => Self::Astc8x5UnormBlock,
            ffi::Format::Astc8x5SrgbBlock => Self::Astc8x5SrgbBlock,
            ffi::Format::Astc8x6UnormBlock => Self::Astc8x6UnormBlock,
            ffi::Format::Astc8x6SrgbBlock => Self::Astc8x6SrgbBlock,
            ffi::Format::Astc8x8UnormBlock => Self::Astc8x8UnormBlock,
            ffi::Format::Astc8x8SrgbBlock => Self::Astc8x8SrgbBlock,
            ffi::Format::Astc10x5UnormBlock => Self::Astc10x5UnormBlock,
            ffi::Format::Astc10x5SrgbBlock => Self::Astc10x5SrgbBlock,
            ffi::Format::Astc10x6UnormBlock => Self::Astc10x6UnormBlock,
            ffi::Format::Astc10x6SrgbBlock => Self::Astc10x6SrgbBlock,
            ffi::Format::Astc10x8UnormBlock => Self::Astc10x8UnormBlock,
            ffi::Format::Astc10x8SrgbBlock => Self::Astc10x8SrgbBlock,
            ffi::Format::Astc10x10UnormBlock => Self::Astc10x10UnormBlock,
            ffi::Format::Astc10x10SrgbBlock => Self::Astc10x10SrgbBlock,
            ffi::Format::Astc12x10UnormBlock => Self::Astc12x10UnormBlock,
            ffi::Format::Astc12x10SrgbBlock => Self::Astc12x10SrgbBlock,
            ffi::Format::Astc12x12UnormBlock => Self::Astc12x12UnormBlock,
            ffi::Format::Astc12x12SrgbBlock => Self::Astc12x12SrgbBlock,
            ffi::Format::G8b8g8r8_422Unorm => Self::G8b8g8r8_422Unorm,
            ffi::Format::B8g8r8g8_422Unorm => Self::B8g8r8g8_422Unorm,
            ffi::Format::G8B8R8_3plane420Unorm => Self::G8B8R8_3plane420Unorm,
            ffi::Format::G8B8r8_2plane420Unorm => Self::G8B8r8_2plane420Unorm,
            ffi::Format::G8B8R8_3plane422Unorm => Self::G8B8R8_3plane422Unorm,
            ffi::Format::G8B8r8_2plane422Unorm => Self::G8B8r8_2plane422Unorm,
            ffi::Format::G8B8R8_3plane444Unorm => Self::G8B8R8_3plane444Unorm,
            ffi::Format::R10x6UnormPack16 => Self::R10x6UnormPack16,
            ffi::Format::R10x6g10x6Unorm2pack16 => Self::R10x6g10x6Unorm2pack16,
            ffi::Format::R10x6g10x6b10x6a10x6Unorm4pack16 => Self::R10x6g10x6b10x6a10x6Unorm4pack16,
            ffi::Format::G10x6b10x6g10x6r10x6_422Unorm4pack16 => {
                Self::G10x6b10x6g10x6r10x6_422Unorm4pack16
            }
            ffi::Format::B10x6g10x6r10x6g10x6_422Unorm4pack16 => {
                Self::B10x6g10x6r10x6g10x6_422Unorm4pack16
            }
            ffi::Format::G10x6B10x6R10x6_3plane420Unorm3pack16 => {
                Self::G10x6B10x6R10x6_3plane420Unorm3pack16
            }
            ffi::Format::G10x6B10x6r10x6_2plane420Unorm3pack16 => {
                Self::G10x6B10x6r10x6_2plane420Unorm3pack16
            }
            ffi::Format::G10x6B10x6R10x6_3plane422Unorm3pack16 => {
                Self::G10x6B10x6R10x6_3plane422Unorm3pack16
            }
            ffi::Format::G10x6B10x6r10x6_2plane422Unorm3pack16 => {
                Self::G10x6B10x6r10x6_2plane422Unorm3pack16
            }
            ffi::Format::G10x6B10x6R10x6_3plane444Unorm3pack16 => {
                Self::G10x6B10x6R10x6_3plane444Unorm3pack16
            }
            ffi::Format::R12x4UnormPack16 => Self::R12x4UnormPack16,
            ffi::Format::R12x4g12x4Unorm2pack16 => Self::R12x4g12x4Unorm2pack16,
            ffi::Format::R12x4g12x4b12x4a12x4Unorm4pack16 => Self::R12x4g12x4b12x4a12x4Unorm4pack16,
            ffi::Format::G12x4b12x4g12x4r12x4_422Unorm4pack16 => {
                Self::G12x4b12x4g12x4r12x4_422Unorm4pack16
            }
            ffi::Format::B12x4g12x4r12x4g12x4_422Unorm4pack16 => {
                Self::B12x4g12x4r12x4g12x4_422Unorm4pack16
            }
            ffi::Format::G12x4B12x4R12x4_3plane420Unorm3pack16 => {
                Self::G12x4B12x4R12x4_3plane420Unorm3pack16
            }
            ffi::Format::G12x4B12x4r12x4_2plane420Unorm3pack16 => {
                Self::G12x4B12x4r12x4_2plane420Unorm3pack16
            }
            ffi::Format::G12x4B12x4R12x4_3plane422Unorm3pack16 => {
                Self::G12x4B12x4R12x4_3plane422Unorm3pack16
            }
            ffi::Format::G12x4B12x4r12x4_2plane422Unorm3pack16 => {
                Self::G12x4B12x4r12x4_2plane422Unorm3pack16
            }
            ffi::Format::G12x4B12x4R12x4_3plane444Unorm3pack16 => {
                Self::G12x4B12x4R12x4_3plane444Unorm3pack16
            }
            ffi::Format::G16b16g16r16_422Unorm => Self::G16b16g16r16_422Unorm,
            ffi::Format::B16g16r16g16_422Unorm => Self::B16g16r16g16_422Unorm,
            ffi::Format::G16B16R16_3plane420Unorm => Self::G16B16R16_3plane420Unorm,
            ffi::Format::G16B16r16_2plane420Unorm => Self::G16B16r16_2plane420Unorm,
            ffi::Format::G16B16R16_3plane422Unorm => Self::G16B16R16_3plane422Unorm,
            ffi::Format::G16B16r16_2plane422Unorm => Self::G16B16r16_2plane422Unorm,
            ffi::Format::G16B16R16_3plane444Unorm => Self::G16B16R16_3plane444Unorm,
            ffi::Format::Pvrtc1_2bppUnormBlockImg => Self::Pvrtc1_2bppUnormBlockImg,
            ffi::Format::Pvrtc1_4bppUnormBlockImg => Self::Pvrtc1_4bppUnormBlockImg,
            ffi::Format::Pvrtc2_2bppUnormBlockImg => Self::Pvrtc2_2bppUnormBlockImg,
            ffi::Format::Pvrtc2_4bppUnormBlockImg => Self::Pvrtc2_4bppUnormBlockImg,
            ffi::Format::Pvrtc1_2bppSrgbBlockImg => Self::Pvrtc1_2bppSrgbBlockImg,
            ffi::Format::Pvrtc1_4bppSrgbBlockImg => Self::Pvrtc1_4bppSrgbBlockImg,
            ffi::Format::Pvrtc2_2bppSrgbBlockImg => Self::Pvrtc2_2bppSrgbBlockImg,
            ffi::Format::Pvrtc2_4bppSrgbBlockImg => Self::Pvrtc2_4bppSrgbBlockImg,
            ffi::Format::Astc4x4SfloatBlockExt => Self::Astc4x4SfloatBlockExt,
            ffi::Format::Astc5x4SfloatBlockExt => Self::Astc5x4SfloatBlockExt,
            ffi::Format::Astc5x5SfloatBlockExt => Self::Astc5x5SfloatBlockExt,
            ffi::Format::Astc6x5SfloatBlockExt => Self::Astc6x5SfloatBlockExt,
            ffi::Format::Astc6x6SfloatBlockExt => Self::Astc6x6SfloatBlockExt,
            ffi::Format::Astc8x5SfloatBlockExt => Self::Astc8x5SfloatBlockExt,
            ffi::Format::Astc8x6SfloatBlockExt => Self::Astc8x6SfloatBlockExt,
            ffi::Format::Astc8x8SfloatBlockExt => Self::Astc8x8SfloatBlockExt,
            ffi::Format::Astc10x5SfloatBlockExt => Self::Astc10x5SfloatBlockExt,
            ffi::Format::Astc10x6SfloatBlockExt => Self::Astc10x6SfloatBlockExt,
            ffi::Format::Astc10x8SfloatBlockExt => Self::Astc10x8SfloatBlockExt,
            ffi::Format::Astc10x10SfloatBlockExt => Self::Astc10x10SfloatBlockExt,
            ffi::Format::Astc12x10SfloatBlockExt => Self::Astc12x10SfloatBlockExt,
            ffi::Format::Astc12x12SfloatBlockExt => Self::Astc12x12SfloatBlockExt,
            ffi::Format::G8B8r8_2plane444UnormExt => Self::G8B8r8_2plane444UnormExt,
            ffi::Format::G10x6B10x6r10x6_2plane444Unorm3pack16Ext => {
                Self::G10x6B10x6r10x6_2plane444Unorm3pack16Ext
            }
            ffi::Format::G12x4B12x4r12x4_2plane444Unorm3pack16Ext => {
                Self::G12x4B12x4r12x4_2plane444Unorm3pack16Ext
            }
            ffi::Format::G16B16r16_2plane444UnormExt => Self::G16B16r16_2plane444UnormExt,
            ffi::Format::A4r4g4b4UnormPack16Ext => Self::A4r4g4b4UnormPack16Ext,
            ffi::Format::A4b4g4r4UnormPack16Ext => Self::A4b4g4r4UnormPack16Ext,
        }
    }
}

impl From<Format> for ffi::Format {
    fn from(format: Format) -> Self {
        match format {
            Format::Undefined => Self::Undefined,
            Format::R4g4UnormPack8 => Self::R4g4UnormPack8,
            Format::R4g4b4a4UnormPack16 => Self::R4g4b4a4UnormPack16,
            Format::B4g4r4a4UnormPack16 => Self::B4g4r4a4UnormPack16,
            Format::R5g6b5UnormPack16 => Self::R5g6b5UnormPack16,
            Format::B5g6r5UnormPack16 => Self::B5g6r5UnormPack16,
            Format::R5g5b5a1UnormPack16 => Self::R5g5b5a1UnormPack16,
            Format::B5g5r5a1UnormPack16 => Self::B5g5r5a1UnormPack16,
            Format::A1r5g5b5UnormPack16 => Self::A1r5g5b5UnormPack16,
            Format::R8Unorm => Self::R8Unorm,
            Format::R8Snorm => Self::R8Snorm,
            Format::R8Uscaled => Self::R8Uscaled,
            Format::R8Sscaled => Self::R8Sscaled,
            Format::R8Uint => Self::R8Uint,
            Format::R8Sint => Self::R8Sint,
            Format::R8Srgb => Self::R8Srgb,
            Format::R8g8Unorm => Self::R8g8Unorm,
            Format::R8g8Snorm => Self::R8g8Snorm,
            Format::R8g8Uscaled => Self::R8g8Uscaled,
            Format::R8g8Sscaled => Self::R8g8Sscaled,
            Format::R8g8Uint => Self::R8g8Uint,
            Format::R8g8Sint => Self::R8g8Sint,
            Format::R8g8Srgb => Self::R8g8Srgb,
            Format::R8g8b8Unorm => Self::R8g8b8Unorm,
            Format::R8g8b8Snorm => Self::R8g8b8Snorm,
            Format::R8g8b8Uscaled => Self::R8g8b8Uscaled,
            Format::R8g8b8Sscaled => Self::R8g8b8Sscaled,
            Format::R8g8b8Uint => Self::R8g8b8Uint,
            Format::R8g8b8Sint => Self::R8g8b8Sint,
            Format::R8g8b8Srgb => Self::R8g8b8Srgb,
            Format::B8g8r8Unorm => Self::B8g8r8Unorm,
            Format::B8g8r8Snorm => Self::B8g8r8Snorm,
            Format::B8g8r8Uscaled => Self::B8g8r8Uscaled,
            Format::B8g8r8Sscaled => Self::B8g8r8Sscaled,
            Format::B8g8r8Uint => Self::B8g8r8Uint,
            Format::B8g8r8Sint => Self::B8g8r8Sint,
            Format::B8g8r8Srgb => Self::B8g8r8Srgb,
            Format::R8g8b8a8Unorm => Self::R8g8b8a8Unorm,
            Format::R8g8b8a8Snorm => Self::R8g8b8a8Snorm,
            Format::R8g8b8a8Uscaled => Self::R8g8b8a8Uscaled,
            Format::R8g8b8a8Sscaled => Self::R8g8b8a8Sscaled,
            Format::R8g8b8a8Uint => Self::R8g8b8a8Uint,
            Format::R8g8b8a8Sint => Self::R8g8b8a8Sint,
            Format::R8g8b8a8Srgb => Self::R8g8b8a8Srgb,
            Format::B8g8r8a8Unorm => Self::B8g8r8a8Unorm,
            Format::B8g8r8a8Snorm => Self::B8g8r8a8Snorm,
            Format::B8g8r8a8Uscaled => Self::B8g8r8a8Uscaled,
            Format::B8g8r8a8Sscaled => Self::B8g8r8a8Sscaled,
            Format::B8g8r8a8Uint => Self::B8g8r8a8Uint,
            Format::B8g8r8a8Sint => Self::B8g8r8a8Sint,
            Format::B8g8r8a8Srgb => Self::B8g8r8a8Srgb,
            Format::A8b8g8r8UnormPack32 => Self::A8b8g8r8UnormPack32,
            Format::A8b8g8r8SnormPack32 => Self::A8b8g8r8SnormPack32,
            Format::A8b8g8r8UscaledPack32 => Self::A8b8g8r8UscaledPack32,
            Format::A8b8g8r8SscaledPack32 => Self::A8b8g8r8SscaledPack32,
            Format::A8b8g8r8UintPack32 => Self::A8b8g8r8UintPack32,
            Format::A8b8g8r8SintPack32 => Self::A8b8g8r8SintPack32,
            Format::A8b8g8r8SrgbPack32 => Self::A8b8g8r8SrgbPack32,
            Format::A2r10g10b10UnormPack32 => Self::A2r10g10b10UnormPack32,
            Format::A2r10g10b10SnormPack32 => Self::A2r10g10b10SnormPack32,
            Format::A2r10g10b10UscaledPack32 => Self::A2r10g10b10UscaledPack32,
            Format::A2r10g10b10SscaledPack32 => Self::A2r10g10b10SscaledPack32,
            Format::A2r10g10b10UintPack32 => Self::A2r10g10b10UintPack32,
            Format::A2r10g10b10SintPack32 => Self::A2r10g10b10SintPack32,
            Format::A2b10g10r10UnormPack32 => Self::A2b10g10r10UnormPack32,
            Format::A2b10g10r10SnormPack32 => Self::A2b10g10r10SnormPack32,
            Format::A2b10g10r10UscaledPack32 => Self::A2b10g10r10UscaledPack32,
            Format::A2b10g10r10SscaledPack32 => Self::A2b10g10r10SscaledPack32,
            Format::A2b10g10r10UintPack32 => Self::A2b10g10r10UintPack32,
            Format::A2b10g10r10SintPack32 => Self::A2b10g10r10SintPack32,
            Format::R16Unorm => Self::R16Unorm,
            Format::R16Snorm => Self::R16Snorm,
            Format::R16Uscaled => Self::R16Uscaled,
            Format::R16Sscaled => Self::R16Sscaled,
            Format::R16Uint => Self::R16Uint,
            Format::R16Sint => Self::R16Sint,
            Format::R16Sfloat => Self::R16Sfloat,
            Format::R16g16Unorm => Self::R16g16Unorm,
            Format::R16g16Snorm => Self::R16g16Snorm,
            Format::R16g16Uscaled => Self::R16g16Uscaled,
            Format::R16g16Sscaled => Self::R16g16Sscaled,
            Format::R16g16Uint => Self::R16g16Uint,
            Format::R16g16Sint => Self::R16g16Sint,
            Format::R16g16Sfloat => Self::R16g16Sfloat,
            Format::R16g16b16Unorm => Self::R16g16b16Unorm,
            Format::R16g16b16Snorm => Self::R16g16b16Snorm,
            Format::R16g16b16Uscaled => Self::R16g16b16Uscaled,
            Format::R16g16b16Sscaled => Self::R16g16b16Sscaled,
            Format::R16g16b16Uint => Self::R16g16b16Uint,
            Format::R16g16b16Sint => Self::R16g16b16Sint,
            Format::R16g16b16Sfloat => Self::R16g16b16Sfloat,
            Format::R16g16b16a16Unorm => Self::R16g16b16a16Unorm,
            Format::R16g16b16a16Snorm => Self::R16g16b16a16Snorm,
            Format::R16g16b16a16Uscaled => Self::R16g16b16a16Uscaled,
            Format::R16g16b16a16Sscaled => Self::R16g16b16a16Sscaled,
            Format::R16g16b16a16Uint => Self::R16g16b16a16Uint,
            Format::R16g16b16a16Sint => Self::R16g16b16a16Sint,
            Format::R16g16b16a16Sfloat => Self::R16g16b16a16Sfloat,
            Format::R32Uint => Self::R32Uint,
            Format::R32Sint => Self::R32Sint,
            Format::R32Sfloat => Self::R32Sfloat,
            Format::R32g32Uint => Self::R32g32Uint,
            Format::R32g32Sint => Self::R32g32Sint,
            Format::R32g32Sfloat => Self::R32g32Sfloat,
            Format::R32g32b32Uint => Self::R32g32b32Uint,
            Format::R32g32b32Sint => Self::R32g32b32Sint,
            Format::R32g32b32Sfloat => Self::R32g32b32Sfloat,
            Format::R32g32b32a32Uint => Self::R32g32b32a32Uint,
            Format::R32g32b32a32Sint => Self::R32g32b32a32Sint,
            Format::R32g32b32a32Sfloat => Self::R32g32b32a32Sfloat,
            Format::R64Uint => Self::R64Uint,
            Format::R64Sint => Self::R64Sint,
            Format::R64Sfloat => Self::R64Sfloat,
            Format::R64g64Uint => Self::R64g64Uint,
            Format::R64g64Sint => Self::R64g64Sint,
            Format::R64g64Sfloat => Self::R64g64Sfloat,
            Format::R64g64b64Uint => Self::R64g64b64Uint,
            Format::R64g64b64Sint => Self::R64g64b64Sint,
            Format::R64g64b64Sfloat => Self::R64g64b64Sfloat,
            Format::R64g64b64a64Uint => Self::R64g64b64a64Uint,
            Format::R64g64b64a64Sint => Self::R64g64b64a64Sint,
            Format::R64g64b64a64Sfloat => Self::R64g64b64a64Sfloat,
            Format::B10g11r11UfloatPack32 => Self::B10g11r11UfloatPack32,
            Format::E5b9g9r9UfloatPack32 => Self::E5b9g9r9UfloatPack32,
            Format::D16Unorm => Self::D16Unorm,
            Format::X8D24UnormPack32 => Self::X8D24UnormPack32,
            Format::D32Sfloat => Self::D32Sfloat,
            Format::S8Uint => Self::S8Uint,
            Format::D16UnormS8Uint => Self::D16UnormS8Uint,
            Format::D24UnormS8Uint => Self::D24UnormS8Uint,
            Format::D32SfloatS8Uint => Self::D32SfloatS8Uint,
            Format::Bc1RgbUnormBlock => Self::Bc1RgbUnormBlock,
            Format::Bc1RgbSrgbBlock => Self::Bc1RgbSrgbBlock,
            Format::Bc1RgbaUnormBlock => Self::Bc1RgbaUnormBlock,
            Format::Bc1RgbaSrgbBlock => Self::Bc1RgbaSrgbBlock,
            Format::Bc2UnormBlock => Self::Bc2UnormBlock,
            Format::Bc2SrgbBlock => Self::Bc2SrgbBlock,
            Format::Bc3UnormBlock => Self::Bc3UnormBlock,
            Format::Bc3SrgbBlock => Self::Bc3SrgbBlock,
            Format::Bc4UnormBlock => Self::Bc4UnormBlock,
            Format::Bc4SnormBlock => Self::Bc4SnormBlock,
            Format::Bc5UnormBlock => Self::Bc5UnormBlock,
            Format::Bc5SnormBlock => Self::Bc5SnormBlock,
            Format::Bc6hUfloatBlock => Self::Bc6hUfloatBlock,
            Format::Bc6hSfloatBlock => Self::Bc6hSfloatBlock,
            Format::Bc7UnormBlock => Self::Bc7UnormBlock,
            Format::Bc7SrgbBlock => Self::Bc7SrgbBlock,
            Format::Etc2R8g8b8UnormBlock => Self::Etc2R8g8b8UnormBlock,
            Format::Etc2R8g8b8SrgbBlock => Self::Etc2R8g8b8SrgbBlock,
            Format::Etc2R8g8b8a1UnormBlock => Self::Etc2R8g8b8a1UnormBlock,
            Format::Etc2R8g8b8a1SrgbBlock => Self::Etc2R8g8b8a1SrgbBlock,
            Format::Etc2R8g8b8a8UnormBlock => Self::Etc2R8g8b8a8UnormBlock,
            Format::Etc2R8g8b8a8SrgbBlock => Self::Etc2R8g8b8a8SrgbBlock,
            Format::EacR11UnormBlock => Self::EacR11UnormBlock,
            Format::EacR11SnormBlock => Self::EacR11SnormBlock,
            Format::EacR11g11UnormBlock => Self::EacR11g11UnormBlock,
            Format::EacR11g11SnormBlock => Self::EacR11g11SnormBlock,
            Format::Astc4x4UnormBlock => Self::Astc4x4UnormBlock,
            Format::Astc4x4SrgbBlock => Self::Astc4x4SrgbBlock,
            Format::Astc5x4UnormBlock => Self::Astc5x4UnormBlock,
            Format::Astc5x4SrgbBlock => Self::Astc5x4SrgbBlock,
            Format::Astc5x5UnormBlock => Self::Astc5x5UnormBlock,
            Format::Astc5x5SrgbBlock => Self::Astc5x5SrgbBlock,
            Format::Astc6x5UnormBlock => Self::Astc6x5UnormBlock,
            Format::Astc6x5SrgbBlock => Self::Astc6x5SrgbBlock,
            Format::Astc6x6UnormBlock => Self::Astc6x6UnormBlock,
            Format::Astc6x6SrgbBlock => Self::Astc6x6SrgbBlock,
            Format::Astc8x5UnormBlock => Self::Astc8x5UnormBlock,
            Format::Astc8x5SrgbBlock => Self::Astc8x5SrgbBlock,
            Format::Astc8x6UnormBlock => Self::Astc8x6UnormBlock,
            Format::Astc8x6SrgbBlock => Self::Astc8x6SrgbBlock,
            Format::Astc8x8UnormBlock => Self::Astc8x8UnormBlock,
            Format::Astc8x8SrgbBlock => Self::Astc8x8SrgbBlock,
            Format::Astc10x5UnormBlock => Self::Astc10x5UnormBlock,
            Format::Astc10x5SrgbBlock => Self::Astc10x5SrgbBlock,
            Format::Astc10x6UnormBlock => Self::Astc10x6UnormBlock,
            Format::Astc10x6SrgbBlock => Self::Astc10x6SrgbBlock,
            Format::Astc10x8UnormBlock => Self::Astc10x8UnormBlock,
            Format::Astc10x8SrgbBlock => Self::Astc10x8SrgbBlock,
            Format::Astc10x10UnormBlock => Self::Astc10x10UnormBlock,
            Format::Astc10x10SrgbBlock => Self::Astc10x10SrgbBlock,
            Format::Astc12x10UnormBlock => Self::Astc12x10UnormBlock,
            Format::Astc12x10SrgbBlock => Self::Astc12x10SrgbBlock,
            Format::Astc12x12UnormBlock => Self::Astc12x12UnormBlock,
            Format::Astc12x12SrgbBlock => Self::Astc12x12SrgbBlock,
            Format::G8b8g8r8_422Unorm => Self::G8b8g8r8_422Unorm,
            Format::B8g8r8g8_422Unorm => Self::B8g8r8g8_422Unorm,
            Format::G8B8R8_3plane420Unorm => Self::G8B8R8_3plane420Unorm,
            Format::G8B8r8_2plane420Unorm => Self::G8B8r8_2plane420Unorm,
            Format::G8B8R8_3plane422Unorm => Self::G8B8R8_3plane422Unorm,
            Format::G8B8r8_2plane422Unorm => Self::G8B8r8_2plane422Unorm,
            Format::G8B8R8_3plane444Unorm => Self::G8B8R8_3plane444Unorm,
            Format::R10x6UnormPack16 => Self::R10x6UnormPack16,
            Format::R10x6g10x6Unorm2pack16 => Self::R10x6g10x6Unorm2pack16,
            Format::R10x6g10x6b10x6a10x6Unorm4pack16 => Self::R10x6g10x6b10x6a10x6Unorm4pack16,
            Format::G10x6b10x6g10x6r10x6_422Unorm4pack16 => {
                Self::G10x6b10x6g10x6r10x6_422Unorm4pack16
            }
            Format::B10x6g10x6r10x6g10x6_422Unorm4pack16 => {
                Self::B10x6g10x6r10x6g10x6_422Unorm4pack16
            }
            Format::G10x6B10x6R10x6_3plane420Unorm3pack16 => {
                Self::G10x6B10x6R10x6_3plane420Unorm3pack16
            }
            Format::G10x6B10x6r10x6_2plane420Unorm3pack16 => {
                Self::G10x6B10x6r10x6_2plane420Unorm3pack16
            }
            Format::G10x6B10x6R10x6_3plane422Unorm3pack16 => {
                Self::G10x6B10x6R10x6_3plane422Unorm3pack16
            }
            Format::G10x6B10x6r10x6_2plane422Unorm3pack16 => {
                Self::G10x6B10x6r10x6_2plane422Unorm3pack16
            }
            Format::G10x6B10x6R10x6_3plane444Unorm3pack16 => {
                Self::G10x6B10x6R10x6_3plane444Unorm3pack16
            }
            Format::R12x4UnormPack16 => Self::R12x4UnormPack16,
            Format::R12x4g12x4Unorm2pack16 => Self::R12x4g12x4Unorm2pack16,
            Format::R12x4g12x4b12x4a12x4Unorm4pack16 => Self::R12x4g12x4b12x4a12x4Unorm4pack16,
            Format::G12x4b12x4g12x4r12x4_422Unorm4pack16 => {
                Self::G12x4b12x4g12x4r12x4_422Unorm4pack16
            }
            Format::B12x4g12x4r12x4g12x4_422Unorm4pack16 => {
                Self::B12x4g12x4r12x4g12x4_422Unorm4pack16
            }
            Format::G12x4B12x4R12x4_3plane420Unorm3pack16 => {
                Self::G12x4B12x4R12x4_3plane420Unorm3pack16
            }
            Format::G12x4B12x4r12x4_2plane420Unorm3pack16 => {
                Self::G12x4B12x4r12x4_2plane420Unorm3pack16
            }
            Format::G12x4B12x4R12x4_3plane422Unorm3pack16 => {
                Self::G12x4B12x4R12x4_3plane422Unorm3pack16
            }
            Format::G12x4B12x4r12x4_2plane422Unorm3pack16 => {
                Self::G12x4B12x4r12x4_2plane422Unorm3pack16
            }
            Format::G12x4B12x4R12x4_3plane444Unorm3pack16 => {
                Self::G12x4B12x4R12x4_3plane444Unorm3pack16
            }
            Format::G16b16g16r16_422Unorm => Self::G16b16g16r16_422Unorm,
            Format::B16g16r16g16_422Unorm => Self::B16g16r16g16_422Unorm,
            Format::G16B16R16_3plane420Unorm => Self::G16B16R16_3plane420Unorm,
            Format::G16B16r16_2plane420Unorm => Self::G16B16r16_2plane420Unorm,
            Format::G16B16R16_3plane422Unorm => Self::G16B16R16_3plane422Unorm,
            Format::G16B16r16_2plane422Unorm => Self::G16B16r16_2plane422Unorm,
            Format::G16B16R16_3plane444Unorm => Self::G16B16R16_3plane444Unorm,
            Format::Pvrtc1_2bppUnormBlockImg => Self::Pvrtc1_2bppUnormBlockImg,
            Format::Pvrtc1_4bppUnormBlockImg => Self::Pvrtc1_4bppUnormBlockImg,
            Format::Pvrtc2_2bppUnormBlockImg => Self::Pvrtc2_2bppUnormBlockImg,
            Format::Pvrtc2_4bppUnormBlockImg => Self::Pvrtc2_4bppUnormBlockImg,
            Format::Pvrtc1_2bppSrgbBlockImg => Self::Pvrtc1_2bppSrgbBlockImg,
            Format::Pvrtc1_4bppSrgbBlockImg => Self::Pvrtc1_4bppSrgbBlockImg,
            Format::Pvrtc2_2bppSrgbBlockImg => Self::Pvrtc2_2bppSrgbBlockImg,
            Format::Pvrtc2_4bppSrgbBlockImg => Self::Pvrtc2_4bppSrgbBlockImg,
            Format::Astc4x4SfloatBlockExt => Self::Astc4x4SfloatBlockExt,
            Format::Astc5x4SfloatBlockExt => Self::Astc5x4SfloatBlockExt,
            Format::Astc5x5SfloatBlockExt => Self::Astc5x5SfloatBlockExt,
            Format::Astc6x5SfloatBlockExt => Self::Astc6x5SfloatBlockExt,
            Format::Astc6x6SfloatBlockExt => Self::Astc6x6SfloatBlockExt,
            Format::Astc8x5SfloatBlockExt => Self::Astc8x5SfloatBlockExt,
            Format::Astc8x6SfloatBlockExt => Self::Astc8x6SfloatBlockExt,
            Format::Astc8x8SfloatBlockExt => Self::Astc8x8SfloatBlockExt,
            Format::Astc10x5SfloatBlockExt => Self::Astc10x5SfloatBlockExt,
            Format::Astc10x6SfloatBlockExt => Self::Astc10x6SfloatBlockExt,
            Format::Astc10x8SfloatBlockExt => Self::Astc10x8SfloatBlockExt,
            Format::Astc10x10SfloatBlockExt => Self::Astc10x10SfloatBlockExt,
            Format::Astc12x10SfloatBlockExt => Self::Astc12x10SfloatBlockExt,
            Format::Astc12x12SfloatBlockExt => Self::Astc12x12SfloatBlockExt,
            Format::G8B8r8_2plane444UnormExt => Self::G8B8r8_2plane444UnormExt,
            Format::G10x6B10x6r10x6_2plane444Unorm3pack16Ext => {
                Self::G10x6B10x6r10x6_2plane444Unorm3pack16Ext
            }
            Format::G12x4B12x4r12x4_2plane444Unorm3pack16Ext => {
                Self::G12x4B12x4r12x4_2plane444Unorm3pack16Ext
            }
            Format::G16B16r16_2plane444UnormExt => Self::G16B16r16_2plane444UnormExt,
            Format::A4r4g4b4UnormPack16Ext => Self::A4r4g4b4UnormPack16Ext,
            Format::A4b4g4r4UnormPack16Ext => Self::A4b4g4r4UnormPack16Ext,
        }
    }
}

impl From<ffi::ColorSpaceKhr> for ColorSpaceKhr {
    fn from(color_space: ffi::ColorSpaceKhr) -> Self {
        match color_space {
            ffi::ColorSpaceKhr::SrgbNonlinearKhr => Self::SrgbNonlinearKhr,
            ffi::ColorSpaceKhr::DisplayP3NonlinearExt => Self::DisplayP3NonlinearExt,
            ffi::ColorSpaceKhr::ExtendedSrgbLinearExt => Self::ExtendedSrgbLinearExt,
            ffi::ColorSpaceKhr::DisplayP3LinearExt => Self::DisplayP3LinearExt,
            ffi::ColorSpaceKhr::DciP3NonlinearExt => Self::DciP3NonlinearExt,
            ffi::ColorSpaceKhr::Bt709LinearExt => Self::Bt709LinearExt,
            ffi::ColorSpaceKhr::Bt709NonlinearExt => Self::Bt709NonlinearExt,
            ffi::ColorSpaceKhr::Bt2020LinearExt => Self::Bt2020LinearExt,
            ffi::ColorSpaceKhr::Hdr10St2084Ext => Self::Hdr10St2084Ext,
            ffi::ColorSpaceKhr::DolbyvisionExt => Self::DolbyvisionExt,
            ffi::ColorSpaceKhr::Hdr10HlgExt => Self::Hdr10HlgExt,
            ffi::ColorSpaceKhr::AdobergbLinearExt => Self::AdobergbLinearExt,
            ffi::ColorSpaceKhr::AdobergbNonlinearExt => Self::AdobergbNonlinearExt,
            ffi::ColorSpaceKhr::PassThroughExt => Self::PassThroughExt,
            ffi::ColorSpaceKhr::ExtendedSrgbNonlinearExt => Self::ExtendedSrgbNonlinearExt,
            ffi::ColorSpaceKhr::DisplayNativeAmd => Self::DisplayNativeAmd,
        }
    }
}

impl From<ColorSpaceKhr> for ffi::ColorSpaceKhr {
    fn from(color_space: ColorSpaceKhr) -> Self {
        match color_space {
            ColorSpaceKhr::SrgbNonlinearKhr => Self::SrgbNonlinearKhr,
            ColorSpaceKhr::DisplayP3NonlinearExt => Self::DisplayP3NonlinearExt,
            ColorSpaceKhr::ExtendedSrgbLinearExt => Self::ExtendedSrgbLinearExt,
            ColorSpaceKhr::DisplayP3LinearExt => Self::DisplayP3LinearExt,
            ColorSpaceKhr::DciP3NonlinearExt => Self::DciP3NonlinearExt,
            ColorSpaceKhr::Bt709LinearExt => Self::Bt709LinearExt,
            ColorSpaceKhr::Bt709NonlinearExt => Self::Bt709NonlinearExt,
            ColorSpaceKhr::Bt2020LinearExt => Self::Bt2020LinearExt,
            ColorSpaceKhr::Hdr10St2084Ext => Self::Hdr10St2084Ext,
            ColorSpaceKhr::DolbyvisionExt => Self::DolbyvisionExt,
            ColorSpaceKhr::Hdr10HlgExt => Self::Hdr10HlgExt,
            ColorSpaceKhr::AdobergbLinearExt => Self::AdobergbLinearExt,
            ColorSpaceKhr::AdobergbNonlinearExt => Self::AdobergbNonlinearExt,
            ColorSpaceKhr::PassThroughExt => Self::PassThroughExt,
            ColorSpaceKhr::ExtendedSrgbNonlinearExt => Self::ExtendedSrgbNonlinearExt,
            ColorSpaceKhr::DisplayNativeAmd => Self::DisplayNativeAmd,
        }
    }
}

impl From<&SharingMode<'_>> for ffi::SharingMode {
    fn from(sharing_mode: &SharingMode) -> Self {
        match sharing_mode {
            SharingMode::Exclusive => Self::Exclusive,
            SharingMode::Concurrent(_) => Self::Concurrent,
        }
    }
}

impl From<ffi::SurfaceTransformFlagBitsKhr> for SurfaceTransformKhr {
    fn from(surface_transform: ffi::SurfaceTransformFlagBitsKhr) -> Self {
        match surface_transform {
            ffi::SurfaceTransformFlagBitsKhr::IdentityBitKhr => Self::IdentityKhr,
            ffi::SurfaceTransformFlagBitsKhr::Rotate90BitKhr => Self::Rotate90Khr,
            ffi::SurfaceTransformFlagBitsKhr::Rotate180BitKhr => Self::Rotate180Khr,
            ffi::SurfaceTransformFlagBitsKhr::Rotate270BitKhr => Self::Rotate270Khr,
            ffi::SurfaceTransformFlagBitsKhr::HorizontalMirrorBitKhr => Self::HorizontalMirrorKhr,
            ffi::SurfaceTransformFlagBitsKhr::HorizontalMirrorRotate90BitKhr => {
                Self::HorizontalMirrorRotate90Khr
            }
            ffi::SurfaceTransformFlagBitsKhr::HorizontalMirrorRotate180BitKhr => {
                Self::HorizontalMirrorRotate180Khr
            }
            ffi::SurfaceTransformFlagBitsKhr::HorizontalMirrorRotate270BitKhr => {
                Self::HorizontalMirrorRotate270Khr
            }
            ffi::SurfaceTransformFlagBitsKhr::InheritBitKhr => Self::InheritKhr,
        }
    }
}

impl From<SurfaceTransformKhr> for ffi::SurfaceTransformFlagBitsKhr {
    fn from(surface_transform: SurfaceTransformKhr) -> Self {
        match surface_transform {
            SurfaceTransformKhr::IdentityKhr => Self::IdentityBitKhr,
            SurfaceTransformKhr::Rotate90Khr => Self::Rotate90BitKhr,
            SurfaceTransformKhr::Rotate180Khr => Self::Rotate180BitKhr,
            SurfaceTransformKhr::Rotate270Khr => Self::Rotate270BitKhr,
            SurfaceTransformKhr::HorizontalMirrorKhr => Self::HorizontalMirrorBitKhr,
            SurfaceTransformKhr::HorizontalMirrorRotate90Khr => {
                Self::HorizontalMirrorRotate90BitKhr
            }
            SurfaceTransformKhr::HorizontalMirrorRotate180Khr => {
                Self::HorizontalMirrorRotate180BitKhr
            }
            SurfaceTransformKhr::HorizontalMirrorRotate270Khr => {
                Self::HorizontalMirrorRotate270BitKhr
            }
            SurfaceTransformKhr::InheritKhr => Self::InheritBitKhr,
        }
    }
}

impl From<CompositeAlphaKhr> for ffi::CompositeAlphaFlagBitsKhr {
    fn from(composite_alpha: CompositeAlphaKhr) -> Self {
        match composite_alpha {
            CompositeAlphaKhr::OpaqueKhr => Self::OpaqueBitKhr,
            CompositeAlphaKhr::PreMultipliedKhr => Self::PreMultipliedBitKhr,
            CompositeAlphaKhr::PostMultipliedKhr => Self::PostMultipliedBitKhr,
            CompositeAlphaKhr::InheritKhr => Self::InheritBitKhr,
        }
    }
}

impl SurfaceTransformFlagsKhr {
    pub fn contains(&self, flag: SurfaceTransformKhr) -> bool {
        let flag = ffi::SurfaceTransformFlagBitsKhr::from(flag);

        self.0 & (flag as u32) != 0
    }
}

impl CompositeAlphaFlagsKhr {
    pub fn contains(&self, flag: CompositeAlphaKhr) -> bool {
        let flag = ffi::CompositeAlphaFlagBitsKhr::from(flag);

        self.0 & (flag as u32) != 0
    }
}

impl SwapchainCreateFlagsBuilderKhr {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn split_instance_bind_regions(&mut self, val: bool) -> &mut Self {
        if val {
            self.0 |= ffi::SwapchainCreateFlagBitsKhr::SplitInstanceBindRegionsBitKhr as u32;
        }
        self
    }

    pub fn protected(&mut self, val: bool) -> &mut Self {
        if val {
            self.0 |= ffi::SwapchainCreateFlagBitsKhr::ProtectedBitKhr as u32;
        }
        self
    }

    pub fn mutable_format(&mut self, val: bool) -> &mut Self {
        if val {
            self.0 |= ffi::SwapchainCreateFlagBitsKhr::MutableFormatBitKhr as u32;
        }
        self
    }

    pub fn build(&self) -> SwapchainCreateFlagsKhr {
        SwapchainCreateFlagsKhr(self.0)
    }
}

impl ImageUsageFlags {
    pub fn contains(&self, flag: ImageUsage) -> bool {
        let flag = ffi::ImageUsageFlagBits::from(flag);

        self.0 & flag as u32 != 0
    }
}

impl From<ImageUsage> for ffi::ImageUsageFlagBits {
    fn from(image_usage: ImageUsage) -> Self {
        match image_usage {
            ImageUsage::TransferSrc => Self::TransferSrcBit,
            ImageUsage::TransferDst => Self::TransferDstBit,
            ImageUsage::Sampled => Self::SampledBit,
            ImageUsage::Storage => Self::StorageBit,
            ImageUsage::ColorAttachment => Self::ColorAttachmentBit,
            ImageUsage::DepthStencilAttachment => Self::DepthStencilAttachmentBit,
            ImageUsage::TransientAttachment => Self::TransientAttachmentBit,
            ImageUsage::InputAttachment => Self::InputAttachmentBit,
            ImageUsage::ShadingRateImageNv => Self::ShadingRateImageBitNv,
            ImageUsage::FragmentDensityMapExt => Self::FragmentDensityMapBitExt,
        }
    }
}

impl From<ffi::PresentModeKhr> for PresentModeKhr {
    fn from(present_mode: ffi::PresentModeKhr) -> Self {
        match present_mode {
            ffi::PresentModeKhr::ImmediateKhr => Self::ImmediateKhr,
            ffi::PresentModeKhr::MailboxKhr => Self::MailboxKhr,
            ffi::PresentModeKhr::FifoKhr => Self::FifoKhr,
            ffi::PresentModeKhr::FifoRelaxedKhr => Self::FifoRelaxedKhr,
            ffi::PresentModeKhr::SharedDemandRefreshKhr => Self::SharedDemandRefreshKhr,
            ffi::PresentModeKhr::SharedContinuousRefreshKhr => Self::SharedContinuousRefreshKhr,
        }
    }
}

impl From<PresentModeKhr> for ffi::PresentModeKhr {
    fn from(present_mode: PresentModeKhr) -> Self {
        match present_mode {
            PresentModeKhr::ImmediateKhr => Self::ImmediateKhr,
            PresentModeKhr::MailboxKhr => Self::MailboxKhr,
            PresentModeKhr::FifoKhr => Self::FifoKhr,
            PresentModeKhr::FifoRelaxedKhr => Self::FifoRelaxedKhr,
            PresentModeKhr::SharedDemandRefreshKhr => Self::SharedDemandRefreshKhr,
            PresentModeKhr::SharedContinuousRefreshKhr => Self::SharedContinuousRefreshKhr,
        }
    }
}

impl ImageUsageFlagsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn transfer_src(&mut self, transfer_src: bool) -> &mut Self {
        if transfer_src {
            self.0 |= ffi::ImageUsageFlagBits::TransferSrcBit as u32;
        }
        self
    }

    pub fn transfer_dst(&mut self, transfer_dst: bool) -> &mut Self {
        if transfer_dst {
            self.0 |= ffi::ImageUsageFlagBits::TransferDstBit as u32;
        }
        self
    }

    pub fn sampled(&mut self, sampled: bool) -> &mut Self {
        if sampled {
            self.0 |= ffi::ImageUsageFlagBits::SampledBit as u32;
        }
        self
    }

    pub fn storage(&mut self, storage: bool) -> &mut Self {
        if storage {
            self.0 |= ffi::ImageUsageFlagBits::StorageBit as u32;
        }
        self
    }

    pub fn color_attachment(&mut self, color_attachment: bool) -> &mut Self {
        if color_attachment {
            self.0 |= ffi::ImageUsageFlagBits::ColorAttachmentBit as u32;
        }
        self
    }

    pub fn depth_stencil_attachment(&mut self, depth_stencil_attachment: bool) -> &mut Self {
        if depth_stencil_attachment {
            self.0 |= ffi::ImageUsageFlagBits::DepthStencilAttachmentBit as u32;
        }
        self
    }

    pub fn transient_attachment(&mut self, transient_attachment: bool) -> &mut Self {
        if transient_attachment {
            self.0 |= ffi::ImageUsageFlagBits::TransientAttachmentBit as u32;
        }
        self
    }

    pub fn input_attachment(&mut self, input_attachment: bool) -> &mut Self {
        if input_attachment {
            self.0 |= ffi::ImageUsageFlagBits::InputAttachmentBit as u32;
        }
        self
    }

    pub fn shading_rate_image_nv(&mut self, shading_rate_image_nv: bool) -> &mut Self {
        if shading_rate_image_nv {
            self.0 |= ffi::ImageUsageFlagBits::ShadingRateImageBitNv as u32;
        }
        self
    }

    pub fn fragment_density_map_ext(&mut self, fragment_density_map_ext: bool) -> &mut Self {
        if fragment_density_map_ext {
            self.0 |= ffi::ImageUsageFlagBits::FragmentDensityMapBitExt as u32;
        }
        self
    }

    pub fn build(&self) -> ImageUsageFlags {
        ImageUsageFlags(self.0)
    }
}
