
use super::*;

#[test]
fn vertex_formats_map_one_to_one() {
    assert_eq!(vertex_format(VertexFormat::Float32x2), wgpu::VertexFormat::Float32x2);
    assert_eq!(vertex_format(VertexFormat::Float32x3), wgpu::VertexFormat::Float32x3);
    assert_eq!(vertex_format(VertexFormat::Float32x4), wgpu::VertexFormat::Float32x4);
}

#[test]
fn step_modes_map_one_to_one() {
    assert_eq!(step_mode(VertexStepMode::Vertex), wgpu::VertexStepMode::Vertex);
    assert_eq!(step_mode(VertexStepMode::Instance), wgpu::VertexStepMode::Instance);
}

#[test]
fn none_blend_disables_blending() {
    assert_eq!(blend_state(BlendMode::None), None);
    assert!(blend_state(BlendMode::AlphaBlending).is_some());
    assert!(blend_state(BlendMode::Replace).is_some());
}

#[test]
fn color_write_none_is_empty_mask() {
    assert!(color_writes(ColorWriteMask::None).is_empty());
    assert_eq!(color_writes(ColorWriteMask::All), wgpu::ColorWrites::ALL);
}

#[test]
fn cull_mode_none_disables_culling() {
    assert_eq!(cull_mode(CullMode::None), None);
    assert_eq!(cull_mode(CullMode::Back), Some(wgpu::Face::Back));
}

#[test]
fn uniform_min_size_zero_collapses_to_none() {
    let ty = binding_type(BindingKind::UniformBuffer { dynamic_offset: false, min_size: None });
    assert!(matches!(ty, wgpu::BindingType::Buffer { min_binding_size: None, .. }));
}

#[test]
fn uniform_min_size_some_becomes_nonzero() {
    let ty = binding_type(BindingKind::UniformBuffer { dynamic_offset: true, min_size: Some(80) });
    match ty {
        wgpu::BindingType::Buffer { has_dynamic_offset, min_binding_size: Some(size), .. } => {
            assert!(has_dynamic_offset);
            assert_eq!(size.get(), 80);
        }
        other => panic!("expected a dynamic-offset uniform buffer binding, got {other:?}"),
    }
}

#[test]
fn depth_stencil_state_round_trips_bias_and_masks() {
    let spec = DepthStencilSpec {
        format: ui_render::DepthStencilFormat::Depth24PlusStencil8,
        depth_write_enabled: true,
        depth_compare: CompareFunction::LessEqual,
        stencil: ui_render::StencilStateSpec { compare: CompareFunction::Equal, fail_op: StencilOperation::Keep, depth_fail_op: StencilOperation::Keep, pass_op: StencilOperation::Replace, read_mask: 0xff, write_mask: 0x00 },
        bias: ui_render::DepthBiasSpec { constant: -2, slope_scale: -1.0, clamp: 0.0 },
    };
    let state = depth_stencil_state(&spec);
    assert_eq!(state.format, DEPTH_STENCIL_FORMAT);
    assert_eq!(state.depth_write_enabled, Some(true));
    assert_eq!(state.depth_compare, Some(wgpu::CompareFunction::LessEqual));
    assert_eq!(state.bias.constant, -2);
    assert_eq!(state.stencil.read_mask, 0xff);
    assert_eq!(state.stencil.write_mask, 0x00);
}
