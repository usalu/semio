use super::*;
use std::mem::{offset_of, size_of};

fn finish_params(viewport: [f32; 2], dpr: f32) -> FinishParams {
    FinishParams { viewport, dpr, time_seconds_origin: 0.0, resource_ops: Vec::new() }
}

//#region FrameTests

#[test]
fn scissor_intersects_child() {
    let a = ScissorRect { x: 0, y: 0, w: 100, h: 100 };
    let b = ScissorRect { x: 50, y: 50, w: 100, h: 100 };
    let c = a.intersect(&b);
    assert_eq!(c.w, 50);
    assert_eq!(c.h, 50);
}

#[test]
fn scissor_covers_fractional_rect_edges_without_pixel_seams() {
    assert_eq!(ScissorRect::from_rect(LayoutRect::new(10.25, 20.75, 30.5, 40.5)), ScissorRect { x: 10, y: 20, w: 31, h: 42 });
}

const FRAME: [f32; 4] = [10.0, 20.0, 200.0, 120.0];

fn frame_rect() -> LayoutRect {
    LayoutRect::new(FRAME[0], FRAME[1], FRAME[2], FRAME[3])
}

fn inside(rect: [f32; 4], frame: [f32; 4]) -> bool {
    rect[0] >= frame[0] && rect[1] >= frame[1] && rect[0] + rect[2] <= frame[0] + frame[2] && rect[1] + rect[3] <= frame[1] + frame[3]
}

#[test]
fn a_frame_clips_with_exactly_its_rectangular_scissor_and_records_no_mask() {
    let mut builder = SceneBuilder::default();
    builder.push_scissor(frame_rect());
    builder.push_solid(FRAME, [0.1, 0.1, 0.1, 1.0]);
    builder.pop_scissor();
    let packet = Scene::finish(builder, finish_params([500.0, 400.0], 1.0)).expect("finish");
    let batch = packet.batches.iter().find(|draw_batch| draw_batch.pipeline == PipelineKind::UiQuad).expect("frame content batch");
    assert_eq!(batch.layer_state.scissor, Some(ScissorRect::from_rect(frame_rect())), "rect clip equals frame");
    assert_eq!(packet.quad_instances.len(), 1, "no stencil mask instances are appended for a frame");
}

#[test]
fn chip_glass_is_ordered_after_the_frame_content_and_inside_the_frame() {
    let style = GlassStyle { tint: [0.2, 0.2, 0.2, 1.0], alpha: 0.6, blur_px: 8.0, saturate: 1.1 };
    let chip = [FRAME[0] + 8.0, FRAME[1], 80.0, 28.0];
    let mut builder = SceneBuilder::default();
    builder.push_scissor(frame_rect());
    builder.push_solid(FRAME, [0.1, 0.1, 0.1, 1.0]);
    builder.pop_scissor();
    let region = builder.push_glass(chip, 0.0, style);
    builder.begin_glass_content(region);
    builder.push_solid(chip, [1.0, 1.0, 1.0, 1.0]);
    builder.end_glass_content();
    let packet = Scene::finish(builder, finish_params([500.0, 400.0], 1.0)).expect("finish");
    let content = packet.batches.iter().position(|draw_batch| draw_batch.pipeline == PipelineKind::UiQuad && draw_batch.layer_state.foreground_of.is_none()).expect("content batch");
    let glass = packet.batches.iter().position(|draw_batch| draw_batch.pipeline == PipelineKind::Glass).expect("chip glass batch");
    let chip_content = packet.batches.iter().position(|draw_batch| draw_batch.pipeline == PipelineKind::UiQuad && draw_batch.layer_state.foreground_of == Some(region)).expect("chip foreground batch");
    assert!(content < glass, "chip glass is ordered after the content it blurs");
    assert!(content < chip_content, "chip foreground is ordered after the frame content");
    assert_eq!(packet.glass_instances.len(), 1);
    assert!(inside(packet.glass_instances[0].rect, FRAME), "the chip glass rect lies inside the frame rect");
    assert!(packet.glass_instances[0].params[1] < 1.0 && packet.glass_instances[0].params[2] > 0.0, "the chip is translucent and blurs the content behind it");
}

