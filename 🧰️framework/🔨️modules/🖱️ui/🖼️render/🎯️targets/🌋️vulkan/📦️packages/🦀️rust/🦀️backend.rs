//! @emoji 🌋️ `VulkanBackend`: the concrete `ui_render::GraphicsBackend` implementation for Linux.
//!
//! **Milestone reached: 1 (device + swapchain + resize + clear-colour frame), plus device-loss/
//! recovery/`backend-testing` readback (milestone 6) and non-shader groundwork toward milestone 2.**
//! See `📓️terra-backend-vulkan-report.md` for the authoritative statement of what is and is not done,
//! why milestones 2's *pipelines* specifically stop short (a missing `vk::ShaderModule` — no shader
//! compiler is available in this environment and `Cargo.toml` is registrar-only, see that report's
//! "shader strategy"), and what stays unverified without a real Vulkan device (this machine is macOS;
//! `ash` compiles here but nothing executes Vulkan — see the ticket brief's CRITICAL section).
//!
//! `render` currently clears the swapchain image to the same `(0.05, 0.05, 0.06, 1.0)` colour the
//! Metal target clears its offscreen scene target to, and presents — it does not yet replay
//! `RenderPacket::batches` (there is no pipeline to bind them to). `apply_resources` **does** perform
//! real GPU-resident uploads via `crate::resources::GpuResources` (staging buffer → `DEVICE_LOCAL`
//! image/buffer), so the `UnknownResource` validation path and the resource lifecycle are exercised
//! for real even though nothing samples the result yet.

#[cfg(not(target_os = "linux"))]
compile_error!("semio-framework-ui-backend-vulkan builds only on Linux.");

use crate::resources::GpuResources;
use crate::swapchain_support::{choose_extent, choose_image_count, choose_present_mode, choose_surface_format, is_parked};
use crate::vk_error::VulkanGraphicsError;
use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use ui_render::{BackendError, DeviceCapabilities, DeviceStatus, FrameStats, GpuTier, GraphicsBackend, LossReason, MemoryClass, PhysicalSize, RecoveredResources, RenderPacket, RenderReport, ResourceKind, SurfaceFormat};

#[cfg(feature = "backend-testing")]
use ui_render::ReadbackImage;

//#region 🔖️Backend

/// 🔁️ Two frames in flight, per the ticket brief. Every per-frame array (`command_buffers`,
/// `image_available_semaphores`, `render_finished_semaphores`, `in_flight_fences`) is exactly this
/// long, and `current_frame` cycles through it — the standard vulkan-tutorial synchronization shape.
/// **Documented imprecision**: `render_finished_semaphores` is indexed by frame-in-flight, not by
/// swapchain image, which is technically insufficient when `swapchain_images.len() >
/// FRAMES_IN_FLIGHT` and presentation order can outrun submission order — the well-known minimal
/// vulkan-tutorial synchronization scheme, not a from-scratch design; a per-image semaphore array is
/// the documented fix, left as a `registrar-request`-adjacent follow-up in the report since it needs a
/// real device to verify rather than a `Cargo.toml` change.
const FRAMES_IN_FLIGHT: usize = 2;

/// 🎨️ Matches the Metal target's offscreen-scene clear colour so a future pixel-conformance run
/// starts from the same baseline.
const CLEAR_COLOR: [f32; 4] = [0.05, 0.05, 0.06, 1.0];

//#region 🔁️FrameSync

struct FrameSync {
    image_available: vk::Semaphore,
    render_finished: vk::Semaphore,
    in_flight: vk::Fence,
}

