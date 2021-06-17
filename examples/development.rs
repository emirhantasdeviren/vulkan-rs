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
        let queue_family_properties = physical_device.queue_family_properties();

        println!("\nDevice API version: {}", properties.api_version);
        print!("Device name: {}, ", properties.device_name);
        match properties.device_type {
            vk::PhysicalDeviceType::IntegratedGpu => println!("Device type: INTEGRATED_GPU\n"),
            vk::PhysicalDeviceType::DiscreteGpu => println!("Device type: DISCRETE_GPU\n"),
            _ => println!("OTHER"),
        };

        println!("Physical Device Queue Families:");
        for (i, queue_family) in queue_family_properties.iter().enumerate() {
            print!("[{}]:\t", i);
            if queue_family.graphics() {
                print!("GRAPHICS | ");
            }
            if queue_family.compute() {
                print!("COMPUTE | ");
            }
            if queue_family.transfer() {
                print!("TRANSFER | ");
            }
            if queue_family.sparse_binding() {
                print!("SPARSE BINDING");
            }
            println!(
                "\n\tQueue Count: {}\n",
                queue_family.queue_count,
            );
        }
    }
}
