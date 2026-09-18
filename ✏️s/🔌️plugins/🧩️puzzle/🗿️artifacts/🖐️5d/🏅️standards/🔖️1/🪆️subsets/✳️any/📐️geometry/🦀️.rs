//! @emoji 📐️ The 5D flatten solver — the Rust twin of the React target
//! `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/🟦️.tsx`'s `🔖️Flatten` region, ported rule for rule.
//!
//! That 638-line file is the whole 5D visualisation layer React has: a pure geometry module (no JSX,
//! no canvas) that walks the fastener graph, poses every part from its parent's grip frame, and lays
//! the same graph out as a 2-D topology diagram. Until ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`
//! packet W2k there was NO Rust counterpart anywhere in the tree (`Puzzle5dFastener`'s own docstring
//! names `compose geom::flatten`, which did not exist), so the wgpu renderer's `Board2d`/`MeshWindow`
//! projections could only ever paint the AUTHORED poses while React painted solved ones.
//!
//! Typed over [`Puzzle5dSnapshot`] rather than over JSON records: React composes from two untyped
//! fixtures because the sketchpad bridge hands it raw JSON, whereas every Rust caller already holds
//! the schema. Everything downstream of that boundary — the tolerance, the diagram constants, the
//! BFS root order, the degenerate-direction branches, the matrix order — is React's.

use crate::{Puzzle5dFastener, Puzzle5dGrip, Puzzle5dPart, Puzzle5dPartAnchor, Puzzle5dSnapshot};
use std::collections::{HashMap, HashSet, VecDeque};

//#region 📐️Constants

/// 📐️ Sketchpad topology diagram icon width — React's `PUZZLE_5D_TOPOLOGY_ICON_WIDTH` (`🟦️.tsx:44`).
/// [`prepare_topology_poses`] multiplies the diagram centres by it to land in board pixels.
pub const PUZZLE_5D_TOPOLOGY_ICON_WIDTH: f64 = 48.0;

/// 📐️ React's `TOLERANCE` (`🟦️.tsx:188`) — the cross-product length under which two grip directions
/// count as collinear and the alignment quaternion takes a degenerate branch.
const TOLERANCE: f64 = 0.01;

/// 📐️ React's `DIAGRAM_RADIUS` (`🟦️.tsx:189`) — the circle a first child is placed on.
const DIAGRAM_RADIUS: f64 = 2.697;

/// 📐️ React's `DIAGRAM_VERTICAL_V_EXTRA` (`🟦️.tsx:190`) — the extra v a vertical parent grip adds.
const DIAGRAM_VERTICAL_V_EXTRA: f64 = 1.0;

/// 📐️ React's `DIAGRAM_HORIZONTAL_SCALE` (`🟦️.tsx:191`) — the u/v scale a horizontal grip uses.
const DIAGRAM_HORIZONTAL_SCALE: f64 = 3.0633;

//#endregion 📐️Constants

//#region 📐️Types

/// 📐️ A part's solved frame — React's `FlattenPlane` (`🟦️.tsx:193`). `x_axis`/`y_axis` are unit-ish
/// rows of the frame; the implied z is their cross product, exactly as `planeToMatrix` derives it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlattenPlane {
    pub origin: [f64; 3],
    pub x_axis: [f64; 3],
    pub y_axis: [f64; 3],
}

impl FlattenPlane {
    /// 📐️ The identity XY plane a derived root and every degenerate branch fall back to
    /// (`🟦️.tsx:497`).
    pub const IDENTITY: Self = Self { origin: [0.0, 0.0, 0.0], x_axis: [1.0, 0.0, 0.0], y_axis: [0.0, 1.0, 0.0] };
}

impl Default for FlattenPlane {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// 📐️ One part's solved pose — React's `FlattenPose` (`🟦️.tsx:194`): the 3-D frame, the 2-D diagram
/// centre, and the frame as a quaternion for the instance stream a mesh window uploads.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlattenPose {
    pub plane: FlattenPlane,
    pub center: [f64; 2],
    pub orientation: [f64; 4],
}

/// 📐️ The eight transform parameters plus the two diagram offsets one fastener contributes — React's
/// `FlatAttraction` (`🟦️.tsx:196-207`), read off the typed [`Puzzle5dFastener`] instead of a record.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Attraction {
    pub gap: f64,
    pub shift: f64,
    pub rise: f64,
    pub rotation: f64,
    pub turn: f64,
    pub tilt: f64,
    pub x: f64,
    pub y: f64,
}

