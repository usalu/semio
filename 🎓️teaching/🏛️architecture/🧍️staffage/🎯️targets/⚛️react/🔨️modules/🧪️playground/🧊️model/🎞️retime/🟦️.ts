/** 🎞️ Retiming of animation channels: the clip of one placed model, laid on the shared timeline.
 *
 * A clip channel is keyed in clip time. The timeline asks for the clip at `offsetS + (T - startS) · speed` for every instant `T` of a window; `retimeChannel` returns the channel keyed in timeline time that gives exactly that, with the clip unrolled over its repeats (`loop`), held at its first frame before the window and at its last after it, or held at one clip time (`holdS`). It is the 3D twin of `readTrack` (`../../🎞️timeline/🟦️.ts`): the same window, the same repeats, so the 2D drawing and the 3D model show one instant.
 * Rotations interpolate by spherical linear interpolation, every other path linearly, `step` channels stay steps; keys of one channel that come out at the same instant keep the first.
 *
 * @see ../../🎞️timeline/🟦️.ts — the reading this reproduces
 * @see ../🧩️compose/🟦️.ts — the composition that calls it
 */
import { qSlerp, type Channel, type Q4 } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";

/** 🪟️ A window of the timeline: `[startS, endS]` plays a clip of `clipS` seconds from the clip time `offsetS` at `speed`; `holdS` freezes it at one clip time. */
export type Window = { readonly startS: number; readonly endS: number; readonly speed: number; readonly offsetS: number; readonly loop: boolean; readonly holdS: number | null; readonly clipS: number };

/** 🔑️ One channel keyed in timeline time. */
export type Keyed = { readonly times: readonly number[]; readonly values: readonly number[] };

const EPSILON = 1e-9;
const isRotation = (channel: Channel): boolean => "node" in channel.target && channel.target.path === "rotation";
const widthOf = (channel: Channel): number => (channel.times.length === 0 ? 0 : channel.values.length / channel.times.length);

/** 🔍️ The value of a channel at clip time `timeS`, held before its first and after its last key. */
export function sampleChannel(channel: Channel, timeS: number): readonly number[] {
  const width = widthOf(channel);
  const { times, values } = channel;
  const at = (index: number): number[] => values.slice(index * width, index * width + width);
  if (times.length === 0) return [];
  if (timeS <= times[0]!) return at(0);
  const last = times.length - 1;
  if (timeS >= times[last]!) return at(last);
  let high = 1;
  while (times[high]! < timeS) high += 1;
  const low = high - 1;
  if (channel.interpolation === "step") return at(low);
  const span = times[high]! - times[low]!;
  const fraction = span <= 0 ? 0 : (timeS - times[low]!) / span;
  const a = at(low);
  const b = at(high);
  if (isRotation(channel)) return [...qSlerp(a as unknown as Q4, b as unknown as Q4, fraction)];
  return a.map((value, index) => value + (b[index]! - value) * fraction);
}

/** ⏸️ A channel held at one value for the whole timeline. */
export const constant = (value: readonly number[], timelineS: number): Keyed => ({ times: [0, timelineS], values: [...value, ...value] });

/** 🕰️ The channel in timeline time for one window; `timelineS` is the length of the timeline (the animation holds its last value up to it). */
export function retimeChannel(channel: Channel, window: Window, timelineS: number): Keyed {
  if (channel.times.length === 0) return { times: [], values: [] };
  if (window.holdS !== null || window.clipS <= 0) {
    const clipS = Math.max(window.clipS, 0);
    const held = window.holdS ?? 0;
    return constant(sampleChannel(channel, window.loop && clipS > 0 ? ((held % clipS) + clipS) % clipS : Math.min(Math.max(held, 0), clipS)), timelineS);
  }
  const { clipS, offsetS, speed, loop, startS } = window;
  const endS = Math.min(window.endS, timelineS);
  const clipAt = (timeS: number): number => offsetS + (timeS - startS) * speed;
  const value = (timeS: number): readonly number[] => {
    const raw = clipAt(timeS);
    return sampleChannel(channel, loop ? ((raw % clipS) + clipS) % clipS : Math.min(Math.max(raw, 0), clipS));
  };
  const times: number[] = [];
  const out: number[] = [];
  const push = (timeS: number, values: readonly number[]): void => {
    if (times.length > 0 && timeS <= times[times.length - 1]! + EPSILON) return;
    times.push(timeS);
    out.push(...values);
  };
  if (startS > EPSILON) push(0, value(startS));
  push(startS, value(startS));
  const clipEnd = clipAt(endS);
  const lastCycle = loop ? Math.floor(Math.max(clipEnd, offsetS) / clipS) : 0;
  const firstCycle = loop ? Math.floor(Math.min(clipEnd, offsetS) / clipS) : 0;
  const hold = loop ? endS : Math.min(endS, startS + (clipS - offsetS) / speed);
  for (let cycle = firstCycle; cycle <= lastCycle; cycle += 1) {
    channel.times.forEach((keyS, index) => {
      const timeS = startS + (cycle * clipS + keyS - offsetS) / speed;
      if (timeS > startS + EPSILON && timeS < hold - EPSILON) push(timeS, channel.values.slice(index * widthOf(channel), (index + 1) * widthOf(channel)));
    });
  }
  push(hold, value(hold));
  if (hold < timelineS - EPSILON) push(timelineS, value(hold));
  return { times, values: out };
}
