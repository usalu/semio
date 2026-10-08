
use super::camera::{Camera, Viewport, screen_to_world, world_to_screen};
use super::lod::{Lod, LodScale};
use super::text;
use super::theme;
use super::{Point, Scene};

#[test]
fn scale_scene_for_device_pixel_ratio_scales_logical_scene() {
    let mut scene = Scene::new();
    text::append_label(&mut scene, "A", Point::new(10.0, 10.0), 12.0, theme::default_icon_fg(), theme::default_icon_bg());
    let logical = scene.path_count();
    let scaled = super::render::scale_scene_for_device_pixel_ratio(scene, 2.0);
    assert!(scaled.path_count() >= logical);
    let identity = super::render::scale_scene_for_device_pixel_ratio(Scene::new(), 1.0);
    assert_eq!(identity.path_count(), 0);
}

#[test]
fn append_label_renders_glyphs() {
    let mut scene = Scene::new();
    text::append_label(&mut scene, "Zürich", Point::new(40.0, 40.0), 14.0, theme::default_icon_fg(), theme::default_icon_bg());
    assert!(!scene.path_count().eq(&0));
    let mut empty = Scene::new();
    text::append_label(&mut empty, "  ", Point::new(0.0, 0.0), 14.0, theme::default_icon_fg(), theme::default_icon_bg());
    assert!(empty.is_empty());
}

#[test]
fn label_advance_excludes_outer_padding() {
    let px = 14.0;
    let (box_w, _) = text::label_extent("MATCH", px);
    let advance = text::label_advance("MATCH", px);
    assert!(box_w > advance);
    assert_eq!(advance, "MATCH".len() as f64 * px * ui_styling::metrics::label::CHAR_WIDTH_RATIO);
}

#[test]
fn label_byte_world_x_is_narrower_than_char_width_estimate() {
    let line = "MATCH";
    let x0 = text::label_byte_world_x(line, 0, 0.0, 14.0);
    let x5 = text::label_byte_world_x(line, 5, 0.0, 14.0);
    let estimate = text::label_advance("MATCH", 14.0);
    assert!(x5 - x0 < estimate * 0.85);
}

#[test]
fn label_extent_matches_append_label_box() {
    let (w, h) = text::label_extent("math.add", 12.0);
    assert!(w > 32.0);
    assert!(h >= 16.0);
    assert_eq!(text::label_extent("  ", 12.0), (0.0, 0.0));
}

#[test]
fn camera_round_trip() {
    let camera = Camera { x: 10.0, y: -5.0, zoom: 2.0 };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let world = Point::new(12.0, 3.0);
    let screen = world_to_screen(&camera, &viewport, world);
    let back = screen_to_world(&camera, &viewport, screen);
    assert!((back.x - world.x).abs() < 1e-9);
    assert!((back.y - world.y).abs() < 1e-9);
}

#[test]
fn lod_scale_resolve() {
    const LODS: &[Lod] =
        &[Lod { id: "minimap", name: "Minimap", description: "min", max_zoom: 0.15 }, Lod { id: "overview", name: "Overview", description: "ov", max_zoom: 0.35 }, Lod { id: "micro", name: "Micro", description: "mi", max_zoom: f64::INFINITY }];
    let scale = LodScale { lods: LODS };
    assert_eq!(scale.resolve(0.1).id, "minimap");
    assert_eq!(scale.resolve(0.2).id, "overview");
    assert_eq!(scale.resolve(3.0).id, "micro");
    assert_eq!(scale.index_of("overview"), Some(1));
}
