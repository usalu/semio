/** 🌤️ The environment at an absolute time: a constant environment or a keyed {@link EnvironmentTimeline} (`step` holds, `linear` blends to the next key).
 *
 * Wind direction blends along the shorter arc; every other field blends linearly. Before the first key the first environment holds, after the last the last.
 *
 * @see ../../🔗️contracts/🟦️.ts — EnvironmentTimeline
 */
import type { Environment } from "../../../🧬️schema/🟦️.ts";
import type { EnvironmentTimeline } from "../../🔗️contracts/🟦️.ts";

/** 🌤️ A constant environment or a timeline. */
export type EnvironmentInput = Environment | EnvironmentTimeline;

/** 🌤️ Whether the input is a timeline. */
export const isTimeline = (input: EnvironmentInput): input is EnvironmentTimeline => "keys" in input;

function blend(a: Environment, b: Environment, f: number): Environment {
  const lerp = (x: number, y: number): number => x + (y - x) * f;
  const opt = (x: number | undefined, y: number | undefined): number | undefined => (x === undefined && y === undefined ? undefined : lerp(x ?? y!, y ?? x!));
  const turn = ((((b.windDirectionDeg - a.windDirectionDeg) % 360) + 540) % 360) - 180;
  const sky = opt(a.skyTempC, b.skyTempC);
  const ground = opt(a.groundTempC, b.groundTempC);
  return {
    airTempC: lerp(a.airTempC, b.airTempC),
    meanRadiantTempC: lerp(a.meanRadiantTempC, b.meanRadiantTempC),
    ...(sky === undefined ? {} : { skyTempC: sky }),
    ...(ground === undefined ? {} : { groundTempC: ground }),
    relativeHumidity: lerp(a.relativeHumidity, b.relativeHumidity),
    windMS: lerp(a.windMS, b.windMS),
    windDirectionDeg: a.windDirectionDeg + turn * f,
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
