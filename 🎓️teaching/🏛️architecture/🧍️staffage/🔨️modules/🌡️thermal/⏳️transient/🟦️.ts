/** ⏳️ Transient response: first-order relaxation of a lumped region towards its steady state with time constant `τ = C / G` (heat capacity over restoring conductance).
 *
 * The exponential integrator `T(t + Δt) = T* + (T − T*) exp(−Δt/τ)` is exact for constant forcing, unconditionally stable and monotone: it cannot overshoot or
 * oscillate. The response at time `t` is a pure function of the input history: the history starts at an epoch boundary `t₀` (see {@link historyPlan}) in the steady state of the inputs at
 * `t₀` (at `t = 0` this is the stated assumption; for looping clips the window of several time constants covers many periods, giving the periodic steady state
 * up to `exp(−window/τ)`); the grid and the start never depend on the query time.
 *
 * @see ../../../🧬️schema/🟦️.ts — ThermalRegion.heatCapacityJM2K
 */

/** 📉️ Relaxes `current` towards `target` over `dtS` with time constant `tauS`. */
export const relax = (current: number, target: number, dtS: number, tauS: number): number => (tauS <= 1e-9 ? target : target + (current - target) * Math.exp(-dtS / tauS));

/** 📜️ A history plan: the absolute time the integration starts at (in the steady state of the inputs of that time), the step of its grid and the strictly increasing times it steps through, ending at the requested time. */
export type HistoryPlan = { readonly startS: number; readonly stepS: number; readonly times: readonly number[] };

/** 🗓️ Plans the integration up to `timeS` on a grid that depends only on the asset and the session, never on the query time. `tauMaxS` is the reference time constant of the asset (its slowest response at the inputs of t = 0), the window is `W = 7 τ_max` capped at `maxWindowS` (3 h) or `windowS`,
 * the step is `minStepS · 2^k` (0.5 s base) for the smallest `k` with `2 W / step ≤ maxSteps` (180), and the grid points are multiples of the step on the absolute timeline. The history starts at an epoch boundary: epochs are `E = ⌈W / step⌉ · step` long and a query in epoch `n`
 * starts at `(n − 1) E` (at 0 for the first epoch) in the steady state of the inputs of that time, so the history before the query is between `W` and `2 W` long (the periodic steady state of a looping clip up to `exp(−W/τ)`), all queries of one epoch share ONE start and ONE chain, and
 * frames never change grid: scrubbing, playing and a cold solve integrate the same steps from the same start and are bit-identical. The last step is the partial step from the last grid point to `timeS`. */
export function historyPlan(timeS: number, tauMaxS: number, options: { readonly windowS?: number; readonly maxWindowS?: number; readonly minStepS?: number; readonly maxSteps?: number } = {}): HistoryPlan {
  const window = options.windowS ?? Math.min(options.maxWindowS ?? 10800, 7 * tauMaxS);
  const base = options.minStepS ?? 0.5;
  const maxSteps = options.maxSteps ?? 180;
  const stepS = base * 2 ** Math.max(0, Math.ceil(Math.log2(Math.max(1e-9, (2 * window) / maxSteps / base))));
  const epochS = Math.max(1, Math.ceil(window / stepS)) * stepS;
  const startS = Math.max(0, (Math.floor((timeS + 1e-9) / epochS) - 1) * epochS);
  const times: number[] = [];
  for (let k = Math.floor(startS / stepS) + 1; k * stepS < timeS - 1e-9; k++) times.push(k * stepS);
  if (timeS > startS) times.push(timeS);
  return { startS, stepS, times };
}
