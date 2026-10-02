//! 🎞️ The motion of a pet in whole ticks: CSS cubic-bezier easing by bisection, keyframed tracks and clips sampled into
//! poses, the cross-fade of two poses, the one-tick spring of the gaze and the lid of a blink (design §4.4).
//!
//! Built from `+ − × ÷`, `floor`, the integer remainder and comparison of whole ticks and comparisons only (design
//! §2.4). Every constant is a literal and every expression is the TypeScript twin's, term for term and in its order
//! (no reassociation, no `mul_add`), so both cores yield the same bits. A clip speaks in offsets from the rest pose: a
//! channel no track of the clip names stays at rest (offsets 0, scale 1).
//!
//! @see <https://www.w3.org/TR/css-easing-1/#cubic-bezier-easing-functions> — the easing a key carries
//! @see <https://gafferongames.com/post/integration_basics/> — semi-implicit Euler, the integrator of [`spring_step`]
//! @see ../📐️trigonometry/🦀️.rs — `lerp`, `smoothstep`
//! @see ../🦴️rig/🦀️.rs — `Pose`, solved into matrices by `solve_rig`
//! @see ../../🧬️schema/🦀️.rs — `Ease`, `Track`, `Clip`, `Species`, `Ticks`
//! @see ../🎞️animation/🟦️.ts — the TypeScript twin

use crate::rig::{rest_pose, BonePose, Pose};
use crate::schema::{Channel, Clip, Ease, Species, Ticks, Track, TICKS_PER_SECOND};
use crate::trigonometry::{lerp, smoothstep};
use serde::{Deserialize, Serialize};

//#region 🔖️Easing
const BISECTIONS: u32 = 48;

/// 🎢️ The CSS `cubic-bezier(x1, y1, x2, y2)` easing of `amount`: 0 at and below 0, 1 at and above 1, in between the curve's `y` at the parameter where its `x` equals `amount`.
///
/// Both coordinates are the cubic `((a·s + b)·s + c)·s` with `c = 3·p1`, `b = 3·(p2 − p1) − c`, `a = 1 − c − b`. The
/// parameter is found by 48 halvings of `[0, 1]` (the lower half is kept while `x(middle) < amount`) and is the middle
/// of the last interval, so it is off by at most 2⁻⁴⁹. Where the curve's `x` stands still (`x1 = 1, x2 = 0` at one
/// half) the parameter is ill-conditioned for every solver; the result still lies on the curve.
pub fn ease_bezier(ease: Ease, amount: f64) -> f64 {
    if amount <= 0.0 {
        return 0.0;
    }
    if amount >= 1.0 {
        return 1.0;
    }
    let cx = 3.0 * ease[0];
    let bx = 3.0 * (ease[2] - ease[0]) - cx;
    let ax = 1.0 - cx - bx;
    let cy = 3.0 * ease[1];
    let by = 3.0 * (ease[3] - ease[1]) - cy;
    let ay = 1.0 - cy - by;
    let mut low = 0.0;
    let mut high = 1.0;
    for _ in 0..BISECTIONS {
        let middle = (low + high) * 0.5;
        if ((ax * middle + bx) * middle + cx) * middle < amount {
            low = middle;
        } else {
            high = middle;
        }
    }
    let solved = (low + high) * 0.5;
    ((ay * solved + by) * solved + cy) * solved
}
//#endregion 🔖️Easing

//#region 🔖️Sampling
const RATE: f64 = TICKS_PER_SECOND as f64;

/// 🧘️ The value a channel has at rest: 1 for the scale factors, 0 for the offsets.
fn rest_of(channel: Channel) -> f64 {
    if matches!(channel, Channel::ScaleX | Channel::ScaleY) {
        1.0
    } else {
        0.0
    }
}

/// 🛤️ The value of a track at `phase` (0…1 of its clip): the first key's value at and before its phase, the last key's at and after its phase, in between `lerp` of the two neighbouring keys by their local phase, shaped by the earlier key's ease (linear without one).
///
/// The segment is the last one whose earlier key lies at or before `phase`, so a phase on a key yields that key's
/// value exactly. A track without keys yields its channel's rest value.
pub fn sample_track(track: &Track, phase: f64) -> f64 {
    let keys = &track.keys;
    let Some(last) = keys.len().checked_sub(1) else {
        return rest_of(track.channel);
    };
    if phase <= keys[0].at {
        return keys[0].value;
    }
    if phase >= keys[last].at {
        return keys[last].value;
    }
    let mut index = 0;
    while keys[index + 1].at <= phase {
        index += 1;
    }
    let from = keys[index];
    let to = keys[index + 1];
    let local = (phase - from.at) / (to.at - from.at);
    lerp(from.value, to.value, from.ease.map_or(local, |ease| ease_bezier(ease, local)))
}

/// ⏱️ The length of a clip in whole ticks: `floor(seconds × 64 + 0.5)`, at least 1.
pub fn clip_ticks(clip: &Clip) -> Ticks {
    let ticks = (clip.seconds * RATE + 0.5).floor();
    if ticks >= 1.0 {
        ticks as Ticks
    } else {
        1
    }
}

