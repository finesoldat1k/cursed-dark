impl App {
    fn pick_surface_format(&self, formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
        // Идеал: sRGB + UNORM (не SRGB_NONLINEAR у swapchain — иначе двойная гамма-коррекция
        // и мы потеряем "плёночный" вид, который потом накрутим пост-процессом).
        // Для максимально "PS1"-ощущения не бойся взять UNORM.
        for f in formats {
            if f.format == vk::Format::B8G8R8A8_UNORM
                && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return *f;
            }
        }
        formats[0]
    }

    fn pick_present_mode(&self, modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
        // FIFO — стабильный vsync, никакого тиринга. Для атмосферного хоррора
        // тиринг = дешёвка, а не стиль. MAILBOX/Immediate оставь для отладки.
        if modes.contains(&vk::PresentModeKHR::MAILBOX) {
            // можно MAILBOX, если есть, но FIFO безопаснее
            return vk::PresentModeKHR::FIFO;
        }
        vk::PresentModeKHR::FIFO
    }

    fn pick_extent(&self, caps: &vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
        if caps.current_extent.width != u32::MAX {
            return caps.current_extent;
        }
        let s = self.window.inner_size();
        vk::Extent2D {
            width: s.width.clamp(caps.min_image_extent.width, caps.max_image_extent.width),
            height: s.height.clamp(caps.min_image_extent.height, caps.max_image_extent.height),
        }
    }

    fn create_swapchain(&mut self) -> Result<()> {
        let caps = unsafe {
            self.surface_fn.get_physical_device_surface_capabilities(self.pdevice, self.surface)?
        };
        let formats = unsafe {
            self.surface_fn.get_physical_device_surface_formats(self.pdevice, self.surface)?
        };
        let modes = unsafe {
            self.surface_fn.get_physical_device_surface_present_modes(self.pdevice, self.surface)?
        };

        let format = self.pick_surface_format(&formats);
        let present_mode = self.pick_present_mode(&modes);
        let extent = self.pick_extent(&caps);

        if extent.width == 0 || extent.height == 0 {
            return Err(anyhow!("Swapchain extent = 0 (окно свёрнуто?)"));
        }

        // Минимум +1 к image_count — иначе драйвер может ругаться при первом кадре.
        let mut image_count = caps.min_image_count + 1;
        if caps.max_image_count > 0 && image_count > caps.max_image_count {
            image_count = caps.max_image_count;
        }

        let composite = if caps.supported_composite_alpha.contains(
            vk::CompositeAlphaFlagsKHR::OPAQUE,
        ) {
            vk::CompositeAlphaFlagsKHR::OPAQUE
        } else {
            caps.supported_composite_alpha.iter().next().unwrap()
        };

        let info = vk::SwapchainCreateInfoKHR::default()
            .surface(self.surface)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(caps.current_transform)
            .composite_alpha(composite)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(vk::SwapchainKHR::null());

        let swapchain = unsafe { self.swapchain_fn.create_swapchain(&info, None)? };
        let images = unsafe { self.swapchain_fn.get_swapchain_images(swapchain)? };

        self.swapchain = swapchain;
        self.swapchain_format = format.format;
        self.swapchain_extent = extent;
        self.swapchain_images = images;
        Ok(())
    }
}