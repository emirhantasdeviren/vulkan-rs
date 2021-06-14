use std::ffi::c_void;
use std::marker::{PhantomData, PhantomPinned};

#[repr(C)]
pub struct OpaqueInstance {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub enum StructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
}

pub type PFN_vkVoidFunction = unsafe extern "system" fn();
pub type PFN_vkGetInstanceProcAddr =
    unsafe extern "system" fn(*mut OpaqueInstance, *const i8) -> Option<PFN_vkVoidFunction>;
pub type PFN_vkEnumerateInstanceVersion = unsafe extern "system" fn(*mut u32) -> i32;
pub type PFN_vkCreateInstance = unsafe extern "system" fn(
    p_create_info: *const InstanceCreateInfo,
    p_allocator: *const c_void,
    p_instance: *mut *mut OpaqueInstance,
) -> i32;
pub type PFN_vkDestroyInstance = unsafe extern "system" fn(*mut OpaqueInstance, *const c_void);

type Flags = u32;
type InstanceCreateFlags = Flags;

#[repr(C)]
pub struct ApplicationInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_application_name: *const i8,
    pub application_version: u32,
    pub p_engine_name: *const i8,
    pub engine_version: u32,
    pub api_version: u32,
}

#[repr(C)]
pub struct InstanceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: InstanceCreateFlags,
    pub p_application_info: *const ApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const i8,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const i8,
}
