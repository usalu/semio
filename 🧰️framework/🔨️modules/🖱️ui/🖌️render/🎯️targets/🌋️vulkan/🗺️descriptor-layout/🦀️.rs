//! @emoji 🗺️ Pure translation from `ui_render::shader_contract` (the canonical, backend-neutral
//! pipeline description every `GraphicsBackend` builds from — see that file's header) into Vulkan
//! descriptor/vertex-input/scissor value structs. No device call in this file — every function takes
//! and returns plain `vk::` value structs, so the ticket TESTS section's "descriptor-layout and
//! alignment arithmetic" is exercised here without a device or loader.
//!
//! **Not yet consumed by `crate::backend::VulkanBackend::render`** — pipeline objects themselves need
//! a `vk::ShaderModule` this crate cannot yet build (see `📓️terra-backend-vulkan-report.md`'s "shader
//! strategy" section). These functions are the exact shape `crate::backend` wires up once a shader
//! module exists; writing and testing them now means the milestone-2 gap is a missing SPIR-V blob, not
//! missing logic.

use ash::vk;
use ui_render::{BindGroupSpec, BindingKind, ScissorRect, ShaderStageVisibility, VertexBufferSpec, VertexFormat, VertexStepMode};

//#region 🔖️DescriptorLayout

//#region 🗄️Descriptors

/// 🗄️ `BindingKind` → `vk::DescriptorType`. `UniformBuffer { dynamic_offset: true, .. }` becomes
/// `UNIFORM_BUFFER_DYNAMIC` — the type `vkCmdBindDescriptorSets`'s `p_dynamic_offsets` array applies
/// to, which is exactly `crate::backend`'s world3d dynamic-offset ring (milestone 5, not yet reached)
/// once it lands.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn descriptor_type_for(kind: BindingKind) -> vk::DescriptorType {
    match kind {
        BindingKind::UniformBuffer { dynamic_offset: false, .. } => vk::DescriptorType::UNIFORM_BUFFER,
        BindingKind::UniformBuffer { dynamic_offset: true, .. } => vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
        BindingKind::Texture2D => vk::DescriptorType::SAMPLED_IMAGE,
        BindingKind::Sampler => vk::DescriptorType::SAMPLER,
    }
}

/// 🚦️ `ShaderStageVisibility` → `vk::ShaderStageFlags`. The contract only ever sets `vertex`/
/// `fragment` (see that module's header: "no compute stage exists in this family"), so this never
/// needs a compute/geometry/tessellation arm.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn stage_flags_for(visibility: ShaderStageVisibility) -> vk::ShaderStageFlags {
    let mut flags = vk::ShaderStageFlags::empty();
    if visibility.vertex {
        flags |= vk::ShaderStageFlags::VERTEX;
    }
    if visibility.fragment {
        flags |= vk::ShaderStageFlags::FRAGMENT;
    }
    flags
}

/// 🗂️ One `BindGroupSpec` → the `vk::DescriptorSetLayoutBinding` slice `vkCreateDescriptorSetLayout`
/// takes for that `@group(N)`. `p_immutable_samplers` is always null — this contract never uses
/// immutable samplers (every `Sampler`/`Texture2D` binding is written per-frame via
/// `update_descriptor_sets`, see `crate::resources`).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn descriptor_set_layout_bindings(spec: &BindGroupSpec) -> Vec<vk::DescriptorSetLayoutBinding<'static>> {
    spec.entries.iter().map(|entry| vk::DescriptorSetLayoutBinding::default().binding(entry.binding).descriptor_type(descriptor_type_for(entry.kind)).descriptor_count(1).stage_flags(stage_flags_for(entry.visibility))).collect()
}

//#endregion 🗄️Descriptors

//#region 📐️VertexInput

/// 📐️ `VertexFormat` → the `vk::Format` with the identical byte layout (`Rgba32` naming here refers
/// to component *count*, not color channels — `Float32x2` is a plain 2-component float vector used for
/// positions/UVs, not a color).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn vk_format_for(format: VertexFormat) -> vk::Format {
    match format {
        VertexFormat::Float32x2 => vk::Format::R32G32_SFLOAT,
        VertexFormat::Float32x3 => vk::Format::R32G32B32_SFLOAT,
        VertexFormat::Float32x4 => vk::Format::R32G32B32A32_SFLOAT,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn vk_input_rate_for(step_mode: VertexStepMode) -> vk::VertexInputRate {
    match step_mode {
        VertexStepMode::Vertex => vk::VertexInputRate::VERTEX,
        VertexStepMode::Instance => vk::VertexInputRate::INSTANCE,
    }
}

/// 📐️ `PipelineSpec::vertex_buffers` → the `(bindings, attributes)` pair
/// `vk::PipelineVertexInputStateCreateInfo::vertex_binding_descriptions`/
/// `vertex_attribute_descriptions` need. Binding index is the buffer's position in `buffers` — the
/// same convention every `PipelineSpec` in `shader_contract.rs` already assumes (its
/// `VertexAttributeSpec::shader_location` values are `@location` indices, independent of binding
/// index, exactly mirroring wgpu's/Metal's own buffer-index-is-position convention).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn vertex_input_state(buffers: &[VertexBufferSpec]) -> (Vec<vk::VertexInputBindingDescription>, Vec<vk::VertexInputAttributeDescription>) {
    let mut bindings = Vec::with_capacity(buffers.len());
    let mut attributes = Vec::new();
    for (binding_index, buffer) in buffers.iter().enumerate() {
        let binding = binding_index as u32;
        bindings.push(vk::VertexInputBindingDescription::default().binding(binding).stride(buffer.stride as u32).input_rate(vk_input_rate_for(buffer.step_mode)));
        for attribute in buffer.attributes {
            attributes.push(vk::VertexInputAttributeDescription::default().location(attribute.shader_location).binding(binding).format(vk_format_for(attribute.format)).offset(attribute.offset as u32));
        }
    }
    (bindings, attributes)
}

//#endregion 📐️VertexInput

//#region ✂️Scissor

/// ✂️ Per-batch dynamic scissor — the ticket brief's explicit Vulkan divergence from the Metal target
/// (which resets to full-viewport every batch and clips entirely through the stencil silhouette mask;
/// see `📓️terra-backend-metal-report.md`'s "Decisions"). `batch.layer_state.scissor` is `None` for
/// content painted outside any `push_scissor`/`pop_scissor` region, which maps to the full viewport;
/// `Some(rect)` is already physical-pixel and already intersected with its parent scissor by
/// `SceneBuilder::push_scissor` (see `🎬️scene.rs`), so this function only has to clamp it into the
/// current viewport (a batch computed against a since-shrunk viewport must never scissor outside it).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn batch_scissor(scissor: Option<ScissorRect>, viewport_width: u32, viewport_height: u32) -> vk::Rect2D {
    let full = vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: vk::Extent2D { width: viewport_width, height: viewport_height } };
    let Some(rect) = scissor else { return full };
    let x = rect.x.min(viewport_width);
    let y = rect.y.min(viewport_height);
    let width = rect.w.min(viewport_width.saturating_sub(x));
    let height = rect.h.min(viewport_height.saturating_sub(y));
    vk::Rect2D { offset: vk::Offset2D { x: x as i32, y: y as i32 }, extent: vk::Extent2D { width, height } }
}

//#endregion ✂️Scissor

//#region Tests

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️vulkan-descriptor-layout-unit/🦀️.rs"]
mod tests;

//#endregion Tests

//#endregion 🔖️DescriptorLayout
