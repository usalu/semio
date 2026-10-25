/** 🧰️ Shared kit of the differential check table of `@semio-tech/print-viz-inference`: the three sampling densities every check table honours, the shape of one differential check, the sample counts of a level and the recorder of a d3 path.
 *
 * @see ../🟦️.ts — the differential check table that assembles the topics
 */
import * as mark from "../../✒️mark/🟦️.ts";

//#region 🔖️Level
/** 🎚️ The three sampling densities every check table honours. */
export const VIZ_INFERENCE_LEVELS = ["quick", "long", "exhaustive"] as const;

export type Level = (typeof VIZ_INFERENCE_LEVELS)[number];
//#endregion 🔖️Level

//#region 🔖️Checks
/** 🧪️ One differential check: our kernel against the d3 package registered as its oracle. */
export type VizKernelCheck = {
  readonly module: string;
  readonly name: string;
  readonly subject: () => unknown | Promise<unknown>;
  readonly oracle: () => unknown | Promise<unknown>;
  readonly tolerance?: number;
};

function sampleCount(level: Level): number {
  return level === "quick" ? 9 : level === "long" ? 61 : 401;
}

export function samples(level: Level, lo: number, hi: number): number[] {
  const n = sampleCount(level);
  return Array.from({ length: n }, (_, i) => lo + ((hi - lo) * i) / (n - 1));
}

/** 🖊️ Binds an independently imported D3 generator to the owned path-only context contract. */
export function recordD3Path<T extends object>(generator: T, context: mark.VizPathContext): T {
  const setter: unknown = Reflect.get(generator, "context");
  if (typeof setter !== "function") throw new Error("D3 generator has no context setter");
  const configured: unknown = Reflect.apply(setter, generator, [context]);
  if (configured !== generator) throw new Error("D3 context setter did not return its generator");
  return generator;
}
//#endregion 🔖️Checks
