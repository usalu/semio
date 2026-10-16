//! 🧭️ Camera transform and level-of-detail unit laws.

use super::camera::{screen_to_world, world_to_screen, Camera, Viewport};
use super::lod::{Lod, LodScale};
use geometry::Point;

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