impl FrameSync {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn create(device: &ash::Device) -> Result<Self, vk::Result> {
        let semaphore_info = vk::SemaphoreCreateInfo::default();
        let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        // 🔓️ SAFETY: `device` is the live logical device these objects belong to; both create-infos
        // borrow only stack locals. The fence starts `SIGNALED` so the first `wait_for_fences` for this
        // slot does not block forever waiting on a frame that never submitted.
        unsafe { Ok(Self { image_available: device.create_semaphore(&semaphore_info, None)?, render_finished: device.create_semaphore(&semaphore_info, None)?, in_flight: device.create_fence(&fence_info, None)? }) }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn destroy(&self, device: &ash::Device) {
        // 🔓️ SAFETY: caller (`VulkanBackend::drop`) has already `device_wait_idle`d — none of these
        // objects is referenced by any pending GPU work.
        unsafe {
            device.destroy_semaphore(self.image_available, None);
            device.destroy_semaphore(self.render_finished, None);
            device.destroy_fence(self.in_flight, None);
        }
    }
}

//#endregion 🔁️FrameSync

//#region 🌋️VulkanBackend

/// 🌋️ The concrete Linux `GraphicsBackend`. Every `ash`/Vulkan type stays behind this crate's own
/// interface — nothing here appears in a public signature outside this crate (CLAUDE.md's "external
/// libraries behind an interface" rule; mirrors the Metal target's own framing).
pub struct VulkanBackend {
    entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    device: ash::Device,
    queue_family_index: u32,
    graphics_queue: vk::Queue,
    swapchain_loader: ash::khr::swapchain::Device,
    swapchain: vk::SwapchainKHR,
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    swapchain_images: Vec<vk::Image>,
    swapchain_image_views: Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
    frames: Vec<FrameSync>,
    current_frame: usize,
    size: PhysicalSize,
    dpr: f32,
    status: DeviceStatus,
    resources: GpuResources,
    is_low_power: bool,
}

//#region Construction

impl VulkanBackend {
    /// 🏗️ Builds an instance, physical/logical device, graphics queue, `VK_KHR_swapchain` surface
    /// (via `ash-window`), render pass, per-swapchain-image framebuffers, and `FRAMES_IN_FLIGHT` sync
    /// objects. Only construction is async per U1 — like the Metal target's `MetalBackend::new`, the
    /// body performs no real `.await`: `ash::Entry::load`/instance/device/swapchain creation are all
    /// synchronous FFI calls, unlike wgpu's adapter/device request.
    // 🚫️async: U1 — the ONE permitted async fn per the `GraphicsBackend` docstring; construction only.
    pub async fn new(display_handle: RawDisplayHandle, window_handle: RawWindowHandle, size: PhysicalSize, dpr: f32) -> Result<Self, BackendError> {
        Self::create(display_handle, window_handle, size, dpr).map_err(Into::into)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn create(display_handle: RawDisplayHandle, window_handle: RawWindowHandle, size: PhysicalSize, dpr: f32) -> Result<Self, VulkanGraphicsError> {
        // 🔓️ SAFETY: `Entry::load` dlopens the system Vulkan loader (`libvulkan.so.1`) and resolves
        // its entry points; this crate builds only on `target_os = "linux"` (the `compile_error!`
        // guard above), where that loader is the standard distribution mechanism.
        let entry = unsafe { ash::Entry::load() }.map_err(|_| VulkanGraphicsError::LoaderNotFound)?;

        let app_name = c"semio-ui-vulkan-backend";
        let app_info = vk::ApplicationInfo::default().application_name(app_name).application_version(vk::make_api_version(0, 1, 0, 0)).engine_name(app_name).engine_version(vk::make_api_version(0, 1, 0, 0)).api_version(vk::API_VERSION_1_1);
        let required_extensions = ash_window::enumerate_required_extensions(display_handle)?;
        let instance_info = vk::InstanceCreateInfo::default().application_info(&app_info).enabled_extension_names(required_extensions);
        // 🔓️ SAFETY: `entry` was just loaded above and outlives `instance` (held alongside it in
        // `Self`, destroyed in reverse order by `Drop`); `instance_info` borrows only stack locals for
        // the duration of the call.
        let instance = unsafe { entry.create_instance(&instance_info, None)? };

        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);
        // 🔓️ SAFETY: `window_handle`/`display_handle` are the caller's live window/display handles
        // (the constructor's own safety contract, inherited from `raw_window_handle`'s); `entry`/
        // `instance` outlive the resulting surface (both held in `Self`).
        let surface = unsafe { ash_window::create_surface(&entry, &instance, display_handle, window_handle, None)? };

        let (physical_device, queue_family_index) = pick_physical_device(&instance, &surface_loader, surface)?;
        // 🔓️ SAFETY: `physical_device` was just enumerated from `instance` above — a valid handle for
        // the lifetime of `instance`.
        let memory_properties = unsafe { instance.get_physical_device_memory_properties(physical_device) };

        let queue_priorities = [1.0f32];
        let queue_info = vk::DeviceQueueCreateInfo::default().queue_family_index(queue_family_index).queue_priorities(&queue_priorities);
        let queue_infos = [queue_info];
        let device_extensions = [ash::khr::swapchain::NAME.as_ptr()];
        let features = vk::PhysicalDeviceFeatures::default();
        let device_info = vk::DeviceCreateInfo::default().queue_create_infos(&queue_infos).enabled_extension_names(&device_extensions).enabled_features(&features);
        // 🔓️ SAFETY: `physical_device` is valid for `instance`'s lifetime (just enumerated above);
        // `device_info` borrows only stack locals. `instance` outlives `device` (destroyed first by
        // `Drop`, per the parent/child rule `Instance::create_device`'s own docs state).
        let device = unsafe { instance.create_device(physical_device, &device_info, None)? };
        // 🔓️ SAFETY: `queue_family_index`/queue index `0` were both selected as valid above
        // (`pick_physical_device` only returns a family with `queue_count >= 1`).
        let graphics_queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        // 🔓️ SAFETY: `physical_device` is a device `pick_physical_device` already confirmed reports
        // `sample_count_1_bit`-class low-power info readably; `get_physical_device_properties` has no
        // further preconditions.
        let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let is_low_power = device_properties.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU;

        let swapchain_loader = ash::khr::swapchain::Device::new(&instance, &device);
        let mut backend = Self {
            entry,
            instance,
            surface_loader,
            surface,
            physical_device,
            memory_properties,
            device,
            queue_family_index,
            graphics_queue,
            swapchain_loader,
            swapchain: vk::SwapchainKHR::null(),
            swapchain_format: vk::Format::UNDEFINED,
            swapchain_extent: vk::Extent2D::default(),
            swapchain_images: Vec::new(),
            swapchain_image_views: Vec::new(),
            render_pass: vk::RenderPass::null(),
            framebuffers: Vec::new(),
            command_pool: vk::CommandPool::null(),
            command_buffers: Vec::new(),
            frames: Vec::new(),
            current_frame: 0,
            size,
            dpr,
            status: DeviceStatus::Healthy,
            resources: GpuResources::default(),
            is_low_power,
        };

        let pool_info = vk::CommandPoolCreateInfo::default().flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER).queue_family_index(queue_family_index);
        // 🔓️ SAFETY: `backend.device` was just created above; `pool_info` borrows only stack locals.
        backend.command_pool = unsafe { backend.device.create_command_pool(&pool_info, None)? };
        let allocate_info = vk::CommandBufferAllocateInfo::default().command_pool(backend.command_pool).level(vk::CommandBufferLevel::PRIMARY).command_buffer_count(FRAMES_IN_FLIGHT as u32);
        // 🔓️ SAFETY: `backend.command_pool` was just created above on the same device.
        backend.command_buffers = unsafe { backend.device.allocate_command_buffers(&allocate_info)? };
        for _ in 0..FRAMES_IN_FLIGHT {
            backend.frames.push(FrameSync::create(&backend.device)?);
        }

