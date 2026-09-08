
use super::*;
use ui_render::{UI_CONTENT_PIPELINE, VECTOR_PIPELINE};

#[test]
fn dynamic_offset_uniform_maps_to_the_dynamic_descriptor_type() {
    assert_eq!(descriptor_type_for(BindingKind::UniformBuffer { dynamic_offset: true, min_size: None }), vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC);
    assert_eq!(descriptor_type_for(BindingKind::UniformBuffer { dynamic_offset: false, min_size: None }), vk::DescriptorType::UNIFORM_BUFFER);
}

#[test]
fn texture_and_sampler_map_to_their_own_descriptor_types() {
    assert_eq!(descriptor_type_for(BindingKind::Texture2D), vk::DescriptorType::SAMPLED_IMAGE);
    assert_eq!(descriptor_type_for(BindingKind::Sampler), vk::DescriptorType::SAMPLER);
}

#[test]
fn stage_visibility_combines_vertex_and_fragment_bits() {
    let both = stage_flags_for(ShaderStageVisibility { vertex: true, fragment: true });
    assert!(both.contains(vk::ShaderStageFlags::VERTEX));
    assert!(both.contains(vk::ShaderStageFlags::FRAGMENT));
    let fragment_only = stage_flags_for(ShaderStageVisibility { vertex: false, fragment: true });
    assert!(!fragment_only.contains(vk::ShaderStageFlags::VERTEX));
}

#[test]
fn ui_globals_bind_group_translates_to_five_bindings_matching_the_shader() {
    let bindings = descriptor_set_layout_bindings(&UI_CONTENT_PIPELINE.bind_groups[0]);
    assert_eq!(bindings.len(), 5);
    assert_eq!(bindings[0].descriptor_type, vk::DescriptorType::UNIFORM_BUFFER);
    assert_eq!(bindings[1].descriptor_type, vk::DescriptorType::SAMPLED_IMAGE);
    assert_eq!(bindings[2].descriptor_type, vk::DescriptorType::SAMPLER);
    assert!(bindings[0].stage_flags.contains(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT));
    assert!(bindings[1].stage_flags.contains(vk::ShaderStageFlags::FRAGMENT));
    assert!(!bindings[1].stage_flags.contains(vk::ShaderStageFlags::VERTEX));
}

#[test]
fn ui_content_pipeline_vertex_input_matches_the_64_byte_quad_instance_stride() {
    let (bindings, attributes) = vertex_input_state(UI_CONTENT_PIPELINE.vertex_buffers);
    assert_eq!(bindings.len(), 2);
    assert_eq!(bindings[0].input_rate, vk::VertexInputRate::VERTEX);
    assert_eq!(bindings[1].input_rate, vk::VertexInputRate::INSTANCE);
    assert_eq!(bindings[1].stride, 64);
    assert_eq!(attributes.len(), 5);
    let instance_attributes: Vec<_> = attributes.iter().filter(|attribute| attribute.binding == 1).collect();
    assert_eq!(instance_attributes.len(), 4);
    for attribute in &instance_attributes {
        assert_eq!(attribute.format, vk::Format::R32G32B32A32_SFLOAT);
    }
}

#[test]
fn vector_pipeline_vertex_input_has_a_single_interleaved_binding() {
    let (bindings, attributes) = vertex_input_state(VECTOR_PIPELINE.vertex_buffers);
    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings[0].stride, 24);
    assert_eq!(attributes.len(), 2);
    assert_eq!(attributes[0].format, vk::Format::R32G32_SFLOAT);
    assert_eq!(attributes[0].offset, 0);
    assert_eq!(attributes[1].format, vk::Format::R32G32B32A32_SFLOAT);
    assert_eq!(attributes[1].offset, 8);
}

#[test]
fn no_scissor_covers_the_full_viewport() {
    let rect = batch_scissor(None, 800, 600);
    assert_eq!(rect.offset, vk::Offset2D { x: 0, y: 0 });
    assert_eq!(rect.extent, vk::Extent2D { width: 800, height: 600 });
}

#[test]
fn a_scissor_rect_within_the_viewport_passes_through_unchanged() {
    let scissor = ScissorRect { x: 10, y: 20, w: 100, h: 50 };
    let rect = batch_scissor(Some(scissor), 800, 600);
    assert_eq!(rect.offset, vk::Offset2D { x: 10, y: 20 });
    assert_eq!(rect.extent, vk::Extent2D { width: 100, height: 50 });
}

#[test]
fn a_scissor_rect_extending_past_a_since_shrunk_viewport_is_clamped() {
    let scissor = ScissorRect { x: 700, y: 500, w: 200, h: 200 };
    let rect = batch_scissor(Some(scissor), 800, 600);
    assert_eq!(rect.offset, vk::Offset2D { x: 700, y: 500 });
    assert_eq!(rect.extent, vk::Extent2D { width: 100, height: 100 });
}
