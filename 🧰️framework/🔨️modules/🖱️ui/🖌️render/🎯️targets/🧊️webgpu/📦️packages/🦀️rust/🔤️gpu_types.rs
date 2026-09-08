//! @emoji 🔤️ Pure `ui_render::shader_contract` → `wgpu` translations. Every fn here maps one enum/
//! struct to its `wgpu` mirror and nothing else — no device, no allocation — so each is exercised by
//! `#[cfg(test)]` without a device, per this packet's brief ("resource-op→GPU-op translation" and
//! sibling pure-logic units).

use ui_render::{BindingKind, BlendMode, ColorWriteMask, CompareFunction, CullMode, DepthStencilSpec, PrimitiveTopology, StencilOperation, VertexFormat, VertexStepMode};

//#region 🔖️GpuTypes

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn vertex_format(format: VertexFormat) -> wgpu::VertexFormat {
    match format {
        VertexFormat::Float32x2 => wgpu::VertexFormat::Float32x2,
        VertexFormat::Float32x3 => wgpu::VertexFormat::Float32x3,
        VertexFormat::Float32x4 => wgpu::VertexFormat::Float32x4,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn step_mode(mode: VertexStepMode) -> wgpu::VertexStepMode {
    match mode {
        VertexStepMode::Vertex => wgpu::VertexStepMode::Vertex,
        VertexStepMode::Instance => wgpu::VertexStepMode::Instance,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn compare_function(compare: CompareFunction) -> wgpu::CompareFunction {
    match compare {
        CompareFunction::Always => wgpu::CompareFunction::Always,
        CompareFunction::Equal => wgpu::CompareFunction::Equal,
        CompareFunction::Less => wgpu::CompareFunction::Less,
        CompareFunction::LessEqual => wgpu::CompareFunction::LessEqual,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn stencil_operation(op: StencilOperation) -> wgpu::StencilOperation {
    match op {
        StencilOperation::Keep => wgpu::StencilOperation::Keep,
        StencilOperation::Replace => wgpu::StencilOperation::Replace,
    }
}

/// 🎨️ `None` disables blending (the silhouette mask pass writes no color anyway); `Replace`/
/// `AlphaBlending` mirror `wgpu::BlendState::REPLACE`/`ALPHA_BLENDING`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn blend_state(mode: BlendMode) -> Option<wgpu::BlendState> {
    match mode {
        BlendMode::None => None,
        BlendMode::AlphaBlending => Some(wgpu::BlendState::ALPHA_BLENDING),
        BlendMode::Replace => Some(wgpu::BlendState::REPLACE),
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn color_writes(mask: ColorWriteMask) -> wgpu::ColorWrites {
    match mask {
        ColorWriteMask::None => wgpu::ColorWrites::empty(),
        ColorWriteMask::All => wgpu::ColorWrites::ALL,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn primitive_topology(topology: PrimitiveTopology) -> wgpu::PrimitiveTopology {
    match topology {
        PrimitiveTopology::TriangleList => wgpu::PrimitiveTopology::TriangleList,
        PrimitiveTopology::LineList => wgpu::PrimitiveTopology::LineList,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn cull_mode(mode: CullMode) -> Option<wgpu::Face> {
    match mode {
        CullMode::None => None,
        CullMode::Back => Some(wgpu::Face::Back),
    }
}

/// 🕳️ The one depth/stencil texture format this whole shader family uses.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) const DEPTH_STENCIL_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24PlusStencil8;

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn depth_stencil_state(spec: &DepthStencilSpec) -> wgpu::DepthStencilState {
    let face =
        wgpu::StencilFaceState { compare: compare_function(spec.stencil.compare), fail_op: stencil_operation(spec.stencil.fail_op), depth_fail_op: stencil_operation(spec.stencil.depth_fail_op), pass_op: stencil_operation(spec.stencil.pass_op) };
    wgpu::DepthStencilState {
        format: DEPTH_STENCIL_FORMAT,
        depth_write_enabled: Some(spec.depth_write_enabled),
        depth_compare: Some(compare_function(spec.depth_compare)),
        stencil: wgpu::StencilState { front: face, back: face, read_mask: spec.stencil.read_mask, write_mask: spec.stencil.write_mask },
        bias: wgpu::DepthBiasState { constant: spec.bias.constant, slope_scale: spec.bias.slope_scale, clamp: spec.bias.clamp },
    }
}

/// 🗄️ `min_size` of `None` (wgpu infers it from the shader) needs no `NonZeroU64`; `Some(n)` maps
/// straight through — every real contract value here is a small nonzero constant.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn binding_type(kind: BindingKind) -> wgpu::BindingType {
    match kind {
        BindingKind::UniformBuffer { dynamic_offset, min_size } => wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: dynamic_offset, min_binding_size: min_size.and_then(std::num::NonZeroU64::new) },
        BindingKind::Texture2D => wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
        BindingKind::Sampler => wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
    }
}

//#endregion 🔖️GpuTypes

//#region Tests

#[cfg(test)]
#[path = "../../../../🧪️tests/🔬️webgpu-packages-rust-gpu-types-unit/🦀️.rs"]
mod tests;

//#endregion Tests
