/** 🌤️ The environment at an absolute time: a constant environment or a keyed {@link EnvironmentTimeline} (`step` holds, `linear` blends to the next key).
 *
 * The wind blends as a horizontal velocity vector `(U cos φ, U sin φ)` in the ahead-left plane, component-wise and linearly, so opposite winds pass through calm instead of swinging round; every other field blends linearly. This is the one environment evaluator of thermal and transport. Before the first key the first environment holds, after the last the last.
 *
 * @see ../../🔗️contracts/🟦️.ts — EnvironmentTimeline
 */
import type { Environment } from "../../../🧬️schema/🟦️.ts";
import type { EnvironmentTimeline } from "../../🔗️contracts/🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 🌤️ A constant environment or a timeline. */
export type EnvironmentInput = Environment | EnvironmentTimeline;

/** 🌤️ Whether the input is a timeline. */
export const isTimeline = (input: EnvironmentInput): input is EnvironmentTimeline => "keys" in input;

function blend(a: Environment, b: Environment, f: number): Environment {
  const lerp = (x: number, y: number): number => x + (y - x) * f;
  const opt = (x: number | undefined, y: number | undefined): number | undefined => (x === undefined && y === undefined ? undefined : lerp(x ?? y!, y ?? x!));
  const rad = (d: number): number => (d * Math.PI) / 180;
  const wx = lerp(a.windMS * portable.cos(rad(a.windDirectionDeg)), b.windMS * portable.cos(rad(b.windDirectionDeg)));
  const wz = lerp(a.windMS * portable.sin(rad(a.windDirectionDeg)), b.windMS * portable.sin(rad(b.windDirectionDeg)));
  const speed = portable.hypot(wx, wz);
  const sky = opt(a.skyTempC, b.skyTempC);
  const ground = opt(a.groundTempC, b.groundTempC);
  return {
    airTempC: lerp(a.airTempC, b.airTempC),
    meanRadiantTempC: lerp(a.meanRadiantTempC, b.meanRadiantTempC),
    ...(sky === undefined ? {} : { skyTempC: sky }),
    ...(ground === undefined ? {} : { groundTempC: ground }),
    relativeHumidity: lerp(a.relativeHumidity, b.relativeHumidity),
    windMS: speed,
    windDirectionDeg: speed === 0 ? a.windDirectionDeg : (portable.atan2(wz, wx) * 180) / Math.PI,
    solarWM2: lerp(a.solarWM2, b.solarWM2),
    diffuseFraction: lerp(a.diffuseFraction, b.diffuseFraction),
    pressurePa: lerp(a.pressurePa, b.pressurePa),
  };
}

/** 🌤️ The environment at absolute time `timeS`. */
export function environmentAt(input: EnvironmentInput, timeS: number): Environment {
  if (!isTimeline(input)) return input;
  const keys = input.keys;
  if (keys.length === 0) throw new Error("An environment timeline needs at least one key.");
  const next = keys.findIndex((k) => k.t > timeS);
  if (next === 0) return keys[0]!.environment;
  if (next === -1) return keys[keys.length - 1]!.environment;
  const a = keys[next - 1]!;
  const b = keys[next]!;
  return a.ease === "step" ? a.environment : blend(a.environment, b.environment, (timeS - a.t) / (b.t - a.t));
}