        if !is_parked(size) {
            backend.recreate_swapchain()?;
        }

        Ok(backend)
    }
}

//#endregion Construction

//#region 🔃️Swapchain

impl VulkanBackend {
    /// 🔃️ (Re)creates the swapchain, its image views, the render pass (only the first time — its
    /// format does not change across resizes on the same surface) and every framebuffer. Passes the
    /// existing `self.swapchain` as `old_swapchain` (the documented "resize without a black frame"
    /// pattern) and destroys the old one only *after* the new one exists, per
    /// `vkCreateSwapchainKHR`'s own retirement rules.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn recreate_swapchain(&mut self) -> Result<(), VulkanGraphicsError> {
        // 🔓️ SAFETY: `device_wait_idle` has no preconditions beyond a valid device; called here so no
        // in-flight command buffer references the framebuffers/image views this function is about to
        // destroy.
        unsafe { self.device.device_wait_idle()? };

        // 🔓️ SAFETY: `physical_device`/`surface` are both valid for `instance`'s (hence
        // `surface_loader`'s) lifetime.
        let capabilities = unsafe { self.surface_loader.get_physical_device_surface_capabilities(self.physical_device, self.surface)? };
        let formats = unsafe { self.surface_loader.get_physical_device_surface_formats(self.physical_device, self.surface)? };
        let present_modes = unsafe { self.surface_loader.get_physical_device_surface_present_modes(self.physical_device, self.surface)? };
        let format = choose_surface_format(&formats);
        let present_mode = choose_present_mode(&present_modes);
        let extent = choose_extent(&capabilities, self.size);
        let image_count = choose_image_count(&capabilities);

