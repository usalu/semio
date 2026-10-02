/** 🎞️ The motion of a pet in whole ticks: CSS cubic-bezier easing by bisection, keyframed tracks and clips sampled into poses, the cross-fade of two poses, the one-tick spring of the gaze and the lid of a blink.
 *
 * Built from `+ − × ÷`, `floor`, `min`, the integer remainder of whole ticks and comparisons only (design §2.4). Every
 * constant is a literal and every expression is written once, in a fixed order (no reassociation), so the Rust twin
 * yields the same bits. A clip speaks in offsets from the rest pose: a channel no track of the clip names stays at
 * rest (offsets 0, scale 1).
 *
 * @see https://www.w3.org/TR/css-easing-1/#cubic-bezier-easing-functions — the easing a key carries
 * @see https://gafferongames.com/post/integration_basics/ — semi-implicit Euler, the integrator of {@link springStep}
 * @see ../📐️trigonometry/🟦️.ts — `lerp`, `smoothstep`
 * @see ../🦴️rig/🟦️.ts — `Pose`, solved into matrices by `solveRig`
 * @see ../../🧬️schema/🟦️.ts — `Ease`, `Track`, `Clip`, `Species`, `Ticks`
 * @see ./🦀️.rs — the Rust twin
 */

import { TICKS_PER_SECOND, type Channel, type Clip, type Ease, type Species, type Ticks, type Track } from "../../🧬️schema/🟦️.ts";
import type { BonePose, Pose } from "../🦴️rig/🟦️.ts";
import { lerp, smoothstep } from "../📐️trigonometry/🟦️.ts";

//#region 🔖️Easing
const BISECTIONS = 48;

/** 🎢️ The CSS `cubic-bezier(x1, y1, x2, y2)` easing of `amount`: 0 at and below 0, 1 at and above 1, in between the curve's `y` at the parameter where its `x` equals `amount`.
 *
 * Both coordinates are the cubic `((a·s + b)·s + c)·s` with `c = 3·p1`, `b = 3·(p2 − p1) − c`, `a = 1 − c − b`. The
 * parameter is found by 48 halvings of `[0, 1]` (the lower half is kept while `x(middle) < amount`) and is the middle
 * of the last interval, so it is off by at most 2⁻⁴⁹. Where the curve's `x` stands still (`x1 = 1, x2 = 0` at one
 * half) the parameter is ill-conditioned for every solver; the result still lies on the curve.
 */
export function easeBezier(ease: Ease, amount: number): number {
  if (amount <= 0) return 0;
  if (amount >= 1) return 1;
  const cx = 3 * ease[0];
  const bx = 3 * (ease[2] - ease[0]) - cx;
  const ax = 1 - cx - bx;
  const cy = 3 * ease[1];
  const by = 3 * (ease[3] - ease[1]) - cy;
  const ay = 1 - cy - by;
  let low = 0;
  let high = 1;
  for (let step = 0; step < BISECTIONS; step++) {
    const middle = (low + high) * 0.5;
    if (((ax * middle + bx) * middle + cx) * middle < amount) low = middle;
    else high = middle;
  }
  const solved = (low + high) * 0.5;
  return ((ay * solved + by) * solved + cy) * solved;
}
//#endregion 🔖️Easing

//#region 🔖️Sampling
/** 🧘️ The value a channel has at rest: 1 for the scale factors, 0 for the offsets. */
function restOf(channel: Channel): number {
  return channel === "scaleX" || channel === "scaleY" ? 1 : 0;
}

/** 🛤️ The value of a track at `phase` (0…1 of its clip): the first key's value at and before its phase, the last key's at and after its phase, in between `lerp` of the two neighbouring keys by their local phase, shaped by the earlier key's ease (linear without one).
 *
 * The segment is the last one whose earlier key lies at or before `phase`, so a phase on a key yields that key's
 * value exactly. A track without keys yields its channel's rest value.
 */
export function sampleTrack(track: Track, phase: number): number {
  const keys = track.keys;
  const last = keys.length - 1;
  if (last < 0) return restOf(track.channel);
  if (phase <= keys[0]!.at) return keys[0]!.value;
  if (phase >= keys[last]!.at) return keys[last]!.value;
  let index = 0;
  while (keys[index + 1]!.at <= phase) index++;
  const from = keys[index]!;
  const to = keys[index + 1]!;
  const local = (phase - from.at) / (to.at - from.at);
  return lerp(from.value, to.value, from.ease === undefined ? local : easeBezier(from.ease, local));
}

/** ⏱️ The length of a clip in whole ticks: `floor(seconds × 64 + 0.5)`, at least 1. */
export function clipTicks(clip: Clip): Ticks {
  const ticks = Math.floor(clip.seconds * TICKS_PER_SECOND + 0.5);
  return ticks >= 1 ? ticks : 1;
}

