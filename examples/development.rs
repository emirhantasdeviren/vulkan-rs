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
    let instance = vk::Instance::builder()
        .with_application_info(&app_info)
        .with_extensions(extensions)
        .build()
        .unwrap();

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

    let surface_format = physical_device
        .get_surface_formats_khr(&surface)
        .unwrap()
        .unwrap()
        .into_iter()
        .find(|available_format| {
            available_format.format == vk::Format::B8g8r8a8Srgb
                && available_format.color_space == vk::ColorSpaceKhr::SrgbNonlinearKhr
        })
        .unwrap();

    let present_mode = physical_device
        .get_surface_present_modes_khr(&surface)
        .unwrap()
        .unwrap()
        .into_iter()
        .find(|available_present_mode| *available_present_mode == vk::PresentModeKhr::MailboxKhr)
        .unwrap_or(vk::PresentModeKhr::FifoKhr);

    let mut image_count = surface_capabilities.min_image_count + 1;

    if surface_capabilities.max_image_count > 0
        && image_count > surface_capabilities.max_image_count
    {
        image_count = surface_capabilities.max_image_count;
    }

    let swapchain = vk::SwapchainBuilderKhr::new(
        &surface,
        image_count,
        surface_format.format,
        surface_format.color_space,
        surface_capabilities.current_extent,
        vk::ImageUsageFlagsBuilder::new()
            .color_attachment(true)
            .build(),
        vk::SharingMode::Exclusive,
        surface_capabilities.current_transform,
        vk::CompositeAlphaKhr::OpaqueKhr,
        present_mode,
        true,
    )
    .build(&device)
    .unwrap();

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
