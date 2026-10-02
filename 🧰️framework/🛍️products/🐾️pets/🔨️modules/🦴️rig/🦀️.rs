//! 🦴️ Forward kinematics of a species rig: 2×3 affine matrices in the SVG `matrix(a b c d e f)` convention (`x′ = a·x + c·y + e`, `y′ = b·x + d·y + f`), poses relative to the rest pose, and the gaze direction of an eye.
//!
//! Every expression is the TypeScript twin's, term for term and in its order (no reassociation, no `mul_add`), with
//! sine and cosine from [`crate::trigonometry`], so both cores yield the same bits. [`solve_rig`] multiplies with the
//! expressions of [`compose`]. A bone without a parent listed before it is a root, and a pose shorter than the rig
//! leaves the remaining bones at rest, exactly like the twin.
//!
//! @see <https://www.w3.org/TR/css-transforms-1/#mathematical-description> — the matrix convention
//! @see ../📐️trigonometry/🦀️.rs — `sin_turns`, `cos_turns`
//! @see ../../🧬️schema/🦀️.rs — `Species`, `Bone`, `Eye`, `Point`, `Degrees`
//! @see ../🦴️rig/🟦️.ts — the TypeScript twin

use crate::schema::{Bone, Degrees, Eye, Point, Species};
use crate::trigonometry::{cos_turns, sin_turns};
use serde::{Deserialize, Serialize};

const PUPIL_RIM: f64 = 0.25;

//#region 🔖️Affine
/// 🔳️ A 2×3 affine matrix `[a, b, c, d, e, f]`: the columns `(a, b)` and `(c, d)` carry rotation and scale, `(e, f)` the translation.
pub type Affine = [f64; 6];

/// 🟰️ The matrix that changes nothing.
pub const IDENTITY: Affine = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

/// ✖️ `parent × local`: the matrix that applies `local` first and `parent` after it.
pub fn compose(parent: Affine, local: Affine) -> Affine {
    [
        parent[0] * local[0] + parent[2] * local[1],
        parent[1] * local[0] + parent[3] * local[1],
        parent[0] * local[2] + parent[2] * local[3],
        parent[1] * local[2] + parent[3] * local[3],
        parent[0] * local[4] + parent[2] * local[5] + parent[4],
        parent[1] * local[4] + parent[3] * local[5] + parent[5],
    ]
}

/// ↩️ The matrix that undoes `matrix` (adjugate ÷ determinant `a·d − b·c`); a matrix without an inverse (determinant 0) yields [`IDENTITY`], so the result is always finite.
pub fn invert(matrix: Affine) -> Affine {
    let determinant = matrix[0] * matrix[3] - matrix[1] * matrix[2];
    if determinant == 0.0 {
        return IDENTITY;
    }
    [matrix[3] / determinant, (0.0 - matrix[1]) / determinant, (0.0 - matrix[2]) / determinant, matrix[0] / determinant, (matrix[2] * matrix[5] - matrix[3] * matrix[4]) / determinant, (matrix[1] * matrix[4] - matrix[0] * matrix[5]) / determinant]
}

/// 📌️ The point `(x, y)` carried by `matrix`.
pub fn transform(matrix: Affine, x: f64, y: f64) -> Point {
    Point { x: matrix[0] * x + matrix[2] * y + matrix[4], y: matrix[1] * x + matrix[3] * y + matrix[5] }
}
//#endregion 🔖️Affine

//#region 🔖️Pose
/// 🤸️ What a bone adds to its rest transform: offsets in pixels, an offset in degrees and scale factors (rest = 0, 0, 0, 1, 1).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BonePose {
    pub x: f64,
    pub y: f64,
    pub rotation: Degrees,
    pub scale_x: f64,
    pub scale_y: f64,
}

/// 🧍️ One [`BonePose`] per bone, in rig order.
pub type Pose = Vec<BonePose>;

const REST: BonePose = BonePose { x: 0.0, y: 0.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 };

/// 🛌️ The pose in which every bone stays at rest: no offset, no rotation, unit scale.
pub fn rest_pose(species: &Species) -> Pose {
    vec![REST; species.bones.len()]
}
//#endregion 🔖️Pose

//#region 🔖️Solving
/// 👪️ The index of the bone's parent among the bones listed before it; `None` for a root (no parent, or none listed earlier).
fn parent_index(bones: &[Bone], index: usize) -> Option<usize> {
    let parent = bones[index].parent.as_ref()?;
    (0..index).rev().find(|&candidate| &bones[candidate].id == parent)
}

/// 🩻️ The world matrix of every bone relative to the feet origin, six numbers `a b c d e f` per bone in rig order.
///
/// `local = translate(bone.x + pose.x, bone.y + pose.y) × rotate((bone.rotation + pose.rotation) ÷ 360 turns) × scale(pose.scale_x, pose.scale_y)`,
/// `world = parent world × local` (a root's world matrix is its local one). Bones the pose does not cover stay at rest.
pub fn solve_rig(species: &Species, pose: &[BonePose]) -> Vec<f64> {
    let bones = &species.bones;
    let mut world: Vec<f64> = Vec::with_capacity(bones.len() * 6);
    for (index, bone) in bones.iter().enumerate() {
        let posed = pose.get(index).copied().unwrap_or(REST);
        let turns = (bone.rotation.unwrap_or(0.0) + posed.rotation) / 360.0;
        let sine = sin_turns(turns);
        let cosine = cos_turns(turns);
        let a = cosine * posed.scale_x;
        let b = sine * posed.scale_x;
        let c = (0.0 - sine) * posed.scale_y;
        let d = cosine * posed.scale_y;
        let e = bone.x + posed.x;
        let f = bone.y + posed.y;
        let Some(parent) = parent_index(bones, index) else {
            world.extend([a, b, c, d, e, f]);
            continue;
        };
        let offset = parent * 6;
        let pa = world[offset];
        let pb = world[offset + 1];
        let pc = world[offset + 2];
        let pd = world[offset + 3];
        let pe = world[offset + 4];
        let pf = world[offset + 5];
        world.extend([pa * a + pc * b, pb * a + pd * b, pa * c + pc * d, pb * c + pd * d, pa * e + pc * f + pe, pb * e + pd * f + pf]);
    }
    world
}
//#endregion 🔖️Solving

//#region 🔖️Gaze
/// 👀️ Where a pupil is drawn towards: the direction from `eye` to `target`, scaled to the length `d ÷ (d + reach)` for the distance `d`, so the result lies inside the unit disc (on its rim only when `reach` is not positive); `(0, 0)` when the points coincide.
pub fn look_offset(eye: Point, target: Point, reach: f64) -> Point {
    let dx = target.x - eye.x;
    let dy = target.y - eye.y;
    let distance = (dx * dx + dy * dy).sqrt();
    if distance == 0.0 {
        return Point { x: 0.0, y: 0.0 };
    }
    let span = distance + if reach > 0.0 { reach } else { 0.0 };
    Point { x: dx / span, y: dy / span }
}

/// 👁️ How far the pupil of an eye travels from the centre of its white, in pixels: until it reaches the outline — `radius − pupil − 0.25`, the quarter pixel keeping it off the middle of the stroke —, never less than 0. A gaze between −1 and 1 times this is where a pupil is drawn.
pub fn pupil_reach(eye: &Eye) -> f64 {
    let reach = eye.radius - eye.pupil - PUPIL_RIM;
    if reach > 0.0 {
        reach
    } else {
        0.0
    }
}
//#endregion 🔖️Gaze

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