/** 🎬️ The pose of a species `ticks` after `clip` began, one bone pose per bone in rig order.
 *
 * With `length = clipTicks(clip)` and ticks before the beginning counted as 0, the phase of a looping clip is
 * `(ticks mod length) ÷ length` (it wraps to its first key) and that of any other clip `min(ticks, length) ÷ length`
 * (it holds its last key). Every track then writes `sampleTrack(track, phase)` into its channel of its bone, in
 * track order (the later of two tracks on one channel wins); channels without a track stay at rest and a track on a
 * bone the species does not have is skipped.
 */
export function sampleClip(species: Species, clip: Clip, ticks: Ticks): Pose {
  const length = clipTicks(clip);
  const elapsed = ticks > 0 ? ticks : 0;
  const phase = (clip.loop ? elapsed % length : Math.min(elapsed, length)) / length;
  const bones = species.bones;
  const pose: { -readonly [channel in keyof BonePose]: BonePose[channel] }[] = bones.map(() => ({ x: 0, y: 0, rotation: 0, scaleX: 1, scaleY: 1 }));
  for (const track of clip.tracks) {
    for (let index = 0; index < bones.length; index++) {
      if (bones[index]!.id !== track.bone) continue;
      pose[index]![track.channel] = sampleTrack(track, phase);
      break;
    }
  }
  return pose;
}
//#endregion 🔖️Sampling

//#region 🔖️Blending
/** 🌗️ The pose `amount` of the way from `from` to `to`, two poses of one rig: `from` itself at and below 0, `to` itself at and above 1, in between `lerp` of every channel of every bone (rotations are plain offsets in degrees, never wrapped). */
export function blendPose(from: Pose, to: Pose, amount: number): Pose {
  if (amount <= 0) return from;
  if (amount >= 1) return to;
  return from.map((bone, index) => {
    const other = to[index]!;
    return {
      x: lerp(bone.x, other.x, amount),
      y: lerp(bone.y, other.y, amount),
      rotation: lerp(bone.rotation, other.rotation, amount),
      scaleX: lerp(bone.scaleX, other.scaleX, amount),
      scaleY: lerp(bone.scaleY, other.scaleY, amount),
    };
  });
}
//#endregion 🔖️Blending

//#region 🔖️Spring
const TICK_SECONDS = 0.015625;

/** 🌀️ The state of a spring on one axis: where it is and how fast it moves, in units per second. */
export type Spring = { readonly position: number; readonly velocity: number };

/** 🔭️ The stiffness of the gaze spring in 1/s², for {@link springStep} on each axis of an actor's gaze.
 *
 * With {@link GAZE_DAMPING} the pupil is calm and slightly springy: after a jump of its target a pupil at rest has
 * covered half of the way after 4 ticks, stays within 2 % of the way from the 9th tick (0.14 s) and within 1 % from
 * the 14th (0.22 s, settled inside 0.25 s), and it overshoots once, by 1.01 % of the way at the 13th tick. One tick
 * of this spring is the matrix `[[0.875, 0.0078125], [−8, 0.5]]` on `(position − target, velocity)`, exact in binary.
 */
export const GAZE_STIFFNESS = 512;

/** 🧲️ The damping of the gaze spring in 1/s, the companion of {@link GAZE_STIFFNESS} (a damping ratio of 0.71 before discretisation). */
export const GAZE_DAMPING = 32;

/** 🪀️ One tick (`1/64` s) of a damped spring towards `target` by semi-implicit Euler: the velocity first, `velocity + (stiffness × (target − position) − damping × velocity) × 1/64`, then the position with the new velocity, `position + velocity′ × 1/64`.
 *
 * A spring at its target without velocity stays there exactly. The step is stable (every motion dies out) while
 * `stiffness > 0`, `damping > 0` and `stiffness ÷ 4096 + damping ÷ 32 < 4`.
 */
export function springStep(position: number, velocity: number, target: number, stiffness: number, damping: number): Spring {
  const quickened = velocity + (stiffness * (target - position) - damping * velocity) * TICK_SECONDS;
  return { position: position + quickened * TICK_SECONDS, velocity: quickened };
}
//#endregion 🔖️Spring

//#region 🔖️Blink
/** 😉️ How many ticks a blink lasts (0.1875 s). */
export const BLINK_TICKS = 12;

/** 👁️ How far the lid is shut `ticks` after a blink began: 0 open, 1 shut, 0 at and before the beginning and from {@link BLINK_TICKS} on.
 *
 * The lid closes over 4 ticks (`smoothstep(ticks ÷ 4)`), stays shut from the 4th to the 5th tick and opens over the
 * remaining 7 (`1 − smoothstep((ticks − 5) ÷ 7)`): closing is faster than opening, and a stage drawn at every other
 * tick still shows one fully shut lid whichever parity it draws.
 */
export function lidAt(ticks: Ticks): number {
  if (ticks <= 0 || ticks >= BLINK_TICKS) return 0;
  if (ticks < 5) return smoothstep(ticks / 4);
  return 1 - smoothstep((ticks - 5) / 7);
}
//#endregion 🔖️Blink