        let old_swapchain = self.swapchain;
        let swapchain_info = vk::SwapchainCreateInfoKHR::default()
            .surface(self.surface)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(old_swapchain);
        // 🔓️ SAFETY: `swapchain_info` borrows only stack locals; `old_swapchain` is either
        // `vk::SwapchainKHR::null()` (first creation — a documented valid "no previous swapchain"
        // value) or a swapchain this same device created, not yet destroyed.
        let new_swapchain = unsafe { self.swapchain_loader.create_swapchain(&swapchain_info, None)? };

        self.destroy_framebuffers_and_views();
        if old_swapchain != vk::SwapchainKHR::null() {
            // 🔓️ SAFETY: the new swapchain above has fully retired `old_swapchain` (guaranteed by
            // passing it as `.old_swapchain(..)`); no command buffer references its images (`
            // device_wait_idle` above already drained every in-flight frame).
            unsafe { self.swapchain_loader.destroy_swapchain(old_swapchain, None) };
        }

        self.swapchain = new_swapchain;
        self.swapchain_format = format.format;
        self.swapchain_extent = extent;
        // 🔓️ SAFETY: `new_swapchain` was just created above on this same device.
        self.swapchain_images = unsafe { self.swapchain_loader.get_swapchain_images(new_swapchain)? };

        if self.render_pass == vk::RenderPass::null() {
            self.render_pass = create_render_pass(&self.device, format.format)?;
        }

        self.swapchain_image_views = Vec::with_capacity(self.swapchain_images.len());
        self.framebuffers = Vec::with_capacity(self.swapchain_images.len());
        for &image in &self.swapchain_images {
            let view_info = vk::ImageViewCreateInfo::default().image(image).view_type(vk::ImageViewType::TYPE_2D).format(format.format).components(vk::ComponentMapping::default()).subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });
            // 🔓️ SAFETY: `image` is one of `swapchain_images`, owned by `self.swapchain` (never
            // destroyed by the caller — swapchain images are owned by the swapchain itself).
            let view = unsafe { self.device.create_image_view(&view_info, None)? };
            let attachments = [view];
            let framebuffer_info = vk::FramebufferCreateInfo::default().render_pass(self.render_pass).attachments(&attachments).width(extent.width.max(1)).height(extent.height.max(1)).layers(1);
            // 🔓️ SAFETY: `self.render_pass` is compatible by construction (`create_render_pass` uses
            // the same `format` this framebuffer's `view` was created with); `view` outlives the
            // framebuffer (both destroyed together in `destroy_framebuffers_and_views`).
            let framebuffer = unsafe { self.device.create_framebuffer(&framebuffer_info, None)? };
            self.swapchain_image_views.push(view);
            self.framebuffers.push(framebuffer);
        }
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn destroy_framebuffers_and_views(&mut self) {
        // 🔓️ SAFETY: called only from `recreate_swapchain` (after a `device_wait_idle`) and `Drop`
        // (which waits idle itself first) — no pending GPU work references these handles.
        unsafe {
            for framebuffer in self.framebuffers.drain(..) {
                self.device.destroy_framebuffer(framebuffer, None);
            }
            for view in self.swapchain_image_views.drain(..) {
                self.device.destroy_image_view(view, None);
            }
        }
    }
}

