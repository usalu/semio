//! 📐️ Pose math: the vector and quaternion helpers, the vortex full id and the attraction pose derivation.

use super::*;

/// ✖️ The Hamilton product `a · b` of two `[x, y, z, w]` quaternions — `a` applied after `b`.
pub fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
}

/// 🧭️ The `[x, y, z, w]` quaternion turning `angle` radians about the axis `(ax, ay, az)`; the identity
/// for an axis shorter than `1e-8`.
pub fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let half = angle * 0.5;
    let s = half.sin();
    [ax / len * s, ay / len * s, az / len * s, half.cos()]
}

/// 📏️ A pose scale multiplied per axis by `factors`, always as a per-axis triple: a uniform scalar
/// broadcasts first and an absent scale reads as `[1, 1, 1]`.
pub fn puzzle3d_scaled(scale: Option<crate::Puzzle3dScale>, factors: [f64; 3]) -> crate::Puzzle3dScale {
    let current = match scale {
        Some(crate::Puzzle3dScale::Uniform(value)) => [value; 3],
        Some(crate::Puzzle3dScale::Vec3(value)) => value,
        None => [1.0; 3],
    };
    crate::Puzzle3dScale::Vec3([current[0] * factors[0], current[1] * factors[1], current[2] * factors[2]])
}

/// ⭕️ The quaternion of no rotation, `[x, y, z, w]` — what a record without an orientation stands at.
pub const PUZZLE3D_IDENTITY_QUATERNION: [f64; 4] = [0.0, 0.0, 0.0, 1.0];

/// 🪡️ Below this cross length an attracted vortex counts as (anti)parallel to its attracting one — the compose
/// kernel's own alignment tolerance.
const PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE: f64 = 0.01;

/// 🧭️ The pose fields a selection transform may change on one object — its world origin, orientation and scale.
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle3dPose {
    pub origin: [f64; 3],
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<crate::Puzzle3dScale>,
}

/// 🔗️ The full id an attraction endpoint names one vortex by: `<objectId>:<vortexId>`, or the vortex id itself
/// when it already is a full id.
pub fn puzzle3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
    if vortex_id.contains(':') {
        vortex_id.to_string()
    } else {
        format!("{object_id}:{vortex_id}")
    }
}

/// ➖️ `a − b`.
pub fn vec3_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// ➕️ `a + b`.
pub fn vec3_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// ✳️ `a · s`.
pub fn vec3_scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

/// ❎️ The cross product `a × b`.
pub fn vec3_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// ⚫️ The dot product `a · b`.
pub fn vec3_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// 🦯️ The Euclidean length of `a`.
pub fn vec3_len(a: [f64; 3]) -> f64 {
    vec3_dot(a, a).sqrt()
}

/// 🎱️ `a` scaled to unit length; a vector shorter than `1e-12` stays as it is.
pub fn vec3_normalize(a: [f64; 3]) -> [f64; 3] {
    let len = vec3_len(a);
    if len < 1e-12 {
        a
    } else {
        vec3_scale(a, 1.0 / len)
    }
}

/// 🔁️ Degrees to radians.
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

/// 🔂️ Radians to degrees.
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

/// 🪞️ The conjugate of `q` — its inverse for a unit quaternion.
pub fn quat_conjugate(q: [f64; 4]) -> [f64; 4] {
    [-q[0], -q[1], -q[2], q[3]]
}

/// 🧼️ `q` scaled to unit length; a degenerate quaternion reads as the identity.
pub fn quat_normalize(q: [f64; 4]) -> [f64; 4] {
    let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if len < 1e-12 {
        PUZZLE3D_IDENTITY_QUATERNION
    } else {
        [q[0] / len, q[1] / len, q[2] / len, q[3] / len]
    }
}

/// 🌐️ `vector` turned by the unit quaternion `quat` (`q · v · q⁻¹`).
pub fn quat_rotate_vector(quat: [f64; 4], vector: [f64; 3]) -> [f64; 3] {
    let [x, y, z, w] = quat;
    let vx = vector[0];
    let vy = vector[1];
    let vz = vector[2];
    let ix = w * vx + y * vz - z * vy;
    let iy = w * vy + z * vx - x * vz;
    let iz = w * vz + x * vy - y * vx;
    let iw = -x * vx - y * vy - z * vz;
    [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x]
}

/// 🎏️ The quaternion rotating unit vector `from` onto unit vector `to`.
pub fn puzzle3d_quaternion_from_unit_vectors(from: [f64; 3], to: [f64; 3]) -> [f64; 4] {
    let r = vec3_dot(from, to) + 1.0;
    let quat = if r < 0.000_001 {
        if from[0].abs() > from[2].abs() {
            [-from[1], from[0], 0.0, 0.0]
        } else {
            [0.0, -from[2], from[1], 0.0]
        }
    } else {
        let c = vec3_cross(from, to);
        [c[0], c[1], c[2], r]
    };
    quat_normalize(quat)
}

