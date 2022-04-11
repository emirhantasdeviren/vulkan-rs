use std::marker::PhantomData;
#[cfg(not(target_pointer_width = "64"))]
use std::num::NonZeroU64;
use std::ptr::NonNull;

use crate::device::Device;
use crate::ffi;

pub struct ShaderModule<'a> {
    #[cfg(target_pointer_width = "64")]
    pub(crate) handle: NonNull<ffi::VkShaderModule_T>,
    #[cfg(not(target_pointer_width = "64"))]
    pub(crate) handle: NonZeroU64,
    pub(crate) device: &'a Device<'a>,
    #[cfg(target_pointer_width = "64")]
    pub(crate) _marker: PhantomData<ffi::VkShaderModule_T>,
}

impl<'a> Drop for ShaderModule<'a> {
    fn drop(&mut self) {
        println!("Dropped ShaderModule");
        unsafe {
            (self.device.dispatch_loader.vk_destroy_shader_module)(
                self.device.handle.as_ptr(),
                #[cfg(target_pointer_width = "64")]
                self.handle.as_ptr(),
                #[cfg(not(target_pointer_width = "64"))]
                self.handle.get(),
                std::ptr::null(),
            )
        }
    }
}

impl<'a> std::fmt::Debug for ShaderModule<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShaderModule")
            .field("handle", &self.handle)
            .finish_non_exhaustive()
    }
}