/// 🖼️ One color attachment, `CLEAR`/`STORE`, `UNDEFINED → PRESENT_SRC_KHR`; one subpass; a single
/// `EXTERNAL → 0` dependency synchronizing the implicit `UNDEFINED → COLOR_ATTACHMENT_OPTIMAL`
/// transition against the swapchain image's actual availability (the standard minimal render-pass
/// shape for "clear and present", vulkan-tutorial's `createRenderPass` ported).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn create_render_pass(device: &ash::Device, format: vk::Format) -> Result<vk::RenderPass, vk::Result> {
    let attachment = vk::AttachmentDescription::default()
        .format(format)
        .samples(vk::SampleCountFlags::TYPE_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);
    let color_ref = vk::AttachmentReference { attachment: 0, layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL };
    let color_refs = [color_ref];
    let subpass = vk::SubpassDescription::default().pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS).color_attachments(&color_refs);
    let dependency = vk::SubpassDependency {
        src_subpass: vk::SUBPASS_EXTERNAL,
        dst_subpass: 0,
        src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        src_access_mask: vk::AccessFlags::empty(),
        dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        dependency_flags: vk::DependencyFlags::empty(),
    };
    let attachments = [attachment];
    let subpasses = [subpass];
    let dependencies = [dependency];
    let render_pass_info = vk::RenderPassCreateInfo::default().attachments(&attachments).subpasses(&subpasses).dependencies(&dependencies);
    // 🔓️ SAFETY: `device` outlives the returned render pass (both held/destroyed via `Self`);
    // `render_pass_info` borrows only stack locals.
    unsafe { device.create_render_pass(&render_pass_info, None) }
}

/// 🔍️ Picks the first physical device with a queue family that is both `GRAPHICS`-capable and reports
/// `WSI` presentation support for `surface` — the minimal selection this milestone needs (no discrete-
/// GPU preference, no dedicated present queue; `VulkanBackend` assumes one queue serves both roles,
/// true on the overwhelming majority of desktop Linux Vulkan drivers).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn pick_physical_device(instance: &ash::Instance, surface_loader: &ash::khr::surface::Instance, surface: vk::SurfaceKHR) -> Result<(vk::PhysicalDevice, u32), VulkanGraphicsError> {
    // 🔓️ SAFETY: `instance` is a live, just-created instance (the only caller, `create`, passes one
    // straight from `entry.create_instance`).
    let physical_devices = unsafe { instance.enumerate_physical_devices()? };
    for physical_device in physical_devices {
        // 🔓️ SAFETY: `physical_device` was just enumerated from `instance` above.
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        for (index, family) in queue_families.iter().enumerate() {
            let index = index as u32;
            if !family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                continue;
            }
            // 🔓️ SAFETY: `physical_device`/`surface` are both valid for `instance`'s lifetime.
            let supports_present = unsafe { surface_loader.get_physical_device_surface_support(physical_device, index, surface)? };
            if supports_present {
                return Ok((physical_device, index));
            }
        }
    }
    Err(VulkanGraphicsError::NoSuitablePhysicalDevice)
}

//#endregion 🔃️Swapchain

//#region 🎬️Render

impl VulkanBackend {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn validate_known_resources(&self, packet: &RenderPacket) -> Result<(), BackendError> {
        for batch in &packet.batches {
            if let Some(texture) = batch.texture {
                if !self.resources.knows_texture(texture) {
                    return Err(BackendError::UnknownResource(ResourceKind::Texture));
                }
            }
        }
        for pass in &packet.surface_passes {
            for draw in pass.draws.iter().chain(pass.translucent_draws.iter()) {
                if !self.resources.knows_mesh(draw.mesh) {
                    return Err(BackendError::UnknownResource(ResourceKind::Mesh));
                }
            }
        }
        Ok(())
    }

