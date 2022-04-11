use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use crate::command_buffer::CommandPool;
use crate::core::{Error, Result};
use crate::ffi;
use crate::init::{ApiVersion, DispatchLoaderDevice, DispatchLoaderPhysicalDevice, Instance};
use crate::resource::{Image, ImageUsageFlags};
use crate::shaders::ShaderModule;
use crate::sync::Semaphore;
use crate::wsi::{
    CompositeAlphaFlagsKhr, PresentModeKhr, SurfaceCapabilitiesKhr, SurfaceFormatKhr, SurfaceKhr,
    SurfaceTransformFlagsKhr, SwapchainKhr,
};

pub struct PhysicalDevice<'a> {
    pub(crate) handle: NonNull<ffi::VkPhysicalDevice_T>,
    pub(crate) dispatch_loader: DispatchLoaderPhysicalDevice,
    pub(crate) _marker: PhantomData<(ffi::VkPhysicalDevice_T, &'a Instance)>,
}

pub struct Device<'a> {
    pub(super) handle: NonNull<ffi::VkDevice_T>,
    pub(super) dispatch_loader: DispatchLoaderDevice,
    _marker: PhantomData<(ffi::VkDevice_T, &'a Instance)>,
}

#[derive(Debug)]
pub struct Queue<'a> {
    handle: NonNull<ffi::VkQueue_T>,
    _marker: PhantomData<(ffi::VkQueue_T, &'a Device<'a>)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalDeviceType {
    Other,
    IntegratedGpu,
    DiscreteGpu,
    VirtualGpu,
    Cpu,
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
            api_version: ApiVersion::from(props.api_version),
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

    /// # Errors
    ///
    /// If `"VK_KHR_surface"` extension is not enabled then [`None`] is
    /// returned.
    ///
    /// List of possible [`Error`] variants.
    /// - [`OutOfHostMemory`](Error::OutOfHostMemory)
    /// - [`OutOfDeviceMemory`](Error::OutOfDeviceMemory)
    /// - [`SurfaceLostKhr`](Error::SurfaceLostKhr)
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
    pub fn get_queue(&self, queue_family_index: usize, queue_index: usize) -> Option<Queue<'_>> {
        let mut handle = MaybeUninit::uninit();
        unsafe {
            (self.dispatch_loader.vk_get_device_queue)(
                self.handle.as_ptr(),
                queue_family_index as u32,
                queue_index as u32,
                handle.as_mut_ptr(),
            );
        }

        NonNull::new(unsafe { handle.assume_init() }).map(|handle| Queue {
            handle,
            _marker: PhantomData,
        })
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
                            Ok(swapchain_images
                                .into_iter()
                                .map(|image| Image {
                                    #[cfg(target_pointer_width = "64")]
                                    handle: unsafe { NonNull::new_unchecked(image) },
                                    #[cfg(not(target_pointer_width = "64"))]
                                    handle: unsafe { NonZeroU64::new_unchecked(image) },
                                    device: self,
                                    #[cfg(target_pointer_width = "64")]
                                    _marker: PhantomData,
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

    pub fn create_shader_module(&self, code: &[u8]) -> Result<ShaderModule<'_>> {
        let create_info = ffi::ShaderModuleCreateInfo {
            s_type: ffi::StructureType::ShaderModuleCreateInfo,
            p_next: std::ptr::null(),
            flags: 0,
            code_size: code.len(),
            p_code: code.as_ptr().cast(),
        };

        let mut p_shader_module = MaybeUninit::uninit();

        let result = unsafe {
            (self.dispatch_loader.vk_create_shader_module)(
                self.handle.as_ptr(),
                &create_info,
                std::ptr::null(),
                p_shader_module.as_mut_ptr(),
            )
        };

        match result {
            ffi::Result::Success => Ok(ShaderModule {
                #[cfg(target_pointer_width = "64")]
                handle: unsafe { NonNull::new_unchecked(p_shader_module.assume_init()) },
                #[cfg(not(target_pointer_width = "64"))]
                handle: unsafe { NonZeroU64::new_unchecked(p_shader_module.assume_init()) },
                device: self,
                #[cfg(target_pointer_width = "64")]
                _marker: PhantomData,
            }),
            ffi::Result::ErrorOutOfHostMemory => Err(Error::OutOfHostMemory),
            ffi::Result::ErrorOutOfDeviceMemory => Err(Error::OutOfDeviceMemory),
            ffi::Result::ErrorInvalidShaderNv => Err(Error::InvalidShaderNv),
            _ => unreachable!(),
        }
    }
}

impl<'a> Drop for Device<'a> {
    fn drop(&mut self) {
        println!("Dropped Device");
        unsafe { (self.dispatch_loader.vk_destroy_device)(self.handle.as_ptr(), std::ptr::null()) }
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
