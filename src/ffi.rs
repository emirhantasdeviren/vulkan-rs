#![allow(dead_code, non_camel_case_types)]

use std::ffi::c_void;
use std::marker::{PhantomData, PhantomPinned};

pub type Bool32 = u32;
pub type DeviceAddress = u64;
pub type DeviceSize = u64;
pub type Flags = u32;

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
pub struct OpaqueDevice {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct OpaqueQueue {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
#[cfg(target_pointer_width = "64")]
pub struct OpaqueSemaphore {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct OpaqueCommandBuffer {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
#[cfg(target_pointer_width = "64")]
pub struct OpaqueCommandPool {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
#[cfg(target_pointer_width = "64")]
pub struct OpaqueSurfaceKHR {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
#[cfg(target_pointer_width = "64")]
pub struct OpaqueSwapchainKHR {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Result {
    Success = 0,
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,
    OutOfHostMemory = -1,
    OutOfDeviceMemory = -2,
    InitializationFailed = -3,
    DeviceLost = -4,
    MemoryaMapFailed = -5,
    LayerNotPresent = -6,
    ExtensionNotPresent = -7,
    FeatureNotPresent = -8,
    IncompatibleDriver = -9,
    TooManyObjects = -10,
}

#[derive(Debug)]
#[repr(i32)]
pub enum StructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    DeviceQueueCreateInfo = 2,
    DeviceCreateInfo = 3,
    SemaphoreCreateInfo = 9,
    CommandPoolCreateInfo = 39,
    CommandBufferAllocateInfo = 40,
    XlibSurfaceCreateInfoKHR = 1000004000,
    XcbSurfaceCreateInfoKHR = 1000005000,
    Win32SurfaceCreateInfoKHR = 1000009000,
}

#[repr(i32)]
enum SystemAllocationScope {
    Command = 0,
    Object = 1,
    Cache = 2,
    Device = 3,
    Instance = 4,
    MaxEnum = 0x7FFFFFFF,
}

#[repr(i32)]
enum InternalAllocationType {
    Executable = 0,
}

#[repr(i32)]
pub enum PhysicalDeviceType {
    Other = 0,
    IntegratedGpu = 1,
    DiscreteGpu = 2,
    VirtualGpu = 3,
    Cpu = 4,
    MaxEnum = 0x7FFFFFFF,
}

#[repr(i32)]
pub enum CommandBufferLevel {
    Primary = 0,
    Secondary = 1,
}

#[repr(C)]
pub struct Extent2D {
    width: u32,
    height: u32,
}

#[repr(C)]
pub struct Extent3D {
    width: u32,
    height: u32,
    depth: u32,
}

type PFN_vkAllocationFunction = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut c_void;
type PFN_vkFreeFunction =
    unsafe extern "system" fn(p_user_data: *mut c_void, p_memory: *mut c_void);
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

pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn(
    instance: *mut OpaqueInstance,
    p_name: *const i8,
) -> Option<PFN_vkVoidFunction>;
pub type PFN_vkGetDeviceProcAddr = unsafe extern "system" fn(
    device: *mut OpaqueDevice,
    p_name: *const i8,
) -> Option<PFN_vkVoidFunction>;
pub type PFN_vkEnumerateInstanceVersion = unsafe extern "system" fn(*mut u32) -> self::Result;
pub type PFN_vkCreateInstance = unsafe extern "system" fn(
    p_create_info: *const InstanceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_instance: *mut *mut OpaqueInstance,
) -> self::Result;
pub type PFN_vkDestroyInstance = unsafe extern "system" fn(
    instance: *mut OpaqueInstance,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(
    instance: *mut OpaqueInstance,
    pPhysicalDeviceCount: *mut u32,
    pPhysicalDevices: *mut *mut OpaquePhysicalDevice,
) -> self::Result;
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(
    physical_device: *mut OpaquePhysicalDevice,
    p_properties: *mut PhysicalDeviceProperties,
);
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(
    physical_device: *mut OpaquePhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties,
);
pub type PFN_vkCreateDevice = unsafe extern "system" fn(
    physical_device: *mut OpaquePhysicalDevice,
    p_create_info: *const DeviceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_device: *mut *mut OpaqueDevice,
) -> self::Result;
pub type PFN_vkDestroyDevice =
    unsafe extern "system" fn(device: *mut OpaqueDevice, p_allocator: *const AllocationCallbacks);
pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(
    device: *mut OpaqueDevice,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut *mut OpaqueQueue,
);
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(
    device: *mut OpaqueDevice,
    create_info: *const CommandPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    #[cfg(target_pointer_width = "64")] p_command_pool: *mut *mut OpaqueCommandPool,
    #[cfg(not(target_pointer_width = "64"))] p_command_pool: *mut u64,
) -> self::Result;
pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(
    device: *mut OpaqueDevice,
    #[cfg(target_pointer_width = "64")] command_pool: *mut OpaqueCommandPool,
    #[cfg(not(target_pointer_width = "64"))] command_pool: u64,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(
    device: *mut OpaqueDevice,
    p_allocate_info: *const CommandBufferAllocateInfo,
    p_command_buffers: *mut *mut OpaqueCommandBuffer,
) -> self::Result;
pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(
    device: *mut OpaqueDevice,
    p_create_info: *const SemaphoreCreateInfo,
    p_allocator: *const AllocationCallbacks,
    #[cfg(target_pointer_width = "64")] p_semaphore: *mut *mut OpaqueSemaphore,
    #[cfg(not(target_pointer_width = "64"))] p_semaphore: *mut u64,
) -> self::Result;
pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(
    device: *mut OpaqueDevice,
    #[cfg(target_pointer_width = "64")] semaphore: *mut OpaqueSemaphore,
    #[cfg(not(target_pointer_width = "64"))] semaphore: u64,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(
    instance: *mut OpaqueInstance,
    #[cfg(target_pointer_width = "64")] surface: *mut OpaqueSurfaceKHR,
    #[cfg(not(target_pointer_width = "64"))] surface: u64,
    p_allocator: *const AllocationCallbacks,
);

type InstanceCreateFlags = Flags;
type SampleCountFlags = Flags;
pub type QueueFlags = Flags;
type DeviceQueueCreateFlags = Flags;
type DeviceCreateFlags = Flags;
type CommandPoolCreateFlags = Flags;
type SemaphoreCreateFlags = Flags;
type SwapchainCreateFlagsKhr = Flags;
type ImageUsageFlags = Flags;
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
type XlibSurfaceCreateFlagsKHR = Flags;
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
type XcbSurfaceCreateFlagsKHR = Flags;
#[cfg(target_os = "windows")]
type Win32SurfaceCreateFlagsKHR = Flags;

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

#[repr(C)]
pub struct PhysicalDeviceFeatures {
    robust_buffer_access: Bool32,
    full_draw_index_uint32: Bool32,
    image_cube_array: Bool32,
    independent_blend: Bool32,
    geometry_shader: Bool32,
    tessellation_shader: Bool32,
    sample_rate_shading: Bool32,
    dual_src_blend: Bool32,
    logic_op: Bool32,
    multi_draw_indirect: Bool32,
    draw_indirect_first_instance: Bool32,
    depth_clamp: Bool32,
    depth_bias_clamp: Bool32,
    fill_mode_non_solid: Bool32,
    depth_bounds: Bool32,
    wide_lines: Bool32,
    large_points: Bool32,
    alpha_to_one: Bool32,
    multi_viewport: Bool32,
    sampler_anisotropy: Bool32,
    texture_compression_etc2: Bool32,
    texture_compression_astc_ldr: Bool32,
    texture_compression_bc: Bool32,
    occlusion_query_precise: Bool32,
    pipeline_statistics_query: Bool32,
    vertex_pipeline_stores_and_atomics: Bool32,
    fragment_stores_and_atomics: Bool32,
    shader_tessellation_and_geometry_point_size: Bool32,
    shader_image_gather_extended: Bool32,
    shader_storage_image_extended_formats: Bool32,
    shader_storage_image_multisample: Bool32,
    shader_storage_image_read_without_format: Bool32,
    shader_storage_image_write_without_format: Bool32,
    shader_uniform_buffer_array_dynamic_indexing: Bool32,
    shader_sampled_image_array_dynamic_indexing: Bool32,
    shader_storage_buffer_array_dynamic_indexing: Bool32,
    shader_storage_image_array_dynamic_indexing: Bool32,
    shader_clip_distance: Bool32,
    shader_cull_distance: Bool32,
    shader_float64: Bool32,
    shader_int64: Bool32,
    shader_int16: Bool32,
    shader_resource_residency: Bool32,
    shader_resource_min_lod: Bool32,
    sparse_binding: Bool32,
    sparse_residency_buffer: Bool32,
    sparse_residency_image_2d: Bool32,
    sparse_residency_image_3d: Bool32,
    sparse_residency2_samples: Bool32,
    sparse_residency4_samples: Bool32,
    sparse_residency8_samples: Bool32,
    sparse_residency16_samples: Bool32,
    sparse_residency_aliased: Bool32,
    variable_multisample_rate: Bool32,
    inherited_queries: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceLimits {
    max_image_dimension_1d: u32,
    max_image_dimension_2d: u32,
    max_image_dimension_3d: u32,
    max_image_dimension_cube: u32,
    max_image_array_layers: u32,
    max_texel_buffer_elements: u32,
    max_uniform_buffer_range: u32,
    max_storage_buffer_range: u32,
    max_push_constants_size: u32,
    max_memory_allocation_count: u32,
    max_sampler_allocation_count: u32,
    buffer_image_granularity: DeviceSize,
    sparse_address_space_size: DeviceSize,
    max_bound_descriptor_sets: u32,
    max_per_stage_descriptor_samplers: u32,
    max_per_stage_descriptor_uniform_buffers: u32,
    max_per_stage_descriptor_storage_buffers: u32,
    max_per_stage_descriptor_sampled_images: u32,
    max_per_stage_descriptor_storage_images: u32,
    max_per_stage_descriptor_input_attachments: u32,
    max_per_stage_resources: u32,
    max_descriptor_set_samplers: u32,
    max_descriptor_set_uniform_buffers: u32,
    max_descriptor_set_uniform_buffers_dynamic: u32,
    max_descriptor_set_storage_buffers: u32,
    max_descriptor_set_storage_buffers_dynamic: u32,
    max_descriptor_set_sampled_images: u32,
    max_descriptor_set_storage_images: u32,
    max_descriptor_set_input_attachments: u32,
    max_vertex_input_attributes: u32,
    max_vertex_input_bindings: u32,
    max_vertex_input_attribute_offset: u32,
    max_vertex_input_binding_stride: u32,
    max_vertex_output_components: u32,
    max_tessellation_generation_level: u32,
    max_tessellation_patch_size: u32,
    max_tessellation_control_per_vertex_input_components: u32,
    max_tessellation_control_per_vertex_output_components: u32,
    max_tessellation_control_per_patch_output_components: u32,
    max_tessellation_control_total_output_components: u32,
    max_tessellation_evaluation_input_components: u32,
    max_tessellation_evaluation_output_components: u32,
    max_geometry_shader_invocations: u32,
    max_geometry_input_components: u32,
    max_geometry_output_components: u32,
    max_geometry_output_vertices: u32,
    max_geometry_total_output_components: u32,
    max_fragment_input_components: u32,
    max_fragment_output_attachments: u32,
    max_fragment_dual_src_attachments: u32,
    max_fragment_combined_output_resources: u32,
    max_compute_shared_memory_size: u32,
    max_compute_work_group_count: [u32; 3],
    max_compute_work_group_invocations: u32,
    max_compute_work_group_size: [u32; 3],
    sub_pixel_precision_bits: u32,
    sub_texel_precision_bits: u32,
    mipmap_precision_bits: u32,
    max_draw_indexed_index_value: u32,
    max_draw_indirect_count: u32,
    max_sampler_lod_bias: f32,
    max_sampler_anisotropy: f32,
    max_viewports: u32,
    max_viewport_dimensions: [u32; 2],
    viewport_bounds_range: [f32; 2],
    viewport_sub_pixel_bits: u32,
    min_memory_map_alignment: usize,
    min_texel_buffer_offset_alignment: DeviceSize,
    min_uniform_buffer_offset_alignment: DeviceSize,
    min_storage_buffer_offset_alignment: DeviceSize,
    min_texel_offset: i32,
    max_texel_offset: u32,
    min_texel_gather_offset: i32,
    max_texel_gather_offset: u32,
    min_interpolation_offset: f32,
    max_interpolation_offset: f32,
    sub_pixel_interpolation_offset_bits: u32,
    max_framebuffer_width: u32,
    max_framebuffer_height: u32,
    max_framebuffer_layers: u32,
    framebuffer_color_sample_counts: SampleCountFlags,
    framebuffer_depth_sample_counts: SampleCountFlags,
    framebuffer_stencil_sample_counts: SampleCountFlags,
    framebuffer_no_attachments_sample_counts: SampleCountFlags,
    max_color_attachments: u32,
    sampled_image_color_sample_counts: SampleCountFlags,
    sampled_image_integer_sample_counts: SampleCountFlags,
    sampled_image_depth_sample_counts: SampleCountFlags,
    sampled_image_stencil_sample_counts: SampleCountFlags,
    storage_image_sample_counts: SampleCountFlags,
    max_sample_mask_words: u32,
    timestamp_compute_and_graphics: Bool32,
    timestamp_period: f32,
    max_clip_distances: u32,
    max_cull_distances: u32,
    max_combined_clip_and_cull_distances: u32,
    discrete_queue_priorities: u32,
    point_size_range: [f32; 2],
    line_width_range: [f32; 2],
    point_size_granularity: f32,
    line_width_granularity: f32,
    strict_lines: Bool32,
    standard_sample_locations: Bool32,
    optimal_buffer_copy_offset_alignment: DeviceSize,
    optimal_buffer_copy_row_pitch_alignment: DeviceSize,
    non_coherent_atom_size: DeviceSize,
}

#[repr(C)]
pub struct PhysicalDeviceSparseProperties {
    residency_standard_2d_block_shape: Bool32,
    residency_standard_2d_multisample_block_shape: Bool32,
    residency_standard_3d_block_shape: Bool32,
    residency_aligned_mip_size: Bool32,
    residency_non_resident_strict: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    driver_version: u32,
    vendor_id: u32,
    device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: [i8; 256],
    pipeline_cache_uuid: [u8; 16],
    limits: PhysicalDeviceLimits,
    sparse_properties: PhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    timestamp_valid_bits: u32,
    min_image_transfer_granularity: Extent3D,
}

#[repr(C)]
pub struct DeviceQueueCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const f32,
}

#[repr(C)]
pub struct DeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const DeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const i8,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const i8,
    pub p_enabled_features: *const PhysicalDeviceFeatures,
}

#[repr(C)]
pub struct CommandPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
}

#[repr(C)]
pub struct CommandBufferAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    #[cfg(target_pointer_width = "64")]
    pub command_pool: *mut OpaqueCommandPool,
    #[cfg(not(target_pointer_width = "64"))]
    pub command_pool: u64,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}

#[repr(C)]
pub struct SemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreCreateFlags,
}

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
#[repr(C)]
pub struct xcb_connection_t {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub type xcb_window_t = u32;

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
#[repr(C)]
pub struct XcbSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: XcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_connection_t,
    pub window: xcb_window_t,
}

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(
    instance: *mut OpaqueInstance,
    p_create_info: *const XcbSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    #[cfg(target_pointer_width = "64")] p_surface: *mut *mut OpaqueSurfaceKHR,
    #[cfg(not(target_pointer_width = "64"))] p_surface: *mut u64,
) -> self::Result;

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
#[repr(C)]
pub struct Display {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub type Window = u64;

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
#[repr(C)]
pub struct XlibSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: XlibSurfaceCreateFlagsKHR,
    pub dpy: *mut Display,
    pub window: Window,
}

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
    instance: *mut OpaqueInstance,
    p_create_info: *const XlibSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    #[cfg(target_pointer_width = "64")] p_surface: *mut *mut OpaqueSurfaceKHR,
    #[cfg(not(target_pointer_width = "64"))] p_surface: *mut u64,
) -> self::Result;

#[cfg(target_os = "windows")]
#[repr(C)]
pub struct HINSTANCE__ {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[cfg(target_os = "windows")]
#[repr(C)]
pub struct HWND__ {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[cfg(target_os = "windows")]
#[repr(C)]
pub struct Win32SurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: Win32SurfaceCreateFlagsKHR,
    pub hinstance: *mut HINSTANCE__,
    pub hwnd: *mut HWND__,
}

#[cfg(target_os = "windows")]
pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
    instance: *mut OpaqueInstance,
    p_create_info: *const Win32SurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    #[cfg(target_pointer_width = "64")] p_surface: *mut *mut OpaqueSurfaceKHR,
    #[cfg(not(target_pointer_width = "64"))] p_surface: *mut u64,
) -> self::Result;

#[repr(i32)]
pub enum Format {
    Undefined = 0,
    R4g4UnormPack8 = 1,
    R4g4b4a4UnormPack16 = 2,
    B4g4r4a4UnormPack16 = 3,
    R5g6b5UnormPack16 = 4,
    B5g6r5UnormPack16 = 5,
    R5g5b5a1UnormPack16 = 6,
    B5g5r5a1UnormPack16 = 7,
    A1r5g5b5UnormPack16 = 8,
    R8Unorm = 9,
    R8Snorm = 10,
    R8Uscaled = 11,
    R8Sscaled = 12,
    R8Uint = 13,
    R8Sint = 14,
    R8Srgb = 15,
    R8g8Unorm = 16,
    R8g8Snorm = 17,
    R8g8Uscaled = 18,
    R8g8Sscaled = 19,
    R8g8Uint = 20,
    R8g8Sint = 21,
    R8g8Srgb = 22,
    R8g8b8Unorm = 23,
    R8g8b8Snorm = 24,
    R8g8b8Uscaled = 25,
    R8g8b8Sscaled = 26,
    R8g8b8Uint = 27,
    R8g8b8Sint = 28,
    R8g8b8Srgb = 29,
    B8g8r8Unorm = 30,
    B8g8r8Snorm = 31,
    B8g8r8Uscaled = 32,
    B8g8r8Sscaled = 33,
    B8g8r8Uint = 34,
    B8g8r8Sint = 35,
    B8g8r8Srgb = 36,
    R8g8b8a8Unorm = 37,
    R8g8b8a8Snorm = 38,
    R8g8b8a8Uscaled = 39,
    R8g8b8a8Sscaled = 40,
    R8g8b8a8Uint = 41,
    R8g8b8a8Sint = 42,
    R8g8b8a8Srgb = 43,
    B8g8r8a8Unorm = 44,
    B8g8r8a8Snorm = 45,
    B8g8r8a8Uscaled = 46,
    B8g8r8a8Sscaled = 47,
    B8g8r8a8Uint = 48,
    B8g8r8a8Sint = 49,
    B8g8r8a8Srgb = 50,
    A8b8g8r8UnormPack32 = 51,
    A8b8g8r8SnormPack32 = 52,
    A8b8g8r8UscaledPack32 = 53,
    A8b8g8r8SscaledPack32 = 54,
    A8b8g8r8UintPack32 = 55,
    A8b8g8r8SintPack32 = 56,
    A8b8g8r8SrgbPack32 = 57,
    A2r10g10b10UnormPack32 = 58,
    A2r10g10b10SnormPack32 = 59,
    A2r10g10b10UscaledPack32 = 60,
    A2r10g10b10SscaledPack32 = 61,
    A2r10g10b10UintPack32 = 62,
    A2r10g10b10SintPack32 = 63,
    A2b10g10r10UnormPack32 = 64,
    A2b10g10r10SnormPack32 = 65,
    A2b10g10r10UscaledPack32 = 66,
    A2b10g10r10SscaledPack32 = 67,
    A2b10g10r10UintPack32 = 68,
    A2b10g10r10SintPack32 = 69,
    R16Unorm = 70,
    R16Snorm = 71,
    R16Uscaled = 72,
    R16Sscaled = 73,
    R16Uint = 74,
    R16Sint = 75,
    R16Sfloat = 76,
    R16g16Unorm = 77,
    R16g16Snorm = 78,
    R16g16Uscaled = 79,
    R16g16Sscaled = 80,
    R16g16Uint = 81,
    R16g16Sint = 82,
    R16g16Sfloat = 83,
    R16g16b16Unorm = 84,
    R16g16b16Snorm = 85,
    R16g16b16Uscaled = 86,
    R16g16b16Sscaled = 87,
    R16g16b16Uint = 88,
    R16g16b16Sint = 89,
    R16g16b16Sfloat = 90,
    R16g16b16a16Unorm = 91,
    R16g16b16a16Snorm = 92,
    R16g16b16a16Uscaled = 93,
    R16g16b16a16Sscaled = 94,
    R16g16b16a16Uint = 95,
    R16g16b16a16Sint = 96,
    R16g16b16a16Sfloat = 97,
    R32Uint = 98,
    R32Sint = 99,
    R32Sfloat = 100,
    R32g32Uint = 101,
    R32g32Sint = 102,
    R32g32Sfloat = 103,
    R32g32b32Uint = 104,
    R32g32b32Sint = 105,
    R32g32b32Sfloat = 106,
    R32g32b32a32Uint = 107,
    R32g32b32a32Sint = 108,
    R32g32b32a32Sfloat = 109,
    R64Uint = 110,
    R64Sint = 111,
    R64Sfloat = 112,
    R64g64Uint = 113,
    R64g64Sint = 114,
    R64g64Sfloat = 115,
    R64g64b64Uint = 116,
    R64g64b64Sint = 117,
    R64g64b64Sfloat = 118,
    R64g64b64a64Uint = 119,
    R64g64b64a64Sint = 120,
    R64g64b64a64Sfloat = 121,
    B10g11r11UfloatPack32 = 122,
    E5b9g9r9UfloatPack32 = 123,
    D16Unorm = 124,
    X8D24UnormPack32 = 125,
    D32Sfloat = 126,
    S8Uint = 127,
    D16UnormS8Uint = 128,
    D24UnormS8Uint = 129,
    D32SfloatS8Uint = 130,
    Bc1RgbUnormBlock = 131,
    Bc1RgbSrgbBlock = 132,
    Bc1RgbaUnormBlock = 133,
    Bc1RgbaSrgbBlock = 134,
    Bc2UnormBlock = 135,
    Bc2SrgbBlock = 136,
    Bc3UnormBlock = 137,
    Bc3SrgbBlock = 138,
    Bc4UnormBlock = 139,
    Bc4SnormBlock = 140,
    Bc5UnormBlock = 141,
    Bc5SnormBlock = 142,
    Bc6hUfloatBlock = 143,
    Bc6hSfloatBlock = 144,
    Bc7UnormBlock = 145,
    Bc7SrgbBlock = 146,
    Etc2R8g8b8UnormBlock = 147,
    Etc2R8g8b8SrgbBlock = 148,
    Etc2R8g8b8a1UnormBlock = 149,
    Etc2R8g8b8a1SrgbBlock = 150,
    Etc2R8g8b8a8UnormBlock = 151,
    Etc2R8g8b8a8SrgbBlock = 152,
    EacR11UnormBlock = 153,
    EacR11SnormBlock = 154,
    EacR11g11UnormBlock = 155,
    EacR11g11SnormBlock = 156,
    Astc4x4UnormBlock = 157,
    Astc4x4SrgbBlock = 158,
    Astc5x4UnormBlock = 159,
    Astc5x4SrgbBlock = 160,
    Astc5x5UnormBlock = 161,
    Astc5x5SrgbBlock = 162,
    Astc6x5UnormBlock = 163,
    Astc6x5SrgbBlock = 164,
    Astc6x6UnormBlock = 165,
    Astc6x6SrgbBlock = 166,
    Astc8x5UnormBlock = 167,
    Astc8x5SrgbBlock = 168,
    Astc8x6UnormBlock = 169,
    Astc8x6SrgbBlock = 170,
    Astc8x8UnormBlock = 171,
    Astc8x8SrgbBlock = 172,
    Astc10x5UnormBlock = 173,
    Astc10x5SrgbBlock = 174,
    Astc10x6UnormBlock = 175,
    Astc10x6SrgbBlock = 176,
    Astc10x8UnormBlock = 177,
    Astc10x8SrgbBlock = 178,
    Astc10x10UnormBlock = 179,
    Astc10x10SrgbBlock = 180,
    Astc12x10UnormBlock = 181,
    Astc12x10SrgbBlock = 182,
    Astc12x12UnormBlock = 183,
    Astc12x12SrgbBlock = 184,
    G8b8g8r8422Unorm = 1000156000,
    B8g8r8g8422Unorm = 1000156001,
    G8B8R83plane420Unorm = 1000156002,
    G8B8r82plane420Unorm = 1000156003,
    G8B8R83plane422Unorm = 1000156004,
    G8B8r82plane422Unorm = 1000156005,
    G8B8R83plane444Unorm = 1000156006,
    R10x6UnormPack16 = 1000156007,
    R10x6g10x6Unorm2pack16 = 1000156008,
    R10x6g10x6b10x6a10x6Unorm4pack16 = 1000156009,
    G10x6b10x6g10x6r10x6422Unorm4pack16 = 1000156010,
    B10x6g10x6r10x6g10x6422Unorm4pack16 = 1000156011,
    G10x6B10x6R10x63plane420Unorm3pack16 = 1000156012,
    G10x6B10x6r10x62plane420Unorm3pack16 = 1000156013,
    G10x6B10x6R10x63plane422Unorm3pack16 = 1000156014,
    G10x6B10x6r10x62plane422Unorm3pack16 = 1000156015,
    G10x6B10x6R10x63plane444Unorm3pack16 = 1000156016,
    R12x4UnormPack16 = 1000156017,
    R12x4g12x4Unorm2pack16 = 1000156018,
    R12x4g12x4b12x4a12x4Unorm4pack16 = 1000156019,
    G12x4b12x4g12x4r12x4422Unorm4pack16 = 1000156020,
    B12x4g12x4r12x4g12x4422Unorm4pack16 = 1000156021,
    G12x4B12x4R12x43plane420Unorm3pack16 = 1000156022,
    G12x4B12x4r12x42plane420Unorm3pack16 = 1000156023,
    G12x4B12x4R12x43plane422Unorm3pack16 = 1000156024,
    G12x4B12x4r12x42plane422Unorm3pack16 = 1000156025,
    G12x4B12x4R12x43plane444Unorm3pack16 = 1000156026,
    G16b16g16r16422Unorm = 1000156027,
    B16g16r16g16422Unorm = 1000156028,
    G16B16R163plane420Unorm = 1000156029,
    G16B16r162plane420Unorm = 1000156030,
    G16B16R163plane422Unorm = 1000156031,
    G16B16r162plane422Unorm = 1000156032,
    G16B16R163plane444Unorm = 1000156033,
    Pvrtc12bppUnormBlockImg = 1000054000,
    Pvrtc14bppUnormBlockImg = 1000054001,
    Pvrtc22bppUnormBlockImg = 1000054002,
    Pvrtc24bppUnormBlockImg = 1000054003,
    Pvrtc12bppSrgbBlockImg = 1000054004,
    Pvrtc14bppSrgbBlockImg = 1000054005,
    Pvrtc22bppSrgbBlockImg = 1000054006,
    Pvrtc24bppSrgbBlockImg = 1000054007,
    Astc4x4SfloatBlockExt = 1000066000,
    Astc5x4SfloatBlockExt = 1000066001,
    Astc5x5SfloatBlockExt = 1000066002,
    Astc6x5SfloatBlockExt = 1000066003,
    Astc6x6SfloatBlockExt = 1000066004,
    Astc8x5SfloatBlockExt = 1000066005,
    Astc8x6SfloatBlockExt = 1000066006,
    Astc8x8SfloatBlockExt = 1000066007,
    Astc10x5SfloatBlockExt = 1000066008,
    Astc10x6SfloatBlockExt = 1000066009,
    Astc10x8SfloatBlockExt = 1000066010,
    Astc10x10SfloatBlockExt = 1000066011,
    Astc12x10SfloatBlockExt = 1000066012,
    Astc12x12SfloatBlockExt = 1000066013,
    G8B8r82plane444UnormExt = 1000330000,
    G10x6B10x6r10x62plane444Unorm3pack16Ext = 1000330001,
    G12x4B12x4r12x42plane444Unorm3pack16Ext = 1000330002,
    G16B16r162plane444UnormExt = 1000330003,
    A4r4g4b4UnormPack16Ext = 1000340000,
    A4b4g4r4UnormPack16Ext = 1000340001,
}

#[repr(i32)]
pub enum ColorSpaceKhr {
    SrgbNonlinearKhr = 0,
    DisplayP3NonlinearExt = 1000104001,
    ExtendedSrgbLinearExt = 1000104002,
    DisplayP3LinearExt = 1000104003,
    DciP3NonlinearExt = 1000104004,
    Bt709LinearExt = 1000104005,
    Bt709NonlinearExt = 1000104006,
    Bt2020LinearExt = 1000104007,
    Hdr10St2084Ext = 1000104008,
    DolbyvisionExt = 1000104009,
    Hdr10HlgExt = 1000104010,
    AdobergbLinearExt = 1000104011,
    AdobergbNonlinearExt = 1000104012,
    PassThroughExt = 1000104013,
    ExtendedSrgbNonlinearExt = 1000104014,
    DisplayNativeAmd = 1000213000,
}

#[repr(i32)]
pub enum SharingMode {
    Exclusive = 0,
    Concurrent = 1,
}

#[repr(i32)]
pub enum SurfaceTransformFlagBitsKhr {
    IdentityBitKhr = 0x00000001,
    Rotate90BitKhr = 0x00000002,
    Rotate180BitKhr = 0x00000004,
    Rotate270BitKhr = 0x00000008,
    HorizontalMirrorBitKhr = 0x00000010,
    HorizontalMirrorRotate90BitKhr = 0x00000020,
    HorizontalMirrorRotate180BitKhr = 0x00000040,
    HorizontalMirrorRotate270BitKhr = 0x00000080,
    InheritBitKhr = 0x00000100,
}

#[repr(i32)]
pub enum CompositeAlphaFlagBitsKhr {
    OpaqueBitKhr = 0x00000001,
    PreMultipliedBitKhr = 0x00000002,
    PostMultipliedBitKhr = 0x00000004,
    InheritBitKhr = 0x00000008,
}

#[repr(i32)]
pub enum PresentModeKhr {
    ImmediateKhr = 0,
    MailboxKhr = 1,
    FifoKhr = 2,
    FifoRelaxedKhr = 3,
    SharedDemandRefreshKhr = 1000111000,
    SharedContinuousRefreshKhr = 1000111001,
}

#[repr(C)]
pub struct SwapchainCreateInfoKhr {
    s_type: StructureType,
    p_next: *const c_void,
    flags: SwapchainCreateFlagsKhr,
    #[cfg(target_pointer_width = "64")]
    surface: *mut OpaqueSurfaceKHR,
    #[cfg(not(target_pointer_width = "64"))]
    surface: u64,
    min_image_count: u32,
    image_format: Format,
    image_color_space: ColorSpaceKhr,
    image_extent: Extent2D,
    image_array_layers: u32,
    image_usage: ImageUsageFlags,
    image_sharing_mode: SharingMode,
    queue_family_index_count: u32,
    queue_family_indices: *const u32,
    pre_transform: SurfaceTransformFlagBitsKhr,
    composite_alpha: CompositeAlphaFlagBitsKhr,
    present_mode: PresentModeKhr,
    clipped: Bool32,
    #[cfg(target_pointer_width = "64")]
    old_swapchain: *mut OpaqueSwapchainKHR,
    #[cfg(not(target_pointer_width = "64"))]
    old_swapchain: u64,
}
