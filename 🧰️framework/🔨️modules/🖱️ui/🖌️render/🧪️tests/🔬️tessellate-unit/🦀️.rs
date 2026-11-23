
use super::*;
use crate::scene::{LayoutRect, ScissorRect};

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
fn a_frame_clip_is_its_rectangle_intersected_with_the_viewport() {
    let frame = ScissorRect::from_rect(LayoutRect::new(10.0, 20.0, 200.0, 120.0));
    assert_eq!(viewport_scissor(Some(frame), 500.0, 400.0), Some(ScissorRect { x: 10, y: 20, w: 200, h: 120 }));
    assert_eq!(viewport_scissor(Some(frame), 100.0, 60.0), Some(ScissorRect { x: 10, y: 20, w: 90, h: 40 }), "a frame never paints outside the viewport");
}

#[test]
fn a_batch_without_scissor_is_clipped_to_the_viewport_only() {
    assert_eq!(viewport_scissor(None, 500.0, 400.0), Some(ScissorRect { x: 0, y: 0, w: 500, h: 400 }));
}

#[test]
fn a_frame_outside_the_viewport_or_without_area_paints_nothing() {
    assert_eq!(viewport_scissor(Some(ScissorRect { x: 600, y: 0, w: 10, h: 10 }), 500.0, 400.0), None);
    assert_eq!(viewport_scissor(Some(ScissorRect { x: 10, y: 10, w: 0, h: 10 }), 500.0, 400.0), None);
    assert_eq!(viewport_scissor(None, 0.0, 400.0), None);
}