impl Attraction {
    /// 🔗️ The transform one typed fastener contributes.
    pub fn of(fastener: &Puzzle5dFastener) -> Self {
        Self { gap: fastener.gap, shift: fastener.shift, rise: fastener.rise, rotation: fastener.rotation, turn: fastener.turn, tilt: fastener.tilt, x: fastener.x, y: fastener.y }
    }
}

//#endregion 📐️Types

//#region 📐️Vectors

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if length <= 0.0 {
        return [0.0, 0.0, 1.0];
    }
    [v[0] / length, v[1] / length, v[2] / length]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// 📐️ React's `roundF` (`🟦️.tsx:235`) — the diagram centres are rounded to 1e-6 so a board fixture
/// stays byte-stable across runs.
fn round_micro(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}

//#endregion 📐️Vectors

//#region 📐️Matrices
// 🧮️ Column-major `[f64; 16]`, the exact layout React's own `number[]` uses, so `mul_mat`'s index
// arithmetic is transcribed rather than re-derived.

fn plane_to_matrix(plane: &FlattenPlane) -> [f64; 16] {
    let x = plane.x_axis;
    let y = plane.y_axis;
    let z = cross(x, y);
    [x[0], x[1], x[2], 0.0, y[0], y[1], y[2], 0.0, z[0], z[1], z[2], 0.0, plane.origin[0], plane.origin[1], plane.origin[2], 1.0]
}

fn matrix_to_plane(m: [f64; 16]) -> FlattenPlane {
    FlattenPlane { origin: [m[12], m[13], m[14]], x_axis: [m[0], m[1], m[2]], y_axis: [m[4], m[5], m[6]] }
}

fn mul_mat(a: [f64; 16], b: [f64; 16]) -> [f64; 16] {
    let mut out = [0.0; 16];
    for col in 0..4 {
        for row in 0..4 {
            out[col * 4 + row] = a[row] * b[col * 4] + a[4 + row] * b[col * 4 + 1] + a[8 + row] * b[col * 4 + 2] + a[12 + row] * b[col * 4 + 3];
        }
    }
    out
}

fn translation(x: f64, y: f64, z: f64) -> [f64; 16] {
    [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, z, 1.0]
}

/// 🔄️ Rodrigues rotation about `axis` — React's `rotationAxis` (`🟦️.tsx:265`).
fn rotation_axis(axis: [f64; 3], angle: f64) -> [f64; 16] {
    let [x, y, z] = axis;
    let c = angle.cos();
    let s = angle.sin();
    let t = 1.0 - c;
    [
        t * x * x + c,
        t * x * y + s * z,
        t * x * z - s * y,
        0.0,
        t * x * y - s * z,
        t * y * y + c,
        t * y * z + s * x,
        0.0,
        t * x * z + s * y,
        t * y * z - s * x,
        t * z * z + c,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]
}

fn apply_mat_vec3(m: [f64; 16], v: [f64; 3]) -> [f64; 3] {
    [m[0] * v[0] + m[4] * v[1] + m[8] * v[2], m[1] * v[0] + m[5] * v[1] + m[9] * v[2], m[2] * v[0] + m[6] * v[1] + m[10] * v[2]]
}

/// 🔄️ The shortest-arc quaternion taking `from` onto `to` — React's `quaternionFromUnitVectors`
/// (`🟦️.tsx:294`), antiparallel branch included.
fn quaternion_from_unit_vectors(from: [f64; 3], to: [f64; 3]) -> [f64; 4] {
    let r = dot(from, to) + 1.0;
    let q = if r < 0.000_001 {
        if from[0].abs() > from[2].abs() {
            [-from[1], from[0], 0.0, 0.0]
        } else {
            [0.0, -from[2], from[1], 0.0]
        }
    } else {
        let c = cross(from, to);
        [c[0], c[1], c[2], r]
    };
    let length = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    [q[0] / length, q[1] / length, q[2] / length, q[3] / length]
}

fn quaternion_to_matrix(q: [f64; 4]) -> [f64; 16] {
    let [x, y, z, w] = q;
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;
    [1.0 - (yy + zz), xy + wz, xz - wy, 0.0, xy - wz, 1.0 - (xx + zz), yz + wx, 0.0, xz + wy, yz - wx, 1.0 - (xx + yy), 0.0, 0.0, 0.0, 0.0, 1.0]
}