#[test]
fn glass_content_layers_tagged_with_foreground_of() {
    let style = GlassStyle { tint: [0.2, 0.2, 0.2, 1.0], alpha: 0.6, blur_px: 4.0, saturate: 1.0 };
    let mut builder = SceneBuilder::default();
    builder.push_solid([0.0, 0.0, 100.0, 100.0], [0.2, 0.2, 0.2, 1.0]);
    let glass = builder.push_glass([10.0, 10.0, 80.0, 80.0], 8.0, style);
    assert_eq!(glass, 0);
    builder.begin_glass_content(glass);
    builder.push_solid([10.0, 10.0, 80.0, 80.0], [1.0, 0.0, 0.0, 1.0]);
    builder.end_glass_content();
    let backdrop = builder.layers.iter().filter(|layer| layer.foreground_of.is_none()).count();
    let foreground = builder.layers.iter().filter(|layer| layer.foreground_of == Some(glass)).count();
    assert_eq!(backdrop, 2);
    assert_eq!(foreground, 1);
    assert_eq!(builder.layers[1].quad_instances.len(), 1);
}

#[test]
fn glass_scissor_inherits_foreground_tag() {
    let style = GlassStyle { tint: [0.0; 4], alpha: 0.5, blur_px: 8.0, saturate: 1.0 };
    let mut builder = SceneBuilder::default();
    let glass = builder.push_glass([0.0, 0.0, 100.0, 100.0], 8.0, style);
    builder.begin_glass_content(glass);
    builder.push_scissor(LayoutRect::new(10.0, 10.0, 80.0, 80.0));
    builder.push_solid([10.0, 10.0, 80.0, 80.0], [0.0, 1.0, 0.0, 1.0]);
    builder.pop_scissor();
    builder.end_glass_content();
    let scissor_layer = builder.layers.iter().find(|layer| layer.scissor.is_some()).expect("scissor layer");
    assert_eq!(scissor_layer.foreground_of, Some(glass));
}

//#endregion FrameTests

#[test]
fn push_scissor_splits_layers() {
    let mut builder = SceneBuilder::default();
    builder.push_solid([0.0, 0.0, 200.0, 200.0], [1.0, 0.0, 0.0, 1.0]);
    builder.push_scissor(LayoutRect::new(10.0, 10.0, 80.0, 80.0));
    builder.push_solid([10.0, 10.0, 80.0, 80.0], [0.0, 1.0, 0.0, 1.0]);
    builder.pop_scissor();
    assert!(builder.layers.len() >= 3);
}

