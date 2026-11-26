use anyhow::{anyhow, Result};
use ash::vk;
use std::ffi::CStr;

fn main() -> Result<()> {
    // 1. Окно
    let event_loop = winit::event_loop::EventLoop::new()?;
    let window = winit::window::WindowBuilder::new()
        .with_title("horror")
        .with_inner_size(winit::dpi::LogicalSize::new(960, 540))
        .build(&event_loop)?;

    // 2. Instance
    let entry = unsafe { ash::Entry::load()? };
    let app_name = CStr::from_bytes_with_nul(b"horror\0")?;
    let app_info = vk::ApplicationInfo::builder()
        .application_name(app_name)
        .application_version(vk::make_api_version(0, 1, 0, 0))
        .engine_name(app_name)
        .engine_version(vk::make_api_version(0, 1, 0, 0))
        .api_version(vk::make_api_version(0, 1, 2, 0));

    let surface_exts = ash_window::enumerate_required_extensions(
        window.display_handle()?.as_raw(),
    )?;
    let mut ext_ptrs: Vec<*const i8> = surface_exts.iter().map(|e| e.as_ptr()).collect();
    ext_ptrs.push(ash::extensions::khr::Surface::name().as_ptr());

    let layers = [CStr::from_bytes_with_nul(b"VK_LAYER_KHRONOS_validation\0")?.as_ptr()];

    let instance = unsafe {
        entry.create_instance(
            &vk::InstanceCreateInfo::builder()
                .application_info(&app_info)
                .enabled_extension_names(&ext_ptrs)
                .enabled_layer_names(&layers)
                .build(),
            None,
        )?
    };

    let surface = unsafe {
        ash_window::create_surface(&entry, &instance, &window, None)?
    };

    // 3. Физическое устройство
    let pdevices = unsafe { instance.enumerate_physical_devices()? };
    let pdevice = pdevices.iter().copied()
        .find(|&p| {
            let props = unsafe { instance.get_physical_device_properties(p) };
            props.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
        })
        .ok_or_else(|| anyhow!("Нет подходящей GPU"))?;

    // 4. Семейство очередей: graphics + present
    let qprops = unsafe { instance.get_physical_device_queue_family_properties(pdevice) };
    let qf = qprops.iter().enumerate().position(|(i, q)| {
        q.queue_flags.contains(vk::QueueFlags::GRAPHICS) && unsafe {
            instance.get_physical_device_surface_support(pdevice, i as u32, surface).unwrap_or(false)
        }
    }).ok_or_else(|| anyhow!("Нет очереди с графикой"))? as u32;

    // 5. Логическое устройство
    let priorities = [1.0f32];
    let qinfo = [vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(qf)
        .queue_priorities(&priorities)
        .build()];
    let dev_exts = [ash::extensions::khr::Swapchain::name().as_ptr()];
    let features = vk::PhysicalDeviceFeatures::builder();
    let device = unsafe {
        instance.create_device(
            pdevice,
            &vk::DeviceCreateInfo::builder()
                .queue_create_infos(&qinfo)
                .enabled_extension_names(&dev_exts)
                .enabled_features(&features)
                .build(),
            None,
        )?
    };
    let _queue = unsafe { device.get_device_queue(qf, 0) };

    println!("Vulkan инициализирован. Следующий шаг — swapchain и render pass.");

    // 6. Event loop (пока пустой)
    event_loop.run(move |event, elwt| {
        if let winit::event::Event::WindowEvent { event, .. } = event {
            if let winit::event::WindowEvent::CloseRequested = event {
                elwt.exit();
            }
        }
    })?;

    unsafe { device.destroy_device(None); instance.destroy_instance(None); }
    Ok(())
}