/// 🔄️ A frame back to a quaternion — React's `planeToOrientation` (`🟦️.tsx:342`), all four
/// trace branches.
fn plane_to_orientation(plane: &FlattenPlane) -> [f64; 4] {
    let [xx, xy, xz] = plane.x_axis;
    let [yx, yy, yz] = plane.y_axis;
    let zx = xy * yz - xz * yy;
    let zy = xz * yx - xx * yz;
    let zz = xx * yy - xy * yx;
    let (m00, m01, m02) = (xx, yx, zx);
    let (m10, m11, m12) = (xy, yy, zy);
    let (m20, m21, m22) = (xz, yz, zz);
    let trace = m00 + m11 + m22;
    if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        return [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s];
    }
    if m00 > m11 && m00 > m22 {
        let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0;
        return [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s];
    }
    if m11 > m22 {
        let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0;
        return [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s];
    }
    let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0;
    [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s]
}

/// 🔄️ A stored origin + quaternion as a frame — React's `orientationToPlane` (`🟦️.tsx:378`).
fn orientation_to_plane(origin: [f64; 3], orientation: [f64; 4]) -> FlattenPlane {
    let m = quaternion_to_matrix(orientation);
    FlattenPlane { origin, x_axis: [m[0], m[1], m[2]], y_axis: [m[4], m[5], m[6]] }
}

//#endregion 📐️Matrices

//#region 📐️Solver

/// 🔗️ `"part:grip"` split in two — React's `parseEndpoint` (`🟦️.tsx:383`). A fastener endpoint with
/// no colon is not addressable and drops the whole hop to the identity plane.
fn parse_endpoint(endpoint: &str) -> Option<(&str, &str)> {
    endpoint.split_once(':')
}

/// 🗺️ One child's 2-D diagram centre — React's `diagramCenter` (`🟦️.tsx:389`). A parent still at the
/// origin seeds its children around a circle by the grip's rim parameter; otherwise the fastener's
/// own `x`/`y` offset is scaled by whether the parent grip points mostly along z.
fn diagram_center(parent_center: [f64; 2], parent_direction: [f64; 3], parent_t: f64, attraction: &Attraction) -> [f64; 2] {
    let (child_x, child_y) = if parent_center[0] == 0.0 && parent_center[1] == 0.0 {
        let angle = 2.0 * std::f64::consts::PI * parent_t;
        (DIAGRAM_RADIUS * angle.sin(), DIAGRAM_RADIUS * angle.cos())
    } else if parent_direction[2].abs() > 0.5 {
        (parent_center[0] + attraction.x, parent_center[1] + attraction.y + DIAGRAM_VERTICAL_V_EXTRA)
    } else {
        (parent_center[0] + attraction.x * DIAGRAM_HORIZONTAL_SCALE, parent_center[1] + attraction.y * DIAGRAM_HORIZONTAL_SCALE)
    };
    [round_micro(child_x), round_micro(child_y)]
}

