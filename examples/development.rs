use vulkan_rs::core as vk;

fn main() {
    let app_info = vk::ApplicationInfo {
        application_name: Some(String::from("Hello Triangle")),
        application_version: vk::ApiVersion::new(0, 0, 1, 0),
        engine_name: None,
        engine_version: vk::ApiVersion::default(),
        api_version: vk::ApiVersion::V1_0,
    };
    let extensions = &[
        vk::KHR_SURFACE_EXTENSION_NAME,
        vk::KHR_WIN32_SURFACE_EXTENSION_NAME,
    ];
    let instance = vk::Instance::new(Some(&app_info), None, Some(extensions));

    if let Some(v) = instance.version() {
        println!("Vulkan API {}", v);
    } else {
        println!("Vulkan API 1.0.0");
    }

    let physical_device = instance
        .enumerate_physical_devices()
        .into_iter()
        .find(|p| {
            let properties = p.properties();
            properties.device_type == vk::PhysicalDeviceType::DiscreteGpu
        })
        .expect("Could not find suitable GPU.");

    let device = physical_device.create_device(&[0], &[&[1.0f32]]);
    let _queue = device.get_device_queue(0, 0);
    let command_pool = device.create_command_pool(0);
    let command_buffers = command_pool.allocate_command_buffers(1);
    let semaphore = device.create_semaphore();
}
