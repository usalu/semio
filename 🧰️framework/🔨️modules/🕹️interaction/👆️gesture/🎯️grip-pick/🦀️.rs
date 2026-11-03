//! 🎯️ Grip pick: the pure screen-space law that makes a draggable marker of a 3D viewport pickable by a mouse and by a finger. A marker is a point annotation with a drag line; the host projects it, picks the nearest one inside a per-device hit radius, and while the grip is held turns the cursor ray into the parameter along the line. Domain-neutral: a grip knows its position on the screen and its line in the world, nothing about what the parameter means.
//!
//! 🔗️ Replayed on the language-agnostic fixture `🎯️grip-pick.json` (expectations of the line law computed with numpy's least squares); the TypeScript twin lives in `🟦️.ts`; the hit edges follow WCAG 2.5.8 (24 px pointer) and 2.5.5 (44 px touch).

pub const TOUCH_HIT_EDGE_PX: f64 = 44.0;
pub const POINTER_HIT_EDGE_PX: f64 = 24.0;

/// 🖱️ Which kind of pointer presses: a finger needs the larger target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GripDevice {
    Pointer,
    Touch,
}

/// 📏️ The hit edge in pixels of a grip for `device`: 44 for a finger, 24 for a mouse or pen, raised to the device profile's own minimum (`profile_min_px`, 0 keeps the default).
pub fn grip_hit_edge_px(device: GripDevice, profile_min_px: f64) -> f64 {
    let base = match device {
        GripDevice::Touch => TOUCH_HIT_EDGE_PX,
        GripDevice::Pointer => POINTER_HIT_EDGE_PX,
    };
    if profile_min_px.is_finite() { base.max(profile_min_px) } else { base }
}

/// ⭕️ The pick radius in pixels: half the hit edge, never smaller than the drawn radius of the grip, so a grip is never harder to hit than it looks.
pub fn grip_hit_radius_px(device: GripDevice, profile_min_px: f64, drawn_radius_px: f64) -> f64 {
    (grip_hit_edge_px(device, profile_min_px) / 2.0).max(if drawn_radius_px.is_finite() { drawn_radius_px } else { 0.0 })
}

/// 🎯️ The index of the grip nearest to `at` (screen pixels) that lies within `radius` pixels, the first of equally near ones; `None` when no grip is in reach. Points that are not finite never hit.
pub fn pick_grip(points: &[[f64; 2]], at: [f64; 2], radius: f64) -> Option<usize> {
    points
        .iter()
        .enumerate()
        .filter(|(_, point)| point[0].is_finite() && point[1].is_finite())
        .map(|(index, point)| (index, (point[0] - at[0]).hypot(point[1] - at[1])))
        .filter(|(_, distance)| *distance <= radius)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(index, _)| index)
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// 📐️ The parameter `t` of the point `origin + t · axis` of a line that lies nearest to the cursor ray `ray_origin + u · ray_dir` (the closest points of two lines); `None` when the ray runs parallel to the line or an input is not finite, so a grip seen end-on does not jump.
pub fn line_parameter(ray_origin: [f64; 3], ray_dir: [f64; 3], origin: [f64; 3], axis: [f64; 3]) -> Option<f64> {
    if ![ray_origin, ray_dir, origin, axis].iter().flatten().all(|v| v.is_finite()) {
        return None;
    }
    let w = [origin[0] - ray_origin[0], origin[1] - ray_origin[1], origin[2] - ray_origin[2]];
    let (a, b, c, d, e) = (dot(axis, axis), dot(axis, ray_dir), dot(ray_dir, ray_dir), dot(axis, w), dot(ray_dir, w));
    let denominator = a * c - b * b;
    (denominator.abs() > 1.0e-12 * a * c && a > 0.0 && c > 0.0).then(|| (b * e - c * d) / denominator)
}

/// ✋️ The drag parameter while a grip is held: the parameter `anchor_s` of the grip plus how far the cursor moved along the line since the press (`cursor_t - press_t`), so the grip follows the finger without jumping to it.
pub fn grip_parameter(anchor_s: f64, press_t: f64, cursor_t: f64) -> f64 {
    anchor_s + (cursor_t - press_t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Fixture {
        hit_edge: Vec<Edge>,
        pick: Vec<Pick>,
        line: Vec<Line>,
        drag: Vec<Drag>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Edge {
        device: String,
        profile_min: f64,
        expect: f64,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Pick {
        name: String,
        device: String,
        profile_min: f64,
        drawn: f64,
        points: Vec<[f64; 2]>,
        at: [f64; 2],
        expect: Option<usize>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Line {
        name: String,
        ray_origin: [f64; 3],
        ray_dir: [f64; 3],
        origin: [f64; 3],
        axis: [f64; 3],
        expect: Option<f64>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Drag {
        anchor_s: f64,
        press_t: f64,
        cursor_t: f64,
        expect: f64,
    }

    fn device(name: &str) -> GripDevice {
        if name == "touch" { GripDevice::Touch } else { GripDevice::Pointer }
    }

    fn fixture() -> Fixture {
        serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🎯️grip-pick.json")).expect("grip pick fixture")
    }

    #[test]
    fn the_hit_edge_is_44_pixels_for_a_finger_and_24_for_a_pointer_and_a_profile_only_raises_it() {
        for row in fixture().hit_edge {
            assert_eq!(grip_hit_edge_px(device(&row.device), row.profile_min), row.expect, "{} with profile minimum {}", row.device, row.profile_min);
        }
    }

    #[test]
    fn the_language_agnostic_pick_rows_replay_to_the_same_grips() {
        for row in fixture().pick {
            let radius = grip_hit_radius_px(device(&row.device), row.profile_min, row.drawn);
            assert_eq!(pick_grip(&row.points, row.at, radius), row.expect, "{}", row.name);
        }
    }

    #[test]
    fn the_line_parameter_equals_the_least_squares_solution_computed_with_numpy() {
        for row in fixture().line {
            let got = line_parameter(row.ray_origin, row.ray_dir, row.origin, row.axis);
            match (got, row.expect) {
                (Some(got), Some(expect)) => assert!((got - expect).abs() < 1.0e-9, "{}: {got} vs {expect}", row.name),
                (got, expect) => assert_eq!(got, expect, "{}", row.name),
            }
        }
    }

    #[test]
    fn the_grip_follows_the_cursor_by_the_distance_it_moved_since_the_press() {
        for row in fixture().drag {
            assert!((grip_parameter(row.anchor_s, row.press_t, row.cursor_t) - row.expect).abs() < 1.0e-12);
        }
    }

    #[test]
    fn a_grip_that_is_not_finite_or_out_of_reach_is_never_picked() {
        assert_eq!(pick_grip(&[[f64::NAN, 0.0], [500.0, 500.0]], [0.0, 0.0], 22.0), None);
        assert_eq!(pick_grip(&[], [0.0, 0.0], 22.0), None);
    }
}