/// 📐️ One child's solved frame from its parent's — React's `computeChildPlane` (`🟦️.tsx:408`),
/// transcribed in the same order: align the reversed child direction onto the parent's, build the
/// parent's own gap/shift/rise basis, spin about the parent direction, then turn and tilt, then
/// compose `parent · T(point) · T(rise) · T(shift) · T(gap) · orientation · T(-childPoint)`.
pub fn compute_child_plane(parent_plane: &FlattenPlane, parent_point: [f64; 3], parent_dir: [f64; 3], child_point: [f64; 3], child_dir: [f64; 3], attraction: &Attraction) -> FlattenPlane {
    let parent_matrix = plane_to_matrix(parent_plane);
    let p_dir = normalize(parent_dir);
    let c_dir = normalize(child_dir);
    let rotation_rad = attraction.rotation.to_radians();
    let turn_rad = attraction.turn.to_radians();
    let tilt_rad = attraction.tilt.to_radians();
    let reverse_child = [-c_dir[0], -c_dir[1], -c_dir[2]];
    let cross_vec = cross(p_dir, reverse_child);
    let cross_len = (cross_vec[0] * cross_vec[0] + cross_vec[1] * cross_vec[1] + cross_vec[2] * cross_vec[2]).sqrt();
    let align_quat = if cross_len < TOLERANCE {
        if dot(p_dir, reverse_child) > 0.0 {
            [0.0, 0.0, 0.0, 1.0]
        } else if p_dir[2].abs() < TOLERANCE {
            quaternion_from_unit_vectors([0.0, 1.0, 0.0], [0.0, 0.0, -1.0])
        } else {
            [1.0, 0.0, 0.0, 0.0]
        }
    } else {
        quaternion_from_unit_vectors(reverse_child, p_dir)
    };
    let direction_t = quaternion_to_matrix(align_quat);
    let parent_rotation_t = quaternion_to_matrix(quaternion_from_unit_vectors([0.0, 1.0, 0.0], p_dir));
    let gap_direction = apply_mat_vec3(parent_rotation_t, [0.0, 1.0, 0.0]);
    let shift_direction = apply_mat_vec3(parent_rotation_t, [1.0, 0.0, 0.0]);
    let raise_direction = apply_mat_vec3(parent_rotation_t, [0.0, 0.0, 1.0]);
    let mut turn_axis = apply_mat_vec3(parent_rotation_t, [0.0, 0.0, 1.0]);
    let mut tilt_axis = apply_mat_vec3(parent_rotation_t, [1.0, 0.0, 0.0]);
    let rotate_t = rotation_axis(p_dir, -rotation_rad);
    let mut orientation_t = mul_mat(rotate_t, direction_t);
    turn_axis = apply_mat_vec3(rotate_t, turn_axis);
    tilt_axis = apply_mat_vec3(rotate_t, tilt_axis);
    orientation_t = mul_mat(rotation_axis(turn_axis, turn_rad), orientation_t);
    orientation_t = mul_mat(rotation_axis(tilt_axis, tilt_rad), orientation_t);
    let center_child_t = translation(-child_point[0], -child_point[1], -child_point[2]);
    let mut transform = mul_mat(orientation_t, center_child_t);
    let gap_transform = translation(gap_direction[0] * attraction.gap, gap_direction[1] * attraction.gap, gap_direction[2] * attraction.gap);
    let shift_transform = translation(shift_direction[0] * attraction.shift, shift_direction[1] * attraction.shift, shift_direction[2] * attraction.shift);
    let raise_transform = translation(raise_direction[0] * attraction.rise, raise_direction[1] * attraction.rise, raise_direction[2] * attraction.rise);
    transform = mul_mat(mul_mat(raise_transform, mul_mat(shift_transform, gap_transform)), transform);
    transform = mul_mat(translation(parent_point[0], parent_point[1], parent_point[2]), transform);
    matrix_to_plane(mul_mat(parent_matrix, transform))
}

/// 🔗️ One part's grip by id, plus its rim parameter. React converts `grip_2d.angle` to `t` while
/// composing (`🟦️.tsx:574`); the typed path does it here so the angle stays the authored unit.
fn grip_of<'a>(part: &'a Puzzle5dPart, grip_id: &str) -> Option<&'a Puzzle5dGrip> {
    part.grips.iter().find(|grip| grip.id == grip_id)
}

fn grip_t(grip: &Puzzle5dGrip) -> f64 {
    grip.grip_2d.angle / (2.0 * std::f64::consts::PI)
}

fn grip_direction(grip: &Puzzle5dGrip) -> [f64; 3] {
    normalize(grip.grip_3d.direction.unwrap_or([0.0, 0.0, 1.0]))
}

