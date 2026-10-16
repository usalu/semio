use super::*;
use crate::geometry::Rect;
use crate::kernel_3d_scene::ScenePass3d;
use crate::theme::Rgba;

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
    assert_eq!(ScissorRect::from_rect(Rect::new(10.25, 20.75, 30.5, 40.5), 100.0), ScissorRect { x: 10, y: 20, w: 31, h: 42 });
}

//#region SilhouetteClipTests

#[test]
fn clip_region_preserves_disjoint_cutouts_and_intersects_scissor() {
    let clip = ClipRegion::from_rects(&[Rect::new(0.0, 0.0, 40.0, 20.0), Rect::new(80.0, 0.0, 20.0, 20.0), Rect::new(0.0, 20.0, 100.0, 80.0)], 100.0);
    let scissors = clip.effective_scissors(Some(ScissorRect { x: 10, y: 0, w: 80, h: 100 }), 100.0, 100.0);
    assert_eq!(scissors, vec![ScissorRect { x: 10, y: 0, w: 30, h: 20 }, ScissorRect { x: 80, y: 0, w: 10, h: 20 }, ScissorRect { x: 10, y: 20, w: 80, h: 80 }]);
}

#[test]
fn draw_list_nests_and_restores_clip_regions() {
    let mut draw = DrawList::default();
    draw.begin_silhouette_clip(&[Rect::new(0.0, 0.0, 100.0, 100.0)]);
    draw.begin_silhouette_clip(&[Rect::new(25.0, 25.0, 100.0, 100.0)]);
    assert_eq!(draw.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.as_slice()), Some([ScissorRect { x: 25, y: 25, w: 75, h: 75 }].as_slice()));
    draw.end_silhouette_clip();
    assert_eq!(draw.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.as_slice()), Some([ScissorRect { x: 0, y: 0, w: 100, h: 100 }].as_slice()));
    draw.end_silhouette_clip();
    assert!(draw.layers.last().is_some_and(|layer| layer.clip.is_none()));
}

#[test]
fn glass_foreground_inherits_active_silhouette_clip() {
    let mut draw = DrawList::default();
    draw.begin_silhouette_clip(&[Rect::new(0.0, 0.0, 40.0, 20.0), Rect::new(0.0, 20.0, 100.0, 80.0)]);
    let glass = draw.push_glass([0.0, 0.0, 40.0, 20.0], 0.0, crate::theme::Theme::default().glass(crate::theme::Level::Window));
    draw.begin_glass_content(glass);
    assert_eq!(draw.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.len()), Some(2));
}

#[test]
fn fractional_adjacent_silhouette_pieces_keep_one_physical_owner_per_pixel() {
    let mut draw = DrawList::default();
    draw.begin_retained_output(16, 4_096).expect("bounded retained output grant");
    draw.begin_silhouette_clip(&[Rect::new(10.0, 48.8, 600.0, 371.2), Rect::new(10.0, 20.0, 170.4, 28.8)]);
    assert!(draw.finish_retained_output().is_ok(), "logical adjacency must survive conservative physical scissor rounding");
    let pieces = &draw.layers.last().expect("clipped layer").clip.as_ref().expect("silhouette clip").scissors;
    assert_eq!(pieces.len(), 2);
    assert!(pieces[0].intersect(&pieces[1]).w == 0 || pieces[0].intersect(&pieces[1]).h == 0, "one physical pixel cannot belong to both alpha pieces");
}

#[test]
fn silhouette_stencil_states_write_masks_then_require_equality() {
    let mask = mask_stencil_state();
    assert_eq!(mask.front.compare, gpu_device::CompareFunction::Always);
    assert_eq!(mask.front.pass_op, gpu_device::StencilOperation::Replace);
    assert_eq!(mask.write_mask, 0xff);
    let content = content_stencil_state();
    assert_eq!(content.front.compare, gpu_device::CompareFunction::Equal);
    assert_eq!(content.front.pass_op, gpu_device::StencilOperation::Keep);
    assert_eq!(content.write_mask, 0x00);
}

#[test]
fn silhouette_mask_reset_is_bounded_to_previous_and_current_unions() {
    let previous = Some(ScissorRect { x: 10, y: 10, w: 30, h: 20 });
    let clip = ClipRegion { scissors: vec![ScissorRect { x: 80, y: 15, w: 20, h: 25 }] };
    let (instances, current) = mask_instances(None, Some(&clip), previous, 500.0, 400.0);
    assert_eq!(instances[0].rect, [10.0, 10.0, 90.0, 30.0]);
    assert_eq!(instances[1].rect, [80.0, 15.0, 20.0, 25.0]);
    assert_eq!(current, Some(ScissorRect { x: 80, y: 15, w: 20, h: 25 }));
}

