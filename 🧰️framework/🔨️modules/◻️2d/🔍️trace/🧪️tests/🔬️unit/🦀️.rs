
use super::*;

#[test]
fn traces_filled_square() {
    let width = 8_u32;
    let height = 8_u32;
    let mut mask = vec![0_u8; (width * height) as usize];
    for y in 2..6 {
        for x in 2..6 {
            mask[(y * width + x) as usize] = 255;
        }
    }
    let segments = trace_bitmap_paths(width, height, &mask, 0.5, 0.5).expect("trace");
    assert_eq!(segments.iter().filter(|segment| matches!(segment, PathSegment::Move { .. })).count(), 1);
    assert!(segments.len() <= 6, "a solid square must trace its boundary without interior scanlines");
}

#[test]
fn traces_disjoint_regions_with_move_per_contour() {
    let width = 10_u32;
    let height = 10_u32;
    let mut mask = vec![0_u8; (width * height) as usize];
    for y in 1..3 {
        for x in 1..3 {
            mask[(y * width + x) as usize] = 255;
        }
    }
    for y in 6..8 {
        for x in 6..8 {
            mask[(y * width + x) as usize] = 255;
        }
    }
    let segments = trace_bitmap_paths(width, height, &mask, 0.5, 0.5).expect("trace");
    let moves = segments.iter().filter(|segment| matches!(segment, PathSegment::Move { .. })).count();
    assert!(moves >= 2, "each disjoint contour must start with its own move");
}

#[test]
fn traces_corner_touching_regions_as_separate_contours() {
    let mask = [255_u8, 0, 0, 255];
    let segments = trace_bitmap_paths(2, 2, &mask, 0.5, 0.0).expect("trace");
    assert_eq!(segments.iter().filter(|segment| matches!(segment, PathSegment::Move { .. })).count(), 2);
}

#[test]
fn trace_bitmap_errors_on_zero_dimensions() {
    let err = trace_bitmap_paths(0, 5, &[], 0.5, 0.5).unwrap_err();
    assert!(matches!(err, DrawingError::InvalidInput(_)));
}

#[test]
fn trace_bitmap_errors_on_short_buffer() {
    let err = trace_bitmap_paths(4, 4, &[0_u8; 2], 0.5, 0.5).unwrap_err();
    assert!(matches!(err, DrawingError::InvalidInput(message) if message.contains("expects")));
}

#[test]
fn trace_bitmap_errors_when_no_pixels_above_threshold() {
    let mask = vec![0_u8; 16];
    let err = trace_bitmap_paths(4, 4, &mask, 0.5, 0.5).unwrap_err();
    assert!(matches!(err, DrawingError::Operation(message) if message.contains("no contours")));
}

#[test]
fn douglas_peucker_returns_points_unchanged_when_epsilon_is_non_positive() {
    let points: Vec<Vec2> = vec![[0.0, 0.0], [1.0, 5.0], [2.0, 0.0]];
    assert_eq!(douglas_peucker(&points, 0.0), points);
}

#[test]
fn perpendicular_distance_handles_degenerate_zero_length_line() {
    let dist = perpendicular_distance([3.0, 4.0], [0.0, 0.0], [0.0, 0.0]);
    assert!((dist - 5.0).abs() < 1e-9);
}
