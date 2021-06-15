use vulkan_rs::core as vk;

fn main() {
    let app_info = vk::ApplicationInfo {
        application_name: Some(String::from("Hello Triangle")),
        application_version: vk::ApiVersion::new(0, 0, 1, 0),
        engine_name: None,
        engine_version: vk::ApiVersion::default(),
        api_version: vk::ApiVersion::V1_0,
    };
    let instance = vk::Instance::new(Some(&app_info), None, None);

    if let Some(v) = instance.version() {
        println!("Vulkan API {}", v);
    } else {
        println!("Vulkan API 1.0.0");
    }

    let physical_devices = instance.enumerate_physical_devices();

    for physical_device in physical_devices.iter() {
        let properties = physical_device.properties();

        print!("Device name: {}, ", properties.device_name);
        match properties.device_type {
            vk::PhysicalDeviceType::IntegratedGpu => println!("Device type: INTEGRATED_GPU"),
            vk::PhysicalDeviceType::DiscreteGpu => println!("Device type: DISCRETE_GPU"),
            _ => println!("OTHER"),
        };
    }
}
