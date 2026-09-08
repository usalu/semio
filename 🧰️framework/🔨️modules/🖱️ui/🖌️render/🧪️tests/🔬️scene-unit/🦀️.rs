
use super::*;
use std::mem::{offset_of, size_of};

fn finish_params(viewport: [f32; 2], dpr: f32) -> FinishParams {
    FinishParams { viewport, dpr, time_seconds_origin: 0.0, resource_ops: Vec::new() }
}

//#region SilhouetteClipTests

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

#[test]
fn clip_region_preserves_disjoint_cutouts_and_intersects_scissor() {
    let clip = ClipRegion::from_rects(&[LayoutRect::new(0.0, 0.0, 40.0, 20.0), LayoutRect::new(80.0, 0.0, 20.0, 20.0), LayoutRect::new(0.0, 20.0, 100.0, 80.0)]);
    let scissors = clip.effective_scissors(Some(ScissorRect { x: 10, y: 0, w: 80, h: 100 }), 100.0, 100.0);
    assert_eq!(scissors, vec![ScissorRect { x: 10, y: 0, w: 30, h: 20 }, ScissorRect { x: 80, y: 0, w: 10, h: 20 }, ScissorRect { x: 10, y: 20, w: 80, h: 80 }]);
}

#[test]
fn scene_builder_nests_and_restores_clip_regions() {
    let mut builder = SceneBuilder::default();
    builder.begin_silhouette_clip(&[LayoutRect::new(0.0, 0.0, 100.0, 100.0)]);
    builder.begin_silhouette_clip(&[LayoutRect::new(25.0, 25.0, 100.0, 100.0)]);
    assert_eq!(builder.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.as_slice()), Some([ScissorRect { x: 25, y: 25, w: 75, h: 75 }].as_slice()));
    builder.end_silhouette_clip();
    assert_eq!(builder.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.as_slice()), Some([ScissorRect { x: 0, y: 0, w: 100, h: 100 }].as_slice()));
    builder.end_silhouette_clip();
    assert!(builder.layers.last().is_some_and(|layer| layer.clip.is_none()));
}

#[test]
fn glass_foreground_inherits_active_silhouette_clip() {
    let mut builder = SceneBuilder::default();
    builder.begin_silhouette_clip(&[LayoutRect::new(0.0, 0.0, 40.0, 20.0), LayoutRect::new(0.0, 20.0, 100.0, 80.0)]);
    let style = GlassStyle { tint: [0.1, 0.1, 0.1, 1.0], alpha: 0.5, blur_px: 8.0, saturate: 1.2 };
    let glass = builder.push_glass([0.0, 0.0, 40.0, 20.0], 0.0, style);
    builder.begin_glass_content(glass);
    assert_eq!(builder.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.len()), Some(2));
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

//#endregion SilhouetteClipTests

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
fn finish_computes_a_mask_range_for_a_silhouette_clipped_batch() {
    let mut builder = SceneBuilder::default();
    builder.begin_silhouette_clip(&[LayoutRect::new(0.0, 0.0, 40.0, 40.0)]);
    builder.push_solid([1.0, 1.0, 2.0, 2.0], [1.0, 0.0, 0.0, 1.0]);
    builder.end_silhouette_clip();
    let packet = Scene::finish(builder, finish_params([100.0, 100.0], 1.0)).expect("finish");
    let quad_batch = packet.batches.iter().find(|draw_batch| draw_batch.pipeline == PipelineKind::UiQuad).expect("quad batch");
    assert!(quad_batch.mask_range.is_some());
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
