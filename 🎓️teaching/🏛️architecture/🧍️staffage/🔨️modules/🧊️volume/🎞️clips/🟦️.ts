/** 🎞️ Clip sampling into animation channels: the clip is evaluated by `evaluate` at its own fps (root motion, looping, respiration and events included) and every moving property becomes a keyed channel of the node or material it drives.
 *
 * Sampling times are `k / fps` for `k = 0 … ⌈duration · fps⌉`, the last one clamped to the duration, so the closing key of a looping clip is on the absolute timeline (a planted root has advanced by one cycle there). Rotation keys are made continuous (each quaternion on the hemisphere of its predecessor) so that the linear
 * slerp of the encoders takes the short way; a channel whose every key equals the value the node or material already has is omitted, except the translation of the rig root, which is always written so that a clip that moves nothing still has its duration (an animation without a channel is not an animation in glTF or FBX).
 *
 * @see ../../🧮️evaluation/🟦️.ts — `evaluate`
 */
import type { Clip, Slug } from "../../../🧬️schema/🟦️.ts";
import type { Animation, Channel } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { Triple, Quat } from "../🧭️frame/🟦️.ts";

/** ⏱️ The sampling times of a clip in seconds: `k / fps` up to the duration, the duration included. */
export function frameTimes(clip: Pick<Clip, "durationS" | "fps">): number[] {
  const count = Math.ceil(clip.durationS * clip.fps - 1e-9);
  const times: number[] = [];
  for (let k = 0; k < count; k++) times.push(k / clip.fps);
  times.push(clip.durationS);
  return times;
}

/** 📌️ The events of a clip as model events: `<kind>` or `<kind>-<target>`. */
export function eventsOf(clip: Pick<Clip, "events">): { readonly t: number; readonly id: Slug }[] {
  return clip.events.map((e) => ({ t: e.t, id: e.target === undefined ? e.kind : `${e.kind}-${e.target}` }));
}

/** 🔄️ The rotation keys made continuous: each quaternion is negated when it lies on the far hemisphere of the previous one (`first` is the quaternion the node already has). */
export function continuous(rotations: readonly Quat[], first: Quat): Quat[] {
  let previous = first;
  return rotations.map((q) => {
    const d = previous[0] * q[0] + previous[1] * q[1] + previous[2] * q[2] + previous[3] * q[3];
    const out: Quat = d < 0 ? [-q[0], -q[1], -q[2], -q[3]] : q;
    previous = out;
    return out;
  });
}

/** 🧩️ The full local transform of a node at one instant. */
export type Local = { readonly translation: Triple; readonly rotation: Quat; readonly scale: Triple };

const near = (a: readonly number[], b: readonly number[], eps: number): boolean => a.every((v, i) => Math.abs(v - b[i]!) <= eps);

/** 🎬️ The node channels of one node over the sampled instants against the transform it already has (`base`): translation, rotation, scale; channels that never differ from `base` are omitted unless named in `always`. */
export function nodeChannels(node: Slug, times: readonly number[], frames: readonly Local[], base: Local, always: readonly ("translation" | "rotation" | "scale")[] = []): Channel[] {
  const out: Channel[] = [];
  const keep = (path: "translation" | "rotation" | "scale", values: readonly (readonly number[])[], baseline: readonly number[], same: (a: readonly number[], b: readonly number[]) => boolean): void => {
    if (values.every((v) => same(v, baseline)) && !always.includes(path)) return;
    out.push({ target: { node, path }, interpolation: "linear", times: [...times], values: values.flat() });
  };
  keep("translation", frames.map((f) => f.translation), base.translation, (a, b) => near(a, b, 1e-9));
  const rotations = continuous(frames.map((f) => f.rotation), base.rotation);
  keep("rotation", rotations, base.rotation, (a, b) => near(a, b, 1e-9) || near(a, [-b[0]!, -b[1]!, -b[2]!, -b[3]!], 1e-9));
  keep("scale", frames.map((f) => f.scale), base.scale, (a, b) => near(a, b, 1e-9));
  return out;
}

/** 🎨️ The colour channel of a material over the sampled instants against its static colour; `null` when it never differs. */
export function colourChannel(material: Slug, times: readonly number[], colours: readonly (readonly number[])[], base: readonly number[]): Channel | null {
  if (colours.every((c) => near(c, base, 1e-6))) return null;
  return { target: { material, path: "baseColor" }, interpolation: "linear", times: [...times], values: colours.flat() };
}

/** 🏷️ An animation of a clip from its channels. */
export function animationOf(clip: Pick<Clip, "id" | "title" | "durationS" | "loop" | "events">, channels: readonly Channel[]): Animation {
  return { id: clip.id, title: clip.title, durationS: clip.durationS, loop: clip.loop, channels, events: eventsOf(clip) };
}
