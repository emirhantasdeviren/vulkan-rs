use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use crate::core::{Error, Result};
use crate::device::Device;
use crate::ffi;
use crate::format::Format;

pub struct Image<'a> {
    #[cfg(target_pointer_width = "64")]
    pub(crate) handle: NonNull<ffi::OpaqueImage>,
    #[cfg(not(target_pointer_width = "64"))]
    pub(crate) handle: NonZeroU64,
    pub(crate) device: &'a Device<'a>,
    #[cfg(target_pointer_width = "64")]
    pub(crate) _marker: PhantomData<ffi::OpaqueImage>,
}

pub struct ImageView<'a> {
    #[cfg(target_pointer_width = "64")]
    handle: NonNull<ffi::OpaqueImageView>,
    #[cfg(not(target_pointer_width = "64"))]
    handle: NonZeroU64,
    device: &'a Device<'a>,
    #[cfg(target_pointer_width = "64")]
    _marker: PhantomData<ffi::OpaqueImageView>,
}

pub struct ImageViewBuilder<'a> {
    flags: ImageViewCreateFlags,
    image: &'a Image<'a>,
    view_type: ImageViewType,
    format: Format,
    components: ComponentMapping,
    subresource_range: ImageSubresourceRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageViewType {
    OneD,
    TwoD,
    ThreeD,
    Cube,
    OneDArray,
    TwoDArray,
    CubeArray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageAspect {
    Color,
    Depth,
    Stencil,
    Metadata,
    Plane0,
    Plane1,
    Plane2,
    MemoryPlane0Ext,
    MemoryPlane1Ext,
    MemoryPlane2Ext,
    MemoryPlane3Ext,
}
#[derive(Default)]
pub struct ImageAspectFlags(u32);
#[derive(Default)]
pub struct ImageAspectFlagsBuilder(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentSwizzle {
    Identity,
    Zero,
    One,
    Red,
    Green,
    Blue,
    Alpha,
}

pub enum SharingMode<'a> {
    Exclusive,
    Concurrent(&'a [u32]),
}

pub struct ImageSubresourceRange {
    aspect_mask: ImageAspectFlags,
    base_mip_level: u32,
    level_count: u32,
    base_array_layer: u32,
    layer_count: u32,
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
pub struct ImageUsageFlags(pub(crate) u32);

#[derive(Default)]
pub struct ImageUsageFlagsBuilder(u32);

pub enum ImageViewCreate {
    FragmentDensityMapDynamicExt,
    FragmentDensityMapDeferredExt,
}

#[derive(Debug, Default)]
pub struct ImageViewCreateFlags(u32);

#[derive(Default)]
pub struct ImageViewCreateFlagsBuilder(u32);

#[derive(Default, Clone, Copy)]
pub struct ComponentMapping {
    red: ComponentSwizzle,
    green: ComponentSwizzle,
    blue: ComponentSwizzle,
    alpha: ComponentSwizzle,
}

impl<'a> Drop for ImageView<'a> {
    fn drop(&mut self) {
        println!("Dropped ImageView");
        unsafe {
            (self.device.dispatch_loader.vk_destroy_image_view)(
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

impl<'a> ImageViewBuilder<'a> {
    pub fn new(
        image: &'a Image<'a>,
        view_type: ImageViewType,
        format: Format,
        subresource_range: ImageSubresourceRange,
    ) -> Self {
        Self {
            flags: Default::default(),
            image,
            view_type,
            format,
            components: Default::default(),
            subresource_range,
        }
    }

    pub fn with_components(&mut self, components: ComponentMapping) -> &mut Self {
        self.components = components;
        self
    }

    pub fn with_flags(&mut self, _flags: ImageViewCreateFlags) -> &mut Self {
        todo!()
    }

    pub fn build(self, device: &'a Device) -> Result<ImageView<'a>> {
        let create_info = ffi::ImageViewCreateInfo {
            s_type: ffi::StructureType::ImageViewCreateInfo,
            p_next: std::ptr::null(),
            flags: self.flags.0,
            #[cfg(target_pointer_width = "64")]
            image: self.image.handle.as_ptr(),
            #[cfg(not(target_pointer_width = "64"))]
            image: self.image.handle.get(),
            view_type: self.view_type.into(),
            format: self.format.into(),
            components: self.components.into(),
            subresource_range: self.subresource_range.into(),
        };

        let mut p_view = MaybeUninit::uninit();

        let result = unsafe {
            (device.dispatch_loader.vk_create_image_view)(
                device.handle.as_ptr(),
                &create_info,
                std::ptr::null(),
                p_view.as_mut_ptr(),
            )
        };

        match result {
            ffi::Result::Success => Ok(ImageView {
                #[cfg(target_pointer_width = "64")]
                handle: unsafe { NonNull::new_unchecked(p_view.assume_init()) },
                #[cfg(not(target_pointer_width = "64"))]
                handle: unsafe { NonZeroU64::new_unchecked(p_view.assume_init()) },
                device,
                #[cfg(target_pointer_width = "64")]
                _marker: PhantomData,
            }),
            ffi::Result::ErrorOutOfHostMemory => Err(Error::OutOfHostMemory),
            ffi::Result::ErrorOutOfDeviceMemory => Err(Error::OutOfDeviceMemory),
            _ => unreachable!(),
        }
    }
}

impl From<ImageViewType> for ffi::ImageViewType {
    fn from(view_type: ImageViewType) -> Self {
        match view_type {
            ImageViewType::OneD => Self::OneD,
            ImageViewType::TwoD => Self::TwoD,
            ImageViewType::ThreeD => Self::ThreeD,
            ImageViewType::Cube => Self::Cube,
            ImageViewType::OneDArray => Self::OneDArray,
            ImageViewType::TwoDArray => Self::TwoDArray,
            ImageViewType::CubeArray => Self::CubeArray,
        }
    }
}

impl ImageAspectFlagsBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color(&mut self, color: bool) -> &mut Self {
        if color {
            self.0 |= ffi::ImageAspectFlagBits::ColorBit as u32;
        }
        self
    }

    pub fn depth(&mut self, depth: bool) -> &mut Self {
        if depth {
            self.0 |= ffi::ImageAspectFlagBits::DepthBit as u32;
        }
        self
    }

    pub fn stencil(&mut self, stencil: bool) -> &mut Self {
        if stencil {
            self.0 |= ffi::ImageAspectFlagBits::StencilBit as u32;
        }
        self
    }

    pub fn metadata(&mut self, metadata: bool) -> &mut Self {
        if metadata {
            self.0 |= ffi::ImageAspectFlagBits::MetadataBit as u32;
        }
        self
    }

    pub fn plane0(&mut self, plane0: bool) -> &mut Self {
        if plane0 {
            self.0 |= ffi::ImageAspectFlagBits::Plane0Bit as u32;
        }
        self
    }

    pub fn plane1(&mut self, plane1: bool) -> &mut Self {
        if plane1 {
            self.0 |= ffi::ImageAspectFlagBits::Plane1Bit as u32;
        }
        self
    }

    pub fn plane2(&mut self, plane2: bool) -> &mut Self {
        if plane2 {
            self.0 |= ffi::ImageAspectFlagBits::Plane2Bit as u32;
        }
        self
    }

    pub fn memory_plane0(&mut self, memory_plane0: bool) -> &mut Self {
        if memory_plane0 {
            self.0 |= ffi::ImageAspectFlagBits::MemoryPlane0BitExt as u32;
        }
        self
    }

    pub fn memory_plane1(&mut self, memory_plane1: bool) -> &mut Self {
        if memory_plane1 {
            self.0 |= ffi::ImageAspectFlagBits::MemoryPlane1BitExt as u32;
        }
        self
    }

    pub fn memory_plane2(&mut self, memory_plane2: bool) -> &mut Self {
        if memory_plane2 {
            self.0 |= ffi::ImageAspectFlagBits::MemoryPlane2BitExt as u32;
        }
        self
    }

    pub fn memory_plane3(&mut self, memory_plane3: bool) -> &mut Self {
        if memory_plane3 {
            self.0 |= ffi::ImageAspectFlagBits::MemoryPlane3BitExt as u32;
        }
        self
    }

    pub fn build(&self) -> ImageAspectFlags {
        ImageAspectFlags(self.0)
    }
}

impl Default for ComponentSwizzle {
    fn default() -> Self {
        Self::Identity
    }
}

impl ImageSubresourceRange {
    pub fn new(
        aspect_mask: ImageAspectFlags,
        base_mip_level: u32,
        level_count: u32,
        base_array_layer: u32,
        layer_count: u32,
    ) -> Self {
        Self {
            aspect_mask,
            base_mip_level,
            level_count,
            base_array_layer,
            layer_count,
        }
    }
}

impl From<ImageSubresourceRange> for ffi::ImageSubresourceRange {
    fn from(subresource_range: ImageSubresourceRange) -> Self {
        Self {
            aspect_mask: subresource_range.aspect_mask.0,
            base_mip_level: subresource_range.base_mip_level,
            level_count: subresource_range.level_count,
            base_array_layer: subresource_range.base_array_layer,
            layer_count: subresource_range.layer_count,
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

impl From<ComponentSwizzle> for ffi::ComponentSwizzle {
    fn from(component: ComponentSwizzle) -> Self {
        match component {
            ComponentSwizzle::Identity => Self::Identity,
            ComponentSwizzle::Zero => Self::Zero,
            ComponentSwizzle::One => Self::One,
            ComponentSwizzle::Red => Self::Red,
            ComponentSwizzle::Green => Self::Green,
            ComponentSwizzle::Blue => Self::Blue,
            ComponentSwizzle::Alpha => Self::Alpha,
        }
    }
}

impl From<ffi::ComponentSwizzle> for ComponentSwizzle {
    fn from(component: ffi::ComponentSwizzle) -> Self {
        match component {
            ffi::ComponentSwizzle::Identity => Self::Identity,
            ffi::ComponentSwizzle::Zero => Self::Zero,
            ffi::ComponentSwizzle::One => Self::One,
            ffi::ComponentSwizzle::Red => Self::Red,
            ffi::ComponentSwizzle::Green => Self::Green,
            ffi::ComponentSwizzle::Blue => Self::Blue,
            ffi::ComponentSwizzle::Alpha => Self::Alpha,
        }
    }
}

impl ComponentMapping {
    pub fn new(
        red: ComponentSwizzle,
        green: ComponentSwizzle,
        blue: ComponentSwizzle,
        alpha: ComponentSwizzle,
    ) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl From<ComponentMapping> for ffi::ComponentMapping {
    fn from(components: ComponentMapping) -> Self {
        Self {
            r: components.red.into(),
            g: components.green.into(),
            b: components.blue.into(),
            a: components.alpha.into(),
        }
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