#[test]
fn scene_pass_records_layer_watermarks() {
    let mut builder = SceneBuilder::default();
    builder.push_solid([0.0, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
    builder.push_solid([1.0, 1.0, 8.0, 8.0], [0.0, 1.0, 0.0, 1.0]);
    builder.push_scene_pass(SurfacePass { viewport: [0.0, 0.0, 100.0, 100.0], view_proj: [0.0; 16], light_dir: [0.0, 0.0, 1.0], ..Default::default() });
    builder.push_line(0.0, 0.0, 1.0, 1.0, [0.0, 0.0, 1.0, 1.0], 1.0);
    let pass = &builder.scene_passes[0];
    assert_eq!(pass.layer_index, 0);
    assert_eq!(pass.quad_watermark, 2);
    assert_eq!(pass.vector_watermark, 0);
    assert_eq!(builder.layers[0].quad_instances.len(), 2);
    assert_eq!(builder.layers[0].vector_vertices.len(), 6);
}

#[test]
fn quad_instance_byte_layout_matches_the_wgpu_targets_ui_instance() {
    assert_eq!(size_of::<QuadInstance>(), 64);
    assert_eq!(offset_of!(QuadInstance, rect), 0);
    assert_eq!(offset_of!(QuadInstance, color), 16);
    assert_eq!(offset_of!(QuadInstance, params), 32);
    assert_eq!(offset_of!(QuadInstance, uv_rect), 48);
}

fn sample_builder() -> SceneBuilder {
    let mut builder = SceneBuilder::default();
    builder.push_solid([1.0, 2.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
    builder.push_line(0.0, 0.0, 5.0, 5.0, [0.0, 1.0, 0.0, 1.0], 1.0);
    builder
}

#[test]
fn content_hash_is_stable_across_runs_for_identical_input() {
    let a = Scene::finish(sample_builder(), finish_params([200.0, 200.0], 1.0)).expect("finish");
    let b = Scene::finish(sample_builder(), finish_params([200.0, 200.0], 1.0)).expect("finish");
    assert_eq!(a.content_hash, b.content_hash);
}

#[test]
fn content_hash_differs_when_an_instance_byte_changes() {
    let a = Scene::finish(sample_builder(), finish_params([200.0, 200.0], 1.0)).expect("finish");
    let mut changed = SceneBuilder::default();
    changed.push_solid([1.0, 2.0, 10.0, 11.0], [1.0, 0.0, 0.0, 1.0]);
    changed.push_line(0.0, 0.0, 5.0, 5.0, [0.0, 1.0, 0.0, 1.0], 1.0);
    let b = Scene::finish(changed, finish_params([200.0, 200.0], 1.0)).expect("finish");
    assert_ne!(a.content_hash, b.content_hash);
}

#[test]
fn pixel_snapping_is_deterministic_across_common_dprs() {
    for dpr in [1.0, 1.5, 2.0] {
        let a = Scene::finish(sample_builder(), finish_params([200.0, 200.0], dpr)).expect("finish");
        let b = Scene::finish(sample_builder(), finish_params([200.0, 200.0], dpr)).expect("finish");
        assert_eq!(a.quad_instances, b.quad_instances);
    }
}

#[test]
fn finish_rejects_unbalanced_scissor_stack() {
    let mut builder = SceneBuilder::default();
    builder.push_scissor(LayoutRect::new(0.0, 0.0, 10.0, 10.0));
    let result = Scene::finish(builder, finish_params([50.0, 50.0], 1.0));
    assert_eq!(result.unwrap_err(), SceneError::UnbalancedScissorStack);
}

#[test]
fn finish_rejects_non_finite_geometry() {
    let mut builder = SceneBuilder::default();
    builder.push_solid([f32::NAN, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
    let result = Scene::finish(builder, finish_params([50.0, 50.0], 1.0));
    assert_eq!(result.unwrap_err(), SceneError::NonFiniteGeometry);
}

#[test]
fn finish_drops_layers_left_empty_by_a_push_pop_pair_that_drew_nothing() {
    let mut builder = SceneBuilder::default();
    builder.push_scissor(LayoutRect::new(0.0, 0.0, 10.0, 10.0));
    builder.pop_scissor();
    let packet = Scene::finish(builder, finish_params([50.0, 50.0], 1.0)).expect("finish");
    assert!(packet.batches.is_empty());
}

#[test]
fn finish_merges_layers_that_return_to_an_identical_clip_state() {
    let mut builder = SceneBuilder::default();
    builder.push_scissor(LayoutRect::new(0.0, 0.0, 50.0, 50.0));
    builder.push_solid([1.0, 1.0, 2.0, 2.0], [1.0, 0.0, 0.0, 1.0]);
    builder.pop_scissor();
    builder.push_scissor(LayoutRect::new(0.0, 0.0, 50.0, 50.0));
    builder.push_solid([3.0, 3.0, 2.0, 2.0], [0.0, 1.0, 0.0, 1.0]);
    builder.pop_scissor();
    let packet = Scene::finish(builder, finish_params([100.0, 100.0], 1.0)).expect("finish");
    let quad_batches: Vec<&DrawBatch> = packet.batches.iter().filter(|draw_batch| draw_batch.pipeline == PipelineKind::UiQuad).collect();
    assert_eq!(quad_batches.len(), 1);
    assert_eq!(quad_batches[0].instance_range.1, 2);
}

#[test]
fn finish_emits_one_glass_batch_per_glass_region_with_a_content_hash_dependent_instance() {
    let style = GlassStyle { tint: [0.3, 0.3, 0.3, 1.0], alpha: 0.4, blur_px: 6.0, saturate: 1.1 };
    let mut builder = SceneBuilder::default();
    builder.push_glass([0.0, 0.0, 40.0, 40.0], 8.0, style);
    let packet = Scene::finish(builder, finish_params([100.0, 100.0], 1.0)).expect("finish");
    assert_eq!(packet.glass_instances.len(), 1);
    assert_eq!(packet.batches.iter().filter(|draw_batch| draw_batch.pipeline == PipelineKind::Glass).count(), 1);
}

#[test]
fn stencil_policies_are_distinct() {
    assert_ne!(StencilPolicy::WriteMask, StencilPolicy::RequireMaskEquality);
}