#[test]
fn empty_silhouette_clip_writes_no_visible_stencil_region() {
    let empty = ClipRegion { scissors: Vec::new() };
    let (instances, current) = mask_instances(None, Some(&empty), None, 500.0, 400.0);
    assert!(instances.is_empty(), "a cleared pass needs neither a reset nor a reference-one mask draw");
    assert_eq!(current, None);
}

//#endregion SilhouetteClipTests

#[test]
fn scissor_from_rect_uses_top_left_origin() {
    let scissor = ScissorRect::from_rect(Rect::new(10.0, 20.0, 80.0, 60.0), 720.0);
    assert_eq!(scissor.x, 10);
    assert_eq!(scissor.y, 20);
    assert_eq!(scissor.w, 80);
    assert_eq!(scissor.h, 60);
}

#[test]
fn draw_list_push_scissor_splits_layers() {
    let mut draw = DrawList::default();
    draw.set_screen_height(200.0);
    draw.push_solid([0.0, 0.0, 200.0, 200.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
    draw.push_scissor(Rect::new(10.0, 10.0, 80.0, 80.0));
    draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
    draw.pop_scissor();
    assert!(draw.layers.len() >= 3);
}

#[test]
fn ear_clip_produces_triangles() {
    let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    let tris = ear_clip_polygon(&square);
    assert!(tris.len() >= 3);
}

#[test]
fn scene_pass_terminates_its_layer_and_records_the_complete_preceding_watermarks() {
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 10.0, 10.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
    draw.push_solid([1.0, 1.0, 8.0, 8.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
    draw.push_scene_pass(ScenePass3d { viewport: [0.0, 0.0, 100.0, 100.0], view_proj: [0.0; 16], light_dir: [0.0, 0.0, 1.0], ..Default::default() });
    draw.push_line(0.0, 0.0, 1.0, 1.0, Rgba::new(0.0, 0.0, 1.0, 1.0), 1.0);
    let pass = &draw.scene_passes[0];
    assert_eq!(pass.layer_index, 0);
    assert_eq!(pass.ui_watermark, 2);
    assert_eq!(pass.vector_watermark, 0);
    assert_eq!(draw.layers[0].ui_instances.len(), 2);
    assert!(draw.layers[0].vector_vertices.is_empty());
    assert_eq!(draw.layers[1].vector_vertices.len(), 6);
    assert_eq!(draw.layers[1].scissor, draw.layers[0].scissor);
    assert_eq!(draw.layers[1].foreground_of, draw.layers[0].foreground_of);
}

#[test]
fn mesh_instances_without_lines_are_valid_world_pass() {
    use crate::kernel_3d_scene::{Instance3d, SceneDraw3d, ScenePass3d};

    let pass = ScenePass3d {
        viewport: [0.0, 0.0, 320.0, 240.0],
        view_proj: [0.0; 16],
        light_dir: [0.4, 0.6, 0.8],
        draws: vec![SceneDraw3d {
            mesh_key: "box".into(),
            mesh_version: 1,
            instances: vec![Instance3d { component_source: None, id: "preview".into(), model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]), color: [0.7, 0.7, 0.75, 1.0], selected: false, hovered: false, material: Default::default() }],
            shadow_role: Default::default(),
        }],
        ..Default::default()
    };
    assert!(!pass.draws[0].instances.is_empty());
    assert!(pass.line_draws.is_empty());
}

/// 🧱️ LAW: the vertex-colour/shadow policy and the standard material pack into ONE `flags` lane; the stride grew exactly once, for
/// the authored (GLB) material's emissive colour + alpha cutoff lane (`emissive_cutoff`, 09-28), which a standard instance leaves
/// emissive-free with its cutoff disabled.
#[test]
fn offscreen_shadow_channel_is_measured_and_progressively_retired() {
    use crate::kernel_3d_scene::{Instance3d, SceneDraw3d, ScenePass3d, SceneShadowRole3d};
    let mut draw = DrawList::default();
    draw.push_scene_pass(ScenePass3d {
        shadow_draws: vec![SceneDraw3d {
            mesh_key: "offscreen-caster".into(),
            mesh_version: 7,
            instances: vec![Instance3d { component_source: None, id: "caster".into(), model: Instance3d::model_from_trs([0.0; 3], [0.0, 0.0, 0.0, 1.0], [1.0; 3]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }],
            shadow_role: SceneShadowRole3d { casts: true, receives: true },
        }],
        ..Default::default()
    });
    let (items, bytes) = draw.prepared_output_usage();
    assert!(items >= 1 + "offscreen-caster".len() + "caster".len());
    assert!(bytes >= std::mem::size_of::<ScenePass3d>() + std::mem::size_of::<SceneDraw3d>() + std::mem::size_of::<Instance3d>() + "offscreen-caster".len() + "caster".len());
    let mut steps = 0;
    while !draw.retire_step() {
        steps += 1;
        assert!(steps < 256, "shadow-channel retirement remains scalar and terminal");
    }
    assert!(draw.scene_passes.is_empty());
}

#[test]
fn mesh_content_version_changes_with_indices() {
    let v0 = mesh_content_version(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 1, 2]);
    let v1 = mesh_content_version(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 2, 1]);
    assert_ne!(v0, v1);
}

#[test]
fn overlay_layers_collected_separately_from_backdrop_ui() {
    use super::{build_layer_batches, build_overlay_layer_batches, LayerBatchFilter};
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 100.0, 100.0], Rgba::new(0.1, 0.1, 0.1, 1.0));
    draw.push_glyph_overlay([10.0, 10.0, 20.0, 12.0], Rgba::new(1.0, 1.0, 1.0, 1.0), [0.0, 0.0, 0.1, 0.1]);
    draw.push_line_overlay(0.0, 0.0, 50.0, 50.0, Rgba::new(1.0, 0.0, 0.0, 1.0), 1.0);
    let (backdrop_ui, _, _) = build_layer_batches(&draw, LayerBatchFilter::Backdrop);
    let (overlay_ui, overlay_vec, overlay_batches) = build_overlay_layer_batches(&draw, LayerBatchFilter::Backdrop);
    assert_eq!(backdrop_ui.len(), 1);
    assert_eq!(overlay_ui.len(), 1);
    assert_eq!(overlay_vec.len(), 6);
    assert_eq!(overlay_batches.len(), 1);
    assert_eq!(draw.layers[overlay_batches[0].layer_index].overlay_ui_instances.len(), 1);
}

