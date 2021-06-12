use vulkan_rs::core::Instance;

fn main() {
    let instance = Instance::new();

    if let Some(v) = instance.version() {
        println!("Vulkan API {}", v);
    } else {
        println!("Vulkan API 1.0.0");
    }
}