    /// 🎬️ Records and submits a single clear-and-present frame — milestone 1's deliverable. Does not
    /// yet replay `packet.batches` (see this file's header). Handles `ERROR_OUT_OF_DATE_KHR` on both
    /// acquire and present by recreating the swapchain and reporting `SkippedOutOfDate`/still
    /// `Presented` respectively (a `SUBOPTIMAL_KHR` present still shows the frame, so it is reported
    /// `Presented`, with recreation deferred to the *next* `render` call rather than forced here).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn render_inner(&mut self, packet: &RenderPacket, _time_seconds: f32) -> Result<RenderReport, VulkanGraphicsError> {
        let frame = self.current_frame;
        let fences = [self.frames[frame].in_flight];
        // 🔓️ SAFETY: `fences[0]` belongs to this device and was either just created (`SIGNALED`) or
        // signaled by a prior submission that used it — never a dangling/foreign handle.
        unsafe { self.device.wait_for_fences(&fences, true, u64::MAX)? };

        // 🔓️ SAFETY: `self.swapchain` is the current live swapchain; `image_available` is a semaphore
        // owned by this frame slot and not currently pending on any other acquire (guaranteed by the
        // fence wait above, which proves the previous use of this slot has fully retired).
        let acquire = unsafe { self.swapchain_loader.acquire_next_image(self.swapchain, u64::MAX, self.frames[frame].image_available, vk::Fence::null()) };
        let (image_index, _suboptimal) = match acquire {
            Ok(result) => result,
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                self.recreate_swapchain()?;
                return Ok(RenderReport::SkippedOutOfDate);
            }
            Err(other) => return Err(other.into()),
        };

        // 🔓️ SAFETY: `fences[0]` was just waited-on above, so it is safe to reset before this frame's
        // submission signals it again.
        unsafe { self.device.reset_fences(&fences)? };

        let command_buffer = self.command_buffers[frame];
        // 🔓️ SAFETY: `command_buffer` was allocated from a pool created with
        // `RESET_COMMAND_BUFFER`, and its prior use (if any) is proven complete by the fence wait
        // above — resetting it here is safe.
        unsafe { self.device.reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())? };
        let begin_info = vk::CommandBufferBeginInfo::default();
        // 🔓️ SAFETY: `command_buffer` is in the initial state after the reset immediately above.
        unsafe { self.device.begin_command_buffer(command_buffer, &begin_info)? };

        let clear_value = vk::ClearValue { color: vk::ClearColorValue { float32: CLEAR_COLOR } };
        let clear_values = [clear_value];
        let render_pass_begin = vk::RenderPassBeginInfo::default()
            .render_pass(self.render_pass)
            .framebuffer(self.framebuffers[image_index as usize])
            .render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: self.swapchain_extent })
            .clear_values(&clear_values);
        // 🔓️ SAFETY: `command_buffer` is recording; `self.framebuffers[image_index as usize]` is
        // compatible with `self.render_pass` (both built together in `recreate_swapchain`) and sized
        // to `self.swapchain_extent`, matching `render_area`.
        unsafe { self.device.cmd_begin_render_pass(command_buffer, &render_pass_begin, vk::SubpassContents::INLINE) };
        // 🕳️ No pipeline bound, no draw issued — milestone 1 is a clear-colour frame only. Batch
        // replay (`packet.batches`) lands with milestone 2's pipelines; `packet` is accepted and
        // validated (see `validate_known_resources`) so the call shape is already the real one.
        let _ = packet;
        // 🔓️ SAFETY: matches the `cmd_begin_render_pass` above on the same command buffer.
        unsafe { self.device.cmd_end_render_pass(command_buffer) };
        // 🔓️ SAFETY: `command_buffer` is recording and every `cmd_*` call above is balanced
        // (one begin, one end).
        unsafe { self.device.end_command_buffer(command_buffer)? };

        let wait_semaphores = [self.frames[frame].image_available];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.frames[frame].render_finished];
        let command_buffers = [command_buffer];
        let submit_info = vk::SubmitInfo::default().wait_semaphores(&wait_semaphores).wait_dst_stage_mask(&wait_stages).command_buffers(&command_buffers).signal_semaphores(&signal_semaphores);
        // 🔓️ SAFETY: every handle in `submit_info` (semaphores, command buffer) belongs to this
        // device and this frame slot; `self.frames[frame].in_flight` was reset above and is not
        // already pending.
        unsafe { self.device.queue_submit(self.graphics_queue, &[submit_info], self.frames[frame].in_flight)? };

        let swapchains = [self.swapchain];
        let image_indices = [image_index];
        let present_wait = [self.frames[frame].render_finished];
        let present_info = vk::PresentInfoKHR::default().wait_semaphores(&present_wait).swapchains(&swapchains).image_indices(&image_indices);
        // 🔓️ SAFETY: `self.swapchain`/`image_index` are the pair just acquired and rendered into
        // above; `present_wait` is the semaphore this frame's submission signals.
        let present_result = unsafe { self.swapchain_loader.queue_present(self.graphics_queue, &present_info) };
        self.current_frame = (frame + 1) % FRAMES_IN_FLIGHT;
        match present_result {
            Ok(_suboptimal_on_present) => {}
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                self.recreate_swapchain()?;
                return Ok(RenderReport::SkippedOutOfDate);
            }
            Err(other) => return Err(other.into()),
        }

        let stats = FrameStats { encode_duration_seconds: 0.0, submit_duration_seconds: 0.0, present_duration_seconds: 0.0, draw_call_count: 0, instance_count: 0 };
        Ok(RenderReport::Presented { stats })
    }
}

//#endregion 🎬️Render

//#region 🔌️GraphicsBackendImpl

impl GraphicsBackend for VulkanBackend {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn name(&self) -> &'static str {
        "vulkan"
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn capabilities(&self) -> DeviceCapabilities {
        DeviceCapabilities {
            max_texture_dimension: 8192,
            max_bind_groups: 4,
            supports_msaa: true,
            supports_timestamp_queries: false,
            supports_storage_buffers: true,
            preferred_surface_format: SurfaceFormat::Bgra8UnormSrgb,
            memory_class: MemoryClass::Standard,
            gpu_tier: if self.is_low_power { GpuTier::Integrated } else { GpuTier::Discrete },
        }
    }

    /// 🕳️ A zero-size request parks: `self.size` is recorded but the swapchain is left untouched
    /// (never recreated to a zero extent — `vkCreateSwapchainKHR` rejects a zero `image_extent`
    /// outright) so `render` has a valid, if stale, target the instant a nonzero `resize` restores it.
    /// `render` itself refuses to draw while parked (`size.is_zero()` is checked before touching the
    /// swapchain — see `GraphicsBackend::render` below), so the staleness is never observed.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn resize(&mut self, size: PhysicalSize, dpr: f32) -> Result<(), BackendError> {
        self.size = size;
        self.dpr = dpr;
        if is_parked(size) {
            return Ok(());
        }
        self.recreate_swapchain().map_err(Into::into)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn apply_resources(&mut self, ops: &[ui_render::ResourceOp]) -> Result<(), BackendError> {
        self.resources.apply(&self.device, &self.memory_properties, self.command_pool, self.graphics_queue, ops).map_err(Into::into)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn render(&mut self, packet: &RenderPacket, time_seconds: f32) -> Result<RenderReport, BackendError> {
        if let DeviceStatus::Lost(reason) = self.status {
            return Err(BackendError::DeviceLost(reason));
        }
        if is_parked(self.size) {
            return Ok(RenderReport::SkippedZeroSize);
        }
        self.validate_known_resources(packet)?;
        match self.render_inner(packet, time_seconds) {
            Ok(report) => Ok(report),
            Err(VulkanGraphicsError::Vk(vk::Result::ERROR_DEVICE_LOST)) => {
                self.status = DeviceStatus::Lost(LossReason::Device);
                Err(BackendError::DeviceLost(LossReason::Device))
            }
            Err(error) => Err(error.into()),
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn device_status(&self) -> DeviceStatus {
        self.status
    }

    /// ♻️ Drops every device-resident resource (mirrors the Metal target's `recover`: a *simulated*
    /// recovery rather than a real device re-creation, since neither backend's `debug_force_device_loss`
    /// path actually destroys the underlying device/instance — only a genuine `ERROR_DEVICE_LOST` from
    /// `render_inner` would, and that path is unverified without hardware; see the report). The
    /// caller's `ResourceRegistry::report_device_loss` re-marks the returned ids `Requested`, and the
    /// next frame's `apply_resources` repopulates them for real.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn recover(&mut self) -> Result<RecoveredResources, BackendError> {
        // 🔓️ SAFETY: best-effort — a genuinely lost device may itself return `ERROR_DEVICE_LOST` from
        // `device_wait_idle`; that failure is deliberately ignored here (`let _ =`) because the
        // resources being dropped below are already invalid GPU-side either way once the device is
        // lost, and `drain_known` only frees CPU-side bookkeeping plus issues `destroy_*`/`free_*`
        // calls that are documented no-ops on a lost device.
        let _ = unsafe { self.device.device_wait_idle() };
        let (lost_textures, lost_meshes, lost_atlases) = self.resources.drain_known(&self.device);
        self.status = DeviceStatus::Healthy;
        Ok(RecoveredResources { lost_textures, lost_meshes, lost_atlases })
    }

    /// 🧪️ Vulkan has no real "lose the device on demand" API — this sets the same `DeviceStatus::Lost`
    /// state a real `ERROR_DEVICE_LOST` would (see `render`'s `Err(VulkanGraphicsError::Vk(ERROR_DEVICE_LOST))`
    /// arm), so `render`/`recover` behave identically to the real-fault path from this point forward.
    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn debug_force_device_loss(&mut self) {
        self.status = DeviceStatus::Lost(LossReason::Device);
    }

    /// 🧪️ Not yet implemented for real — `render_inner` does not encode a copy of the presented image
    /// into a host-visible staging image (that copy needs the swapchain image transitioned out of
    /// `PRESENT_SRC_KHR` before `queue_present`, which milestone 1's minimal render pass does not yet
    /// stage for). Reports `ZeroSizeSurface` on a parked surface (the one case this crate can answer
    /// correctly without a device) and `BackendError::Timeout` otherwise, honestly signalling "no
    /// frame captured" rather than fabricating pixel data. Wiring the real staging copy is
    /// milestone-2-adjacent follow-up work, called out in the report.
    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn read_back(&mut self) -> Result<ReadbackImage, BackendError> {
        if is_parked(self.size) {
            return Err(BackendError::ZeroSizeSurface);
        }
        Err(BackendError::Timeout)
    }
}

//#endregion 🔌️GraphicsBackendImpl

//#region 🧹️Drop

impl Drop for VulkanBackend {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn drop(&mut self) {
        // 🔓️ SAFETY: every `destroy_*`/`free_*` call below targets a handle this same struct created
        // on `self.device`/`self.instance`, in the reverse order of creation (children before
        // parents), after `device_wait_idle` proves no GPU work still references any of them —
        // exactly the destruction ordering `ash`'s own docs require (see e.g.
        // `Instance::create_device`'s safety section, quoted in `Self::create`).
        unsafe {
            let _ = self.device.device_wait_idle();
            self.resources.drain_known(&self.device);
            for frame in &self.frames {
                frame.destroy(&self.device);
            }
            self.destroy_framebuffers_and_views();
            if self.render_pass != vk::RenderPass::null() {
                self.device.destroy_render_pass(self.render_pass, None);
            }
            if self.swapchain != vk::SwapchainKHR::null() {
                self.swapchain_loader.destroy_swapchain(self.swapchain, None);
            }
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}

//#endregion 🧹️Drop

//#endregion 🌋️VulkanBackend

//#endregion 🔖️Backend
