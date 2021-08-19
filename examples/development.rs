use vulkan_rs::core as vk;

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

fn main() {
    let app_info = vk::ApplicationInfo {
        application_name: Some(String::from("Hello Triangle")),
        application_version: vk::ApiVersion::new(0, 0, 1, 0),
        engine_name: None,
        engine_version: Default::default(),
        api_version: vk::ApiVersion::V1_0,
    };
    let extensions = &[
        vk::KHR_SURFACE_EXTENSION_NAME,
        vk::KHR_WIN32_SURFACE_EXTENSION_NAME,
    ];
    let instance = vk::Instance::new(Some(&app_info), None, Some(extensions)).unwrap();

    if let Some(v) = instance.version() {
        println!("Vulkan API {}", v);
    } else {
        println!("Vulkan API 1.0.0");
    }

    let physical_device = instance
        .enumerate_physical_devices()
        .into_iter()
        .find(|physical_device| {
            let properties = physical_device.properties();
            println!("{}", properties.device_name);
            properties.device_type == vk::PhysicalDeviceType::DiscreteGpu
        })
        .expect("Could not find suitable GPU.");

    let device = physical_device.create_device(
        &[0],
        &[&[1.0f32]],
        Some(&[vk::KHR_SWAPCHAIN_EXTENSION_NAME]),
    );

    let _queue = device.get_device_queue(0, 0);

    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello Triangle")
        .with_resizable(false)
        .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap();

    let surface = instance.create_surface_khr(&window);
    let surface_capabilities = physical_device
        .get_surface_capabilities_khr(&surface)
        .unwrap()
        .unwrap();
    let surface_formats = physical_device
        .get_surface_formats_khr(&surface)
        .unwrap()
        .unwrap();
    let present_modes = physical_device
        .get_surface_present_modes_khr(&surface)
        .unwrap()
        .unwrap();

    for surface_format in surface_formats.iter() {
        println!("{:?}", surface_format);
    }

    for present_mode in present_modes.iter() {
        println!("{:?}", present_mode);
    }
    println!();

    event_loop.run_return(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
