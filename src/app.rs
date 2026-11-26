use anyhow::{anyhow, Result};
use ash::{vk, Device, Entry, Instance};
use std::ffi::CStr;

pub const MAX_FRAMES_IN_FLIGHT: usize = 2;
pub const CLEAR_COLOR: [f32; 4] = [0.015, 0.014, 0.020, 1.0]; // почти-чёрный с холодком

pub struct App {
    // Instance / surface
    pub entry: Entry,
    pub instance: Instance,
    pub surface: vk::SurfaceKHR,
    pub surface_fn: ash::extensions::khr::Surface,

    // Device / queue
    pub pdevice: vk::PhysicalDevice,
    pub device: Device,
    pub queue: vk::Queue,
    pub queue_family: u32,

    // Swapchain
    pub swapchain_fn: ash::extensions::khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_format: vk::Format,
    pub swapchain_extent: vk::Extent2D,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_views: Vec<vk::ImageView>,

    // Rendering
    pub render_pass: vk::RenderPass,
    pub framebuffers: Vec<vk::Framebuffer>,

    // Commands
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,

    // Sync (per frame-in-flight)
    pub image_available: Vec<vk::Semaphore>,
    pub render_finished: Vec<vk::Semaphore>,
    pub in_flight_fences: Vec<vk::Fence>,
    pub current_frame: usize,

    pub window: winit::window::Window,
}