/// 📐️ Solves every part's pose — React's `flattenObjects` (`🟦️.tsx:461`) over the typed snapshot.
///
/// The graph is walked UNDIRECTED (both endpoints of a fastener push each other, `🟦️.tsx:477-480`)
/// and rooted twice: every `Fixed` part first, then whatever is still unvisited, so an all-derived
/// island still gets a frame. A `Fixed` root keeps its authored origin/orientation; a derived root is
/// reset to [`FlattenPlane::IDENTITY`]. An unaddressable endpoint or a missing grip degrades that one
/// hop to the identity plane at diagram origin rather than dropping the part.
pub fn flatten_poses(document: &Puzzle5dSnapshot) -> HashMap<String, FlattenPose> {
    let mut poses = HashMap::new();
    if document.parts.is_empty() {
        return poses;
    }
    let parts: HashMap<&str, &Puzzle5dPart> = document.parts.iter().map(|part| (part.id.as_str(), part)).collect();
    let mut adjacency: HashMap<&str, Vec<(&str, usize)>> = HashMap::new();
    for (index, fastener) in document.fasteners.iter().enumerate() {
        let Some((source_part, _)) = parse_endpoint(&fastener.source) else { continue };
        let Some((target_part, _)) = parse_endpoint(&fastener.target) else { continue };
        if !parts.contains_key(source_part) || !parts.contains_key(target_part) {
            continue;
        }
        adjacency.entry(source_part).or_default().push((target_part, index));
        adjacency.entry(target_part).or_default().push((source_part, index));
    }
    let mut planes: HashMap<&str, FlattenPlane> = HashMap::new();
    let mut centers: HashMap<&str, [f64; 2]> = HashMap::new();
    let mut visited: HashSet<&str> = HashSet::new();
    let roots = document.parts.iter().filter(|part| part.anchor == Puzzle5dPartAnchor::Fixed).chain(document.parts.iter());
    for root in roots {
        let root_id = root.id.as_str();
        if visited.contains(root_id) {
            continue;
        }
        visited.insert(root_id);
        planes.insert(root_id, if root.anchor == Puzzle5dPartAnchor::Fixed { orientation_to_plane(root.part_3d.origin, root.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])) } else { FlattenPlane::IDENTITY });
        centers.insert(root_id, [root.part_2d.x, root.part_2d.y]);
        let mut queue: VecDeque<&str> = VecDeque::from([root_id]);
        while let Some(current_id) = queue.pop_front() {
            let current_plane = planes.get(current_id).copied().unwrap_or(FlattenPlane::IDENTITY);
            let parent_center = centers.get(current_id).copied().unwrap_or([0.0, 0.0]);
            for (neighbor_id, fastener_index) in adjacency.get(current_id).cloned().unwrap_or_default() {
                if visited.contains(neighbor_id) {
                    continue;
                }
                visited.insert(neighbor_id);
                queue.push_back(neighbor_id);
                let fastener = &document.fasteners[fastener_index];
                let solved = solve_hop(&parts, current_id, neighbor_id, current_plane, parent_center, fastener);
                planes.insert(neighbor_id, solved.0);
                centers.insert(neighbor_id, solved.1);
            }
        }
    }
    for part in document.parts.iter() {
        let plane = planes.get(part.id.as_str()).copied().unwrap_or(FlattenPlane::IDENTITY);
        let center = centers.get(part.id.as_str()).copied().unwrap_or([0.0, 0.0]);
        poses.insert(part.id.clone(), FlattenPose { plane, center, orientation: plane_to_orientation(&plane) });
    }
    poses
}

/// 📐️ One parent→child hop: which end of the fastener the parent is on decides which grip is whose
/// (React's `parentEp[0] === currentId` test, `🟦️.tsx:517-518`).
fn solve_hop(parts: &HashMap<&str, &Puzzle5dPart>, current_id: &str, neighbor_id: &str, current_plane: FlattenPlane, parent_center: [f64; 2], fastener: &Puzzle5dFastener) -> (FlattenPlane, [f64; 2]) {
    let degenerate = (FlattenPlane::IDENTITY, [0.0, 0.0]);
    let Some((source_part, source_grip)) = parse_endpoint(&fastener.source) else { return degenerate };
    let Some((_, target_grip)) = parse_endpoint(&fastener.target) else { return degenerate };
    let parent_is_source = source_part == current_id;
    let current_grip_id = if parent_is_source { source_grip } else { target_grip };
    let neighbor_grip_id = if parent_is_source { target_grip } else { source_grip };
    let Some(current_part) = parts.get(current_id) else { return degenerate };
    let Some(neighbor_part) = parts.get(neighbor_id) else { return degenerate };
    let Some(parent_grip) = grip_of(current_part, current_grip_id) else { return degenerate };
    let Some(child_grip) = grip_of(neighbor_part, neighbor_grip_id) else { return degenerate };
    let attraction = Attraction::of(fastener);
    let parent_direction = grip_direction(parent_grip);
    let plane = compute_child_plane(&current_plane, parent_grip.grip_3d.position, parent_direction, child_grip.grip_3d.position, grip_direction(child_grip), &attraction);
    (plane, diagram_center(parent_center, parent_direction, grip_t(parent_grip), &attraction))
}

/// 🌤️ [`flatten_poses`] with the diagram centres scaled into board pixels — React's
/// `prepareTopologyModel` (`🟦️.tsx:615`). The 3-D half is untouched; only `2d.x`/`2d.y` scale.
pub fn prepare_topology_poses(document: &Puzzle5dSnapshot) -> HashMap<String, FlattenPose> {
    let mut poses = flatten_poses(document);
    for pose in poses.values_mut() {
        pose.center = [pose.center[0] * PUZZLE_5D_TOPOLOGY_ICON_WIDTH, pose.center[1] * PUZZLE_5D_TOPOLOGY_ICON_WIDTH];
    }
    poses
}

//#endregion 📐️Solver

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
