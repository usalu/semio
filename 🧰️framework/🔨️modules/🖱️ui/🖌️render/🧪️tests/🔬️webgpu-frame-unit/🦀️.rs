use super::*;
use ui_render::{FinishParams, Scene, SceneBuilder};

#[test]
fn empty_packet_buckets_into_no_batches() {
    let builder = SceneBuilder::default();
    let packet = Scene::finish(builder, FinishParams { viewport: [100.0, 100.0], dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }).expect("finish");
    let buckets = bucket_batches(&packet);
    assert!(buckets.backdrop_content.is_empty());
    assert!(buckets.glass.is_empty());
}

#[test]
fn solid_quad_batches_into_backdrop_content() {
    let mut builder = SceneBuilder::default();
    builder.push_solid([0.0, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
    let packet = Scene::finish(builder, FinishParams { viewport: [100.0, 100.0], dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }).expect("finish");
    let buckets = bucket_batches(&packet);
    assert_eq!(buckets.backdrop_content.len(), 1);
    assert!(buckets.backdrop_overlay.is_empty());
    assert!(buckets.foreground_content.is_empty());
}

#[test]
fn overlay_quad_batches_into_backdrop_overlay() {
    let mut builder = SceneBuilder::default();
    builder.push_solid_overlay([0.0, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
    let packet = Scene::finish(builder, FinishParams { viewport: [100.0, 100.0], dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }).expect("finish");
    let buckets = bucket_batches(&packet);
    assert!(buckets.backdrop_content.is_empty());
    assert_eq!(buckets.backdrop_overlay.len(), 1);
}

#[test]
fn glass_region_batches_into_its_own_bucket_never_a_scene_phase() {
    let mut builder = SceneBuilder::default();
    builder.push_glass([0.0, 0.0, 40.0, 40.0], 8.0, ui_render::GlassStyle { tint: [1.0, 1.0, 1.0, 1.0], alpha: 0.5, blur_px: 12.0, saturate: 1.0 });
    let packet = Scene::finish(builder, FinishParams { viewport: [100.0, 100.0], dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }).expect("finish");
    let buckets = bucket_batches(&packet);
    assert_eq!(buckets.glass.len(), 1);
    assert!(buckets.backdrop_content.is_empty());
}

#[test]
fn gather_world_passes_is_empty_for_no_surface_passes() {
    let (prepared, instances, lines, points, masks) = gather_world_passes(&[], 800.0, 600.0);
    assert!(prepared.is_empty());
    assert!(instances.is_empty());
    assert!(lines.is_empty());
    assert!(points.is_empty());
    assert!(masks.is_empty());
}

#[test]
fn gather_world_passes_emits_two_mask_quads_per_pass() {
    let pass = SurfacePass { viewport: [0.0, 0.0, 200.0, 150.0], ..SurfacePass::default() };
    let (prepared, _, _, _, masks) = gather_world_passes(std::slice::from_ref(&pass), 800.0, 600.0);
    assert_eq!(prepared.len(), 1);
    assert_eq!(masks.len(), 2);
    assert_eq!(masks[0].rect, [0.0, 0.0, 800.0, 600.0]);
    assert_eq!(masks[1].rect, [0.0, 0.0, 200.0, 150.0]);
}

#[test]
fn gather_world_passes_packs_point_sprites_with_the_points_material_extent() {
    let draw = ui_render::SurfacePointDraw { vertices: vec![ui_render::PointVertex3 { position: [1.0, 2.0, 3.0], color: [1.0, 0.0, 0.0, 1.0] }, ui_render::PointVertex3 { position: [0.0; 3], color: [1.0; 4] }], size: 2.0, size_attenuation: true };
    let pass = SurfacePass { viewport: [0.0, 0.0, 200.0, 100.0], point_draws: vec![draw], ..SurfacePass::default() };
    let (prepared, _, _, points, _) = gather_world_passes(std::slice::from_ref(&pass), 800.0, 600.0);
    assert_eq!(prepared[0].point_range, (0, 2));
    assert_eq!(points[0].position, [1.0, 2.0, 3.0]);
    assert_eq!(points[0].extent, [0.5, 1.0, 0.0, 0.0]);
    assert_eq!(std::mem::size_of::<crate::gpu_uniforms::WorldPointGpuInstance>(), 44);
}