/// 🎬️ The pose of a species `ticks` after `clip` began, one bone pose per bone in rig order.
///
/// With `length = clip_ticks(clip)` and ticks before the beginning counted as 0, the phase of a looping clip is
/// `(ticks mod length) ÷ length` (it wraps to its first key) and that of any other clip `min(ticks, length) ÷ length`
/// (it holds its last key). Every track then writes `sample_track(track, phase)` into its channel of its bone, in
/// track order (the later of two tracks on one channel wins); channels without a track stay at rest and a track on a
/// bone the species does not have is skipped.
pub fn sample_clip(species: &Species, clip: &Clip, ticks: Ticks) -> Pose {
    let length = clip_ticks(clip);
    let elapsed = if ticks > 0 { ticks } else { 0 };
    let played = if clip.looping {
        elapsed % length
    } else if elapsed < length {
        elapsed
    } else {
        length
    };
    let phase = played as f64 / length as f64;
    let mut pose = rest_pose(species);
    for track in &clip.tracks {
        let Some(index) = species.bones.iter().position(|bone| bone.id == track.bone) else {
            continue;
        };
        let value = sample_track(track, phase);
        let posed = &mut pose[index];
        match track.channel {
            Channel::X => posed.x = value,
            Channel::Y => posed.y = value,
            Channel::Rotation => posed.rotation = value,
            Channel::ScaleX => posed.scale_x = value,
            Channel::ScaleY => posed.scale_y = value,
        }
    }
    pose
}
//#endregion 🔖️Sampling

//#region 🔖️Blending
/// 🌗️ The pose `amount` of the way from `from` to `to`, two poses of one rig: `from` itself at and below 0, `to` itself at and above 1, in between `lerp` of every channel of every bone (rotations are plain offsets in degrees, never wrapped).
pub fn blend_pose(from: &[BonePose], to: &[BonePose], amount: f64) -> Pose {
    if amount <= 0.0 {
        return from.to_vec();
    }
    if amount >= 1.0 {
        return to.to_vec();
    }
    from.iter()
        .enumerate()
        .map(|(index, bone)| {
            let other = to[index];
            BonePose { x: lerp(bone.x, other.x, amount), y: lerp(bone.y, other.y, amount), rotation: lerp(bone.rotation, other.rotation, amount), scale_x: lerp(bone.scale_x, other.scale_x, amount), scale_y: lerp(bone.scale_y, other.scale_y, amount) }
        })
        .collect()
}
//#endregion 🔖️Blending

//#region 🔖️Spring
const TICK_SECONDS: f64 = 0.015625;

/// 🌀️ The state of a spring on one axis: where it is and how fast it moves, in units per second.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spring {
    pub position: f64,
    pub velocity: f64,
}

/// 🔭️ The stiffness of the gaze spring in 1/s², for [`spring_step`] on each axis of an actor's gaze.
///
/// With [`GAZE_DAMPING`] the pupil is calm and slightly springy: after a jump of its target a pupil at rest has
/// covered half of the way after 4 ticks, stays within 2 % of the way from the 9th tick (0.14 s) and within 1 % from
/// the 14th (0.22 s, settled inside 0.25 s), and it overshoots once, by 1.01 % of the way at the 13th tick. One tick
/// of this spring is the matrix `[[0.875, 0.0078125], [−8, 0.5]]` on `(position − target, velocity)`, exact in binary.
pub const GAZE_STIFFNESS: f64 = 512.0;

/// 🧲️ The damping of the gaze spring in 1/s, the companion of [`GAZE_STIFFNESS`] (a damping ratio of 0.71 before discretisation).
pub const GAZE_DAMPING: f64 = 32.0;

/// 🪀️ One tick (`1/64` s) of a damped spring towards `target` by semi-implicit Euler: the velocity first, `velocity + (stiffness × (target − position) − damping × velocity) × 1/64`, then the position with the new velocity, `position + velocity′ × 1/64`.
///
/// A spring at its target without velocity stays there exactly. The step is stable (every motion dies out) while
/// `stiffness > 0`, `damping > 0` and `stiffness ÷ 4096 + damping ÷ 32 < 4`.
pub fn spring_step(position: f64, velocity: f64, target: f64, stiffness: f64, damping: f64) -> Spring {
    let quickened = velocity + (stiffness * (target - position) - damping * velocity) * TICK_SECONDS;
    Spring { position: position + quickened * TICK_SECONDS, velocity: quickened }
}
//#endregion 🔖️Spring

//#region 🔖️Blink
/// 😉️ How many ticks a blink lasts (0.1875 s).
pub const BLINK_TICKS: Ticks = 12;

/// 👁️ How far the lid is shut `ticks` after a blink began: 0 open, 1 shut, 0 at and before the beginning and from [`BLINK_TICKS`] on.
///
/// The lid closes over 4 ticks (`smoothstep(ticks ÷ 4)`), stays shut from the 4th to the 5th tick and opens over the
/// remaining 7 (`1 − smoothstep((ticks − 5) ÷ 7)`): closing is faster than opening, and a stage drawn at every other
/// tick still shows one fully shut lid whichever parity it draws.
pub fn lid_at(ticks: Ticks) -> f64 {
    if ticks <= 0 || ticks >= BLINK_TICKS {
        return 0.0;
    }
    if ticks < 5 {
        return smoothstep(ticks as f64 / 4.0);
    }
    1.0 - smoothstep((ticks as f64 - 5.0) / 7.0)
}
//#endregion 🔖️Blink

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
