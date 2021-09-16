use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::device::Device;
use crate::ffi;

pub struct CommandBuffer<'a> {
    handle: NonNull<ffi::OpaqueCommandBuffer>,
    _marker: PhantomData<(ffi::OpaqueCommandBuffer, &'a CommandPool<'a>)>,
}

pub struct CommandPool<'a> {
    #[cfg(target_pointer_width = "64")]
    pub(crate) handle: NonNull<ffi::OpaqueCommandPool>,
    #[cfg(not(target_pointer_width = "64"))]
    pub(crate) handle: NonZeroU64,
    pub(crate) device: &'a Device<'a>,
    #[cfg(target_pointer_width = "64")]
    pub(crate) _marker: PhantomData<ffi::OpaqueCommandPool>,
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
