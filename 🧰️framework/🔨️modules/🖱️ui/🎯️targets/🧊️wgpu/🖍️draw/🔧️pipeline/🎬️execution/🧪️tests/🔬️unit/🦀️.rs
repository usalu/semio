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

//#region FrameClipTests

const FRAME: [f32; 4] = [10.0, 20.0, 200.0, 120.0];

fn frame_rect() -> Rect {
    Rect::new(FRAME[0], FRAME[1], FRAME[2], FRAME[3])
}

fn inside(rect: [f32; 4], frame: [f32; 4]) -> bool {
    rect[0] >= frame[0] && rect[1] >= frame[1] && rect[0] + rect[2] <= frame[0] + frame[2] && rect[1] + rect[3] <= frame[1] + frame[3]
}

#[test]
fn a_frame_clips_with_exactly_its_rectangular_scissor_and_no_clip_region() {
    let mut draw = DrawList::default();
    draw.push_scissor(frame_rect());
    draw.push_solid(FRAME, Rgba::new(1.0, 1.0, 1.0, 1.0));
    let layer = draw.layers.last().expect("frame content layer");
    assert_eq!(layer.scissor, Some(ScissorRect::from_rect(frame_rect(), 200.0)));
    assert!(layer.clip.is_none(), "a frame never opens a clip region");
    draw.pop_scissor();
    assert!(draw.layers.iter().all(|layer| layer.clip.is_none()));
    assert_eq!(draw.layers.last().and_then(|layer| layer.scissor), None);
}

#[test]
fn chip_glass_is_recorded_after_the_frame_content_and_inside_the_frame() {
    let theme = crate::theme::Theme::default();
    let mut draw = DrawList::default();
    draw.push_scissor(frame_rect());
    draw.push_solid(FRAME, theme.canvas_clear);
    let content_layer = draw.layers.len() - 1;
    draw.pop_scissor();
    let chip = [FRAME[0] + 8.0, FRAME[1], 80.0, 28.0];
    let region = draw.push_glass(chip, 0.0, theme.glass(crate::theme::Level::Window));
    draw.begin_glass_content(region);
    draw.push_solid(chip, theme.selected);
    draw.end_glass_content();
    let glass = draw.glass_regions[region];
    assert!(glass.layer_index > content_layer, "chip glass samples content recorded before it");
    assert!(inside(glass.rect, FRAME), "chip glass stays inside the frame rect");
    assert!(draw.layers[content_layer].ui_instances.len() == 1 && draw.layers[content_layer].foreground_of.is_none(), "the content plane is backdrop for the chip glass");
    assert_eq!(draw.layers.iter().filter(|layer| layer.foreground_of == Some(region)).map(|layer| layer.ui_instances.len()).sum::<usize>(), 1, "chip content paints above the glass");
    assert!(draw.layers.iter().all(|layer| layer.clip.is_none()));
}

#[test]
fn chip_glass_is_translucent_blurred_and_one_level_above_the_content_surface() {
    use crate::theme::Level;
    let theme = crate::theme::Theme::default();
    let chip = theme.glass(Level::Window);
    assert_eq!(Level::Window.index(), Level::Base.index() + 1);
    assert!(chip.alpha < 1.0, "the content must stay visible through a chip");
    assert!(chip.blur_px > 0.0, "a chip blurs the content behind it");
    assert_eq!(theme.glass(Level::Base).blur_px, 0.0, "the content surface itself does not blur");
}

//#endregion FrameClipTests

//#region ScanlineClipTests

#[test]
fn draw_list_nests_and_restores_scanline_clips() {
    let mut draw = DrawList::default();
    assert!(draw.begin_scanline_clip(&[[0, 0, 100, 100]]));
    assert!(draw.begin_scanline_clip(&[[25, 25, 100, 100]]));
    assert_eq!(draw.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.iter().copied().collect::<Vec<_>>()), Some(vec![ScissorRect { x: 25, y: 25, w: 75, h: 75 }]));
    draw.end_clip();
    assert_eq!(draw.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.iter().copied().collect::<Vec<_>>()), Some(vec![ScissorRect { x: 0, y: 0, w: 100, h: 100 }]));
    draw.end_clip();
    assert!(draw.layers.last().is_some_and(|layer| layer.clip.is_none()));
}

#[test]
fn glass_foreground_inherits_active_scanline_clip() {
    let mut draw = DrawList::default();
    assert!(draw.begin_scanline_clip(&[[0, 0, 40, 20], [0, 20, 100, 80]]));
    let glass = draw.push_glass([0.0, 0.0, 40.0, 20.0], 0.0, crate::theme::Theme::default().glass(crate::theme::Level::Window));
    draw.begin_glass_content(glass);
    assert_eq!(draw.layers.last().and_then(|layer| layer.clip.as_ref()).map(|clip| clip.scissors.len()), Some(2));
}

#[test]
fn content_stencil_requires_equality_and_never_writes() {
    let content = content_stencil_state();
    assert_eq!(content.front.compare, gpu_device::CompareFunction::Equal);
    assert_eq!(content.front.pass_op, gpu_device::StencilOperation::Keep);
    assert_eq!(content.write_mask, 0x00);
}

//#endregion ScanlineClipTests

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
