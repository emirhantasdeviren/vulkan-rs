use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

#[cfg(not(target_pointer_width = "64"))]
use std::num::NonZeroU64;

use crate::core::{Error, Extent2D, Result};
use crate::device::Device;
use crate::ffi;
use crate::format::Format;
use crate::init::Instance;
use crate::resource::{ImageUsageFlags, SharingMode};

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

pub struct SurfaceKhr<'a> {
    #[cfg(target_pointer_width = "64")]
    pub(crate) handle: NonNull<ffi::OpaqueSurfaceKhr>,
    #[cfg(not(target_pointer_width = "64"))]
    pub(crate) handle: NonZeroU64,
    pub(crate) instance: &'a Instance,
    #[cfg(target_pointer_width = "64")]
    pub(crate) _marker: PhantomData<ffi::OpaqueSurfaceKhr>,
}

pub struct SwapchainKhr<'a> {
    #[cfg(target_pointer_width = "64")]
    pub(crate) handle: NonNull<ffi::OpaqueSwapchainKhr>,
    #[cfg(not(target_pointer_width = "64"))]
    pub(crate) handle: NonZeroU64,
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

pub struct SurfaceTransformFlagsKhr(pub(crate) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeAlphaKhr {
    OpaqueKhr,
    PreMultipliedKhr,
    PostMultipliedKhr,
    InheritKhr,
}

pub struct CompositeAlphaFlagsKhr(pub(crate) u32);

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