/// 🧲️ The align-quaternion special case for when the attracted vortex is already (anti)parallel to the
/// attracting vortex. Falls back to an alternate cross axis when the attracting direction is exactly
/// ±Z — a double-degenerate corner the compose kernel's own branch doesn't otherwise guard.
pub fn puzzle3d_attraction_align_quat(parent_dir: [f64; 3], child_dir: [f64; 3]) -> [f64; 4] {
    let reverse_child = vec3_scale(child_dir, -1.0);
    let cross_vec = vec3_cross(parent_dir, reverse_child);
    if vec3_len(cross_vec) < PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE {
        if parent_dir[2].abs() < PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE {
            puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], [0.0, 0.0, -1.0])
        } else {
            let mut axis = vec3_cross([0.0, 0.0, 1.0], parent_dir);
            if vec3_len(axis) < 1e-9 {
                axis = vec3_cross([1.0, 0.0, 0.0], parent_dir);
            }
            let axis = vec3_normalize(axis);
            let half = std::f64::consts::FRAC_PI_2;
            quat_normalize([axis[0] * half.sin(), axis[1] * half.sin(), axis[2] * half.sin(), half.cos()])
        }
    } else {
        puzzle3d_quaternion_from_unit_vectors(reverse_child, parent_dir)
    }
}

/// 📐️ Forward attraction placement — given the attracting object's world pose (`t_a`/`q_a`), both
/// vortices' LOCAL position/direction, and the 6 connection-style parameters (angles in degrees),
/// returns the attracted object's world pose.
#[allow(clippy::too_many_arguments)]
pub fn puzzle3d_attraction_child_pose(t_a: [f64; 3], q_a: [f64; 4], p_a: [f64; 3], d_a: [f64; 3], p_b: [f64; 3], d_b: [f64; 3], gap: f64, shift: f64, rise: f64, rotation_deg: f64, turn_deg: f64, tilt_deg: f64) -> ([f64; 3], [f64; 4]) {
    let parent_dir = vec3_normalize(d_a);
    let child_dir = vec3_normalize(d_b);
    let align_q = puzzle3d_attraction_align_quat(parent_dir, child_dir);

    let pq = puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir);
    let gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0]);
    let shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0]);
    let raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0]);

    let rotate_q = quat_from_axis_angle(parent_dir[0], parent_dir[1], parent_dir[2], -deg_to_rad(rotation_deg));
    let turn_axis = quat_rotate_vector(rotate_q, raise_dir);
    let tilt_axis = quat_rotate_vector(rotate_q, shift_dir);
    let turn_q = quat_from_axis_angle(turn_axis[0], turn_axis[1], turn_axis[2], deg_to_rad(turn_deg));
    let tilt_q = quat_from_axis_angle(tilt_axis[0], tilt_axis[1], tilt_axis[2], deg_to_rad(tilt_deg));

    let mut orientation_local = quat_conjugate(align_q);
    orientation_local = quat_mul(orientation_local, quat_conjugate(rotate_q));
    orientation_local = quat_mul(orientation_local, quat_conjugate(turn_q));
    orientation_local = quat_mul(orientation_local, quat_conjugate(tilt_q));
    let orientation_local = quat_normalize(orientation_local);

    let offset = vec3_add(vec3_add(t_a, p_a), vec3_add(vec3_add(vec3_scale(gap_dir, gap), vec3_scale(shift_dir, shift)), vec3_scale(raise_dir, rise)));
    let t_b = vec3_sub(quat_rotate_vector(orientation_local, offset), p_b);
    let q_b = quat_normalize(quat_mul(orientation_local, q_a));
    (t_b, q_b)
}

/// 🔙️ Inverse of `puzzle3d_attraction_child_pose` — given the attracted object's CURRENT world pose,
/// derives the 6 parameters that reproduce it exactly, so moving/rotating an attracted object never
/// causes a resolve-triggered snap-back and creating an attraction never moves either endpoint.
#[allow(clippy::too_many_arguments)]
pub fn derive_attraction_params(t_a: [f64; 3], q_a: [f64; 4], p_a: [f64; 3], d_a: [f64; 3], p_b: [f64; 3], d_b: [f64; 3], t_b: [f64; 3], q_b: [f64; 4]) -> (f64, f64, f64, f64, f64, f64) {
    let parent_dir = vec3_normalize(d_a);
    let child_dir = vec3_normalize(d_b);
    let align_q = puzzle3d_attraction_align_quat(parent_dir, child_dir);
    let pq = puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir);
    let gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0]);
    let shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0]);
    let raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0]);

    let orientation_local = quat_normalize(quat_mul(q_b, quat_conjugate(q_a)));

    let offset = quat_rotate_vector(quat_conjugate(orientation_local), vec3_add(t_b, p_b));
    let diff = vec3_sub(vec3_sub(offset, t_a), p_a);
    let gap = vec3_dot(diff, gap_dir);
    let shift = vec3_dot(diff, shift_dir);
    let rise = vec3_dot(diff, raise_dir);

    let residual = quat_mul(align_q, orientation_local);
    let m = quat_mul(quat_mul(quat_conjugate(pq), residual), pq);
    let col_x = quat_rotate_vector(m, [1.0, 0.0, 0.0]);
    let col_y = quat_rotate_vector(m, [0.0, 1.0, 0.0]);

    let clamp = |v: f64| v.clamp(-1.0, 1.0);
    let tilt_rad = -(clamp(col_y[2])).asin();
    let (rotation_rad, turn_rad) = if (col_y[2].abs() - 1.0).abs() < 1e-6 {
        (col_x[1].atan2(col_x[0]), 0.0)
    } else {
        let col_z = quat_rotate_vector(m, [0.0, 0.0, 1.0]);
        ((-col_x[2]).atan2(col_z[2]), col_y[0].atan2(col_y[1]))
    };

    (gap, shift, rise, rad_to_deg(rotation_rad), rad_to_deg(turn_rad), rad_to_deg(tilt_rad))
}
