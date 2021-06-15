use std::ffi::c_void;
use std::marker::{PhantomData, PhantomPinned};

#[repr(C)]
pub struct OpaqueInstance {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct OpaquePhysicalDevice {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub enum StructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
}

#[repr(C)]
enum SystemAllocationScope {
    Command = 0,
    Object = 1,
    Cache = 2,
    Device = 3,
    Instance = 4,
    MaxEnum = 0x7FFFFFFF,
}

#[repr(C)]
enum InternalAllocationType {
    Executable = 0,
    MaxEnum = 0x7FFFFFFF,
}

type PFN_vkAllocationFunction = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut c_void;
type PFN_vkFreeFunction = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    p_memory: *mut c_void,
);
type PFN_vkInternalAllocationNotification = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);
type PFN_vkInternalFreeNotification = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);
type PFN_vkReallocationFunction = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    p_original: *mut c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut c_void;

pub type PFN_vkVoidFunction = unsafe extern "system" fn();

#[repr(C)]
pub struct AllocationCallbacks {
    p_user_data: *mut c_void,
    pfn_allocation: PFN_vkAllocationFunction,
    pfn_reallocation: PFN_vkReallocationFunction,
    pfn_free: PFN_vkFreeFunction,
    pfn_internal_allocation: PFN_vkInternalAllocationNotification,
    pfn_internal_free: PFN_vkInternalFreeNotification,
}

pub type PFN_vkGetInstanceProcAddr =
    unsafe extern "system" fn(*mut OpaqueInstance, *const i8) -> Option<PFN_vkVoidFunction>;
pub type PFN_vkEnumerateInstanceVersion = unsafe extern "system" fn(*mut u32) -> i32;
pub type PFN_vkCreateInstance = unsafe extern "system" fn(
    p_create_info: *const InstanceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_instance: *mut *mut OpaqueInstance,
) -> i32;
pub type PFN_vkDestroyInstance = unsafe extern "system" fn(*mut OpaqueInstance, *const c_void);
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(
    instance: *mut OpaqueInstance,
    pPhysicalDeviceCount: *mut u32,
    pPhysicalDevices: *mut *mut OpaquePhysicalDevice,
) -> i32;

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
