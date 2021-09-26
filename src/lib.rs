//! # Vulkan API
//! Vulkan is a low-overhead, cross-platform API, open standard for 3D graphics and computing.
pub mod command_buffer;
pub mod core;
/// Devices and Queues are the primary objects used to interact with a Vulkan implementation.
pub mod device;
pub mod format;
/// An application must initialize Vulkan by creating [`Instance`](init::Instance) object.
pub mod init;
pub mod pipeline;
pub mod resource;
pub mod shaders;
pub mod sync;
/// Window System Integration, between Vulkan and the various forms of displaying the results of
/// rendering to a user.
pub mod wsi;

mod ffi;
mod linker;
