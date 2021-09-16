use std::marker::PhantomData;
#[cfg(not(target_pointer_width = "64"))]
use std::num::NonZeroU64;
#[cfg(target_pointer_width = "64")]
use std::ptr::NonNull;

use crate::device::Device;
use crate::ffi;

pub struct Semaphore<'a> {
    #[cfg(target_pointer_width = "64")]
    pub(crate) handle: NonNull<ffi::OpaqueSemaphore>,
    #[cfg(not(target_pointer_width = "64"))]
    pub(crate) handle: NonZeroU64,
    pub(crate) device: &'a Device<'a>,
    #[cfg(target_pointer_width = "64")]
    pub(crate) _marker: PhantomData<ffi::OpaqueSemaphore>,
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
