/** 🕰️ Thermal response along an asset's own timeline: the inputs at every earlier absolute time come from evaluating the asset's query at that time (pose binding blended
 * along the clip, tracks applied, looping clips continuing on the absolute timeline), so the temperatures at time `t` are the same whether reached by playing or by scrubbing.
 *
 * @see ../🟦️.ts — `solveThermal`
 * @see ../../🧮️evaluation/🟦️.ts — `evaluate`
 */
import type { Asset } from "../../../🧬️schema/🟦️.ts";
import type { EvaluatedState, EvaluationQuery, ThermalField } from "../../🔗️contracts/🟦️.ts";
import { evaluate } from "../../🧮️evaluation/🟦️.ts";
import { type EnvironmentInput, type ThermalOptions, type ThermalSample, solveThermal } from "../🟦️.ts";

/** 🕰️ Options of {@link solveThermalAt}: `evaluate` replaces the asset evaluation (tests, caches). */
export type TimelineOptions = ThermalOptions & { readonly evaluate?: (asset: Asset, query: EvaluationQuery) => EvaluatedState };

/** 🕰️ Solves the thermal field of `asset` at `query` with the full input history: clips and transitions are evaluated at every integration time; a pose holds its inputs, so only a
 * time-varying environment makes the response transient. The evaluation here carries no drawing geometry, which is why the result cannot depend on the drawing scale. */
export function solveThermalAt(asset: Asset, query: EvaluationQuery, environment?: EnvironmentInput, options: TimelineOptions = {}): ThermalField {
  const run = options.evaluate ?? ((a: Asset, q: EvaluationQuery) => evaluate(a, q));
  const state = run(asset, query);
  if (query.kind === "pose") return solveThermal(state, environment, options);
  const cache = new Map<number, ThermalSample>();
  const history = (timeS: number): ThermalSample => {
    const hit = cache.get(timeS);
    if (hit) return hit;
    const s = run(asset, { ...query, timeS });
    const sample = { inputs: s.thermal, comVelocity: s.kinematics.comVelocity };
    cache.set(timeS, sample);
    return sample;
  };
  return solveThermal(state, environment, { ...options, history });
}
