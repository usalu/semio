/** 🎞️ FBX animation curves from model channels: per-axis curves, continuous unwrapped Euler rotation and dense enough keys that linear Euler interpolation follows the quaternion arc.
 *
 * @see https://help.autodesk.com/view/FBX/2017/ENU/?guid=__cpp_ref_class_fbx_anim_curve_html
 */
import type { Channel, Vec3 } from "../../../🧬️schema/🟦️.ts";
import { qConj, qDot, qMul, qNormalize, qSlerp } from "../../../🧮️math/🟦️.ts";
import { eulerDegrees, unwrapDegrees } from "../📐️euler/🟦️.ts";
import { srgbEncode } from "../🎨️material/🟦️.ts";
import * as portable from "../../../🧮️math/🎯️portable/🟦️.ts";

/** 📈️ One curve per component: key times in seconds and values. */
export type Curves = { readonly times: readonly number[]; readonly components: readonly (readonly number[])[]; readonly step: boolean };

const MAX_STEP_RAD = Math.PI / 12;

/** 📐️ Translation or scale channel: three component curves. */
export function vectorCurves(channel: Channel): Curves {
  const keys = channel.times.length;
  return { times: channel.times, components: [0, 1, 2].map((axis) => Array.from({ length: keys }, (_, key) => channel.values[key * 3 + axis])), step: channel.interpolation === "step" };
}

/** 🔄️ Rotation channel as Euler degrees about the XYZ axes of `Lcl Rotation`; `preRotation` (a quaternion) is divided out, long arcs are subdivided and every key is unwrapped against the previous one. */
export function rotationCurves(channel: Channel, preRotation: readonly number[] | undefined): Curves {
  const step = channel.interpolation === "step";
  const inversePre = preRotation ? qConj(preRotation) : undefined;
  const keys: { t: number; q: number[] }[] = [];
  for (let key = 0; key < channel.times.length; key++) {
    let q: number[] = [...qNormalize(channel.values.slice(key * 4, key * 4 + 4))];
    if (inversePre) q = qMul(inversePre, q) as unknown as number[];
    if (keys.length > 0 && qDot(keys[keys.length - 1].q, q) < 0) q = q.map((value) => -value);
    if (!step && keys.length > 0) {
      const previous = keys[keys.length - 1];
      const angle = 2 * portable.acos(Math.min(1, Math.abs(qDot(previous.q, q))));
      const segments = Math.ceil(angle / MAX_STEP_RAD);
      for (let part = 1; part < segments; part++) keys.push({ t: previous.t + ((channel.times[key] - previous.t) * part) / segments, q: qSlerp(previous.q, q, part / segments) as unknown as number[] });
    }
    keys.push({ t: channel.times[key], q });
  }
  const times: number[] = [];
  const components: number[][] = [[], [], []];
  let previous: Vec3 | undefined;
  for (const { t, q } of keys) {
    const angles = unwrapDegrees(eulerDegrees(q), previous);
    previous = angles;
    times.push(t);
    angles.forEach((value, axis) => components[axis].push(value));
  }
  return { times, components, step };
}

/** 🎨️ Colour channel (`baseColor` rgba or `emissive` rgb) as three sRGB-encoded component curves; reports whether alpha varies. */
export function colourCurves(channel: Channel, stride: 3 | 4): Curves & { readonly alphaVaries: boolean } {
  const count = channel.times.length;
  const components = [0, 1, 2].map((axis) => Array.from({ length: count }, (_, key) => srgbEncode(channel.values[key * stride + axis])));
  let alphaVaries = false;
  if (stride === 4) for (let key = 1; key < count; key++) if (channel.values[key * 4 + 3] !== channel.values[3]) alphaVaries = true;
  return { times: channel.times, components, step: channel.interpolation === "step", alphaVaries };
}
