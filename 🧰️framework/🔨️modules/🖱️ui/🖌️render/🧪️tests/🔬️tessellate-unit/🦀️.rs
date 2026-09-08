
use super::*;
use crate::scene::LayoutRect;

#[test]
fn dashed_line_segments_emit_dashes_along_segment() {
    let segments = dashed_line_segments(0.0, 0.0, 20.0, 0.0, 5.0, 4.0);
    assert!(!segments.is_empty());
    let span: f32 = segments.iter().map(|(x0, _, x1, _)| x1 - x0).sum();
    assert!(span > 0.0 && span <= 20.0);
}

#[test]
fn thick_line_positions_are_symmetric_about_the_segment() {
    let positions = thick_line_positions(0.0, 0.0, 10.0, 0.0, 2.0);
    assert_eq!(positions[0], [0.0, 1.0]);
    assert_eq!(positions[2], [0.0, -1.0]);
}

#[test]
fn ear_clip_produces_triangles() {
    let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    let tris = ear_clip_polygon(&square);
    assert!(tris.len() >= 3);
    assert_eq!(tris.len() % 3, 0);
}

#[test]
fn ear_clip_below_three_points_is_empty() {
    assert!(ear_clip_polygon(&[[0.0, 0.0], [1.0, 1.0]]).is_empty());
}

#[test]
fn triangle_fan_covers_every_interior_triangle() {
    let points = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    assert_eq!(triangle_fan_positions(&points).len(), 2);
}

#[test]
fn snap_rect_snaps_both_edges_independently() {
    let rect = [10.4, 20.6, 30.5, 40.5];
    assert_eq!(snap_rect(rect, 1.0), [10.0, 21.0, 31.0, 40.0]);
}

#[test]
fn snap_to_device_pixels_is_deterministic_across_common_dprs() {
    for dpr in [1.0, 1.5, 2.0] {
        let once = snap_to_device_pixels(12.34, dpr);
        let twice = snap_to_device_pixels(12.34, dpr);
        assert_eq!(once, twice);
    }
}

#[test]
fn silhouette_mask_reset_is_bounded_to_previous_and_current_unions() {
    let previous = Some(ScissorRect { x: 10, y: 10, w: 30, h: 20 });
    let clip = ClipRegion::from_rects(&[LayoutRect::new(80.0, 15.0, 20.0, 25.0)]);
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
