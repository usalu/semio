//! 🔍️ Bitmap autotrace: marching squares contours + Douglas-Peucker simplification.

use crate::engine::{DrawingError, PathSegment, Vec2};
use std::collections::HashMap;

fn pixel_on(mask: &[u8], width: u32, x: i32, y: i32, threshold: u8) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    let ux = x as u32;
    let uy = y as u32;
    if ux >= width {
        return false;
    }
    let idx = (uy as usize) * (width as usize) + (ux as usize);
    mask.get(idx).copied().unwrap_or(0) >= threshold
}

fn marching_squares_contours(mask: &[u8], width: u32, height: u32, threshold: u8) -> Vec<Vec<Vec2>> {
    fn direction(start: [i32; 2], end: [i32; 2]) -> i32 {
        match [end[0] - start[0], end[1] - start[1]] {
            [1, 0] => 0,
            [0, 1] => 1,
            [-1, 0] => 2,
            _ => 3,
        }
    }

    let w = width as i32;
    let h = height as i32;
    let mut edges: Vec<([i32; 2], [i32; 2])> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if !pixel_on(mask, width, x, y, threshold) {
                continue;
            }
            if !pixel_on(mask, width, x, y - 1, threshold) {
                edges.push(([x, y], [x + 1, y]));
            }
            if !pixel_on(mask, width, x + 1, y, threshold) {
                edges.push(([x + 1, y], [x + 1, y + 1]));
            }
            if !pixel_on(mask, width, x, y + 1, threshold) {
                edges.push(([x + 1, y + 1], [x, y + 1]));
            }
            if !pixel_on(mask, width, x - 1, y, threshold) {
                edges.push(([x, y + 1], [x, y]));
            }
        }
    }

    let mut outgoing: HashMap<[i32; 2], Vec<usize>> = HashMap::new();
    for (index, (start, _)) in edges.iter().enumerate() {
        outgoing.entry(*start).or_default().push(index);
    }
    let mut used = vec![false; edges.len()];
    let mut contours: Vec<Vec<Vec2>> = Vec::new();
    for first in 0..edges.len() {
        if used[first] {
            continue;
        }
        let start = edges[first].0;
        let mut edge_index = first;
        let mut contour = vec![[start[0] as f64, start[1] as f64]];
        loop {
            used[edge_index] = true;
            let end = edges[edge_index].1;
            if end == start {
                break;
            }
            contour.push([end[0] as f64, end[1] as f64]);
            let incoming_direction = direction(edges[edge_index].0, end);
            let Some(next) = outgoing.get(&end).and_then(|indices| {
                indices.iter().copied().filter(|index| !used[*index]).min_by_key(|index| match (direction(end, edges[*index].1) - incoming_direction).rem_euclid(4) {
                    1 => 0,
                    0 => 1,
                    3 => 2,
                    _ => 3,
                })
            }) else {
                contour.clear();
                break;
            };
            edge_index = next;
        }
        if contour.len() >= 3 {
            contours.push(contour);
        }
    }
    contours
}

fn perpendicular_distance(point: Vec2, line_start: Vec2, line_end: Vec2) -> f64 {
    let dx = line_end[0] - line_start[0];
    let dy = line_end[1] - line_start[1];
    if dx.abs() < f64::EPSILON && dy.abs() < f64::EPSILON {
        let px = point[0] - line_start[0];
        let py = point[1] - line_start[1];
        return (px * px + py * py).sqrt();
    }
    let t = ((point[0] - line_start[0]) * dx + (point[1] - line_start[1]) * dy) / (dx * dx + dy * dy);
    let proj = [line_start[0] + t * dx, line_start[1] + t * dy];
    let ox = point[0] - proj[0];
    let oy = point[1] - proj[1];
    (ox * ox + oy * oy).sqrt()
}

fn douglas_peucker(points: &[Vec2], epsilon: f64) -> Vec<Vec2> {
    if points.len() < 3 || epsilon <= 0.0 {
        return points.to_vec();
    }
    let start = points[0];
    let end = points[points.len() - 1];
    let mut max_dist = 0.0_f64;
    let mut index = 0_usize;
    for (i, point) in points.iter().enumerate().skip(1).take(points.len().saturating_sub(2)) {
        let dist = perpendicular_distance(*point, start, end);
        if dist > max_dist {
            max_dist = dist;
            index = i;
        }
    }
    if max_dist > epsilon {
        let mut left = douglas_peucker(&points[..=index], epsilon);
        let right = douglas_peucker(&points[index..], epsilon);
        left.pop();
        left.extend(right);
        left
    } else {
        vec![start, end]
    }
}

fn contour_to_segments(points: &[Vec2]) -> Vec<PathSegment> {
    if points.is_empty() {
        return Vec::new();
    }
    let mut segments = vec![PathSegment::Move { to: points[0] }];
    for point in points.iter().skip(1) {
        segments.push(PathSegment::Line { to: *point });
    }
    if points.len() > 2 {
        segments.push(PathSegment::Close);
    }
    segments
}

pub fn trace_bitmap_paths(width: u32, height: u32, mask_or_luma: &[u8], threshold: f64, simplify_epsilon: f64) -> Result<Vec<PathSegment>, DrawingError> {
    if width == 0 || height == 0 {
        return Err(DrawingError::InvalidInput("trace bitmap needs non-zero dimensions".into()));
    }
    let expected = (width as usize).saturating_mul(height as usize);
    if mask_or_luma.len() < expected {
        return Err(DrawingError::InvalidInput(format!("trace bitmap expects {expected} bytes, got {}", mask_or_luma.len())));
    }
    let threshold_u8 = (threshold.clamp(0.0, 1.0) * 255.0).round() as u8;
    let contours = marching_squares_contours(&mask_or_luma[..expected], width, height, threshold_u8);
    if contours.is_empty() {
        return Err(DrawingError::Operation("trace produced no contours".into()));
    }
    let mut segments: Vec<PathSegment> = Vec::new();
    for contour in contours {
        let simplified = douglas_peucker(&contour, simplify_epsilon.max(0.0));
        segments.extend(contour_to_segments(&simplified));
    }
    if segments.is_empty() {
        return Err(DrawingError::Operation("trace produced no segments".into()));
    }
    Ok(segments)
}

// #region 🧪️Tests
#[cfg(test)]
mod tests {
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
}
// #endregion 🧪️Tests