#[test]
fn overlay_rasters_are_collected_separately_without_changing_the_texture_identity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🧪️tests/🧫️fixtures/🔽️retained-select-overlay-raster/🔣️.json")).expect("retained Select/overlay raster fixture");
    let key = fixture["image"]["sharedKey"].as_str().expect("shared raster key");
    let mut draw = DrawList::default();
    draw.push_raster_quad(key, [1.0, 2.0, 3.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    draw.begin_overlay_route();
    draw.push_raster_quad(key, [5.0, 6.0, 7.0, 8.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    draw.end_overlay_route();

    assert_eq!(draw.layers[0].raster_instances.len(), 1, "an in-flow image remains in the scene raster lane");
    assert_eq!(draw.layers[0].overlay_raster_instances.len(), 1, "a dialog/popover image follows its overlay route");
    assert_eq!(draw.layers[0].raster_instances[0].0, key);
    assert_eq!(draw.layers[0].overlay_raster_instances[0].0, key, "routing changes only ordering, never the upload key");
    assert_eq!(draw.layers[0].overlay_raster_instances[0].1.rect, [5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn glass_content_layers_tagged_with_foreground_of() {
    use super::Theme;
    use crate::theme::Level;
    let theme = Theme::default();
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 100.0, 100.0], Rgba::new(0.2, 0.2, 0.2, 1.0));
    let glass = draw.push_glass([10.0, 10.0, 80.0, 80.0], 8.0, theme.glass(Level::Panel));
    assert_eq!(glass, 0);
    draw.begin_glass_content(glass);
    draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
    draw.end_glass_content();
    let backdrop = draw.layers.iter().filter(|layer| layer.foreground_of.is_none()).count();
    let foreground = draw.layers.iter().filter(|layer| layer.foreground_of == Some(glass)).count();
    assert_eq!(backdrop, 3);
    assert_eq!(foreground, 1);
    assert_eq!(draw.glass_regions[glass].layer_index, 1);
    assert!(draw.layers[1].ui_instances.is_empty());
    assert_eq!(draw.layers[2].ui_instances.len(), 1);
}

#[test]
fn glass_foreground_layers_excluded_from_backdrop_batches() {
    use super::{build_layer_batches, LayerBatchFilter, Theme};
    use crate::theme::Level;
    let theme = Theme::default();
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 200.0, 200.0], Rgba::new(0.1, 0.1, 0.1, 1.0));
    let glass = draw.push_glass([20.0, 20.0, 160.0, 160.0], 8.0, theme.glass(Level::Panel));
    draw.begin_glass_content(glass);
    draw.push_solid([20.0, 20.0, 160.0, 160.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
    draw.end_glass_content();
    let (backdrop_ui, _, backdrop_batches) = build_layer_batches(&draw, LayerBatchFilter::Backdrop);
    let (foreground_ui, _, foreground_batches) = build_layer_batches(&draw, LayerBatchFilter::Foreground);
    assert_eq!(backdrop_ui.len(), 1);
    assert_eq!(foreground_ui.len(), 1);
    assert_eq!(backdrop_batches.len(), 1);
    assert_eq!(foreground_batches.len(), 1);
    assert!(draw.layers[backdrop_batches[0].layer_index].foreground_of.is_none());
    assert_eq!(draw.layers[foreground_batches[0].layer_index].foreground_of, Some(glass));
}

#[test]
fn glass_scissor_inherits_foreground_tag() {
    use super::Theme;
    use crate::theme::Level;
    let theme = Theme::default();
    let mut draw = DrawList::default();
    let glass = draw.push_glass([0.0, 0.0, 100.0, 100.0], 8.0, theme.glass(Level::Panel));
    draw.begin_glass_content(glass);
    draw.push_scissor(Rect::new(10.0, 10.0, 80.0, 80.0));
    draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
    draw.pop_scissor();
    draw.end_glass_content();
    let scissor_layer = draw.layers.iter().find(|layer| layer.scissor.is_some()).expect("scissor layer");
    assert_eq!(scissor_layer.foreground_of, Some(glass));
}

/// 🪜️ `Theme::glass` must be formula-derived off `Level::index` (never a per-tier lookup
/// table): alpha/blur both monotone across all 6 levels. There is deliberately no separate
/// "chrome" variant — a level's attached chrome (title caps, ribbons, tab bars, rails) always
/// renders the exact same `glass(level)` as its body, so one level never shows two appearances.
#[test]
fn glass_alpha_and_blur_are_formula_derived_per_level() {
    use super::Theme;
    use crate::theme::Level;
    let theme = Theme::default();
    let ordered = [Level::Base, Level::Window, Level::Pane, Level::Panel, Level::Dialog, Level::Menu];
    for (k, level) in ordered.iter().enumerate() {
        assert_eq!(level.index(), k);
        let style = theme.glass(*level);
        assert!((style.alpha - (1.0 - k as f32 * ui_styling::levels::GLASS_ALPHA_STEP as f32)).abs() < 1e-6);
        assert!((style.blur_px - k as f32 * ui_styling::levels::GLASS_BLUR_STEP_PX as f32).abs() < 1e-6);
        assert_eq!(style.tint, theme.level_bg[k]);
    }
    assert!(theme.glass(Level::Base).alpha > theme.glass(Level::Menu).alpha);
    assert!(theme.glass(Level::Base).blur_px < theme.glass(Level::Menu).blur_px);
    assert_eq!(theme.surface(Level::Panel), theme.level_bg[Level::Panel.index()]);
}

/// 📻️ The image sample owns the avatar radius without changing ordinary raster geometry.
#[test]
fn rounded_avatar_rasters_keep_their_radius_through_the_draw_packet() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🧱️elements/⚙️VirtualFileSystem/🧪️tests/🧫️fixtures/🧾️descriptors/🔣️.json")).unwrap();
    let law = &fixture["avatarGeometry"];
    let rect: [f32; 4] = serde_json::from_value(law["rect"].clone()).unwrap();
    let uv: [f32; 4] = serde_json::from_value(law["uv"].clone()).unwrap();
    let mut draw = DrawList::default();
    draw.push_rounded_raster_quad("avatar", rect, uv, law["alpha"].as_f64().unwrap() as f32, law["oversizedRadius"].as_f64().unwrap() as f32);
    draw.push_raster_quad("ordinary", rect, uv, 1.0);
    assert_eq!(draw.layers[0].raster_instances[0].1.params[0], law["radius"].as_f64().unwrap() as f32);
    assert_eq!(draw.layers[0].raster_instances[0].1.rect, rect);
    assert_eq!(draw.layers[0].raster_instances[0].1.uv_rect, uv);
    assert_eq!(draw.layers[0].raster_instances[1].1.params[0], 0.0);
}

/// 🎨️ LAW (ticket 26/09/23 session 14d, WG11): every edit that turns the lit mesh shader into the painted one finds its anchor
/// EXACTLY once — the emissive anchor once lost its match when the lit shader gained `+ in.emissive_cutoff.rgb`, the
/// `lit_color` declaration was silently dropped, and every GPU context refused `world3d_painted_shader`.
