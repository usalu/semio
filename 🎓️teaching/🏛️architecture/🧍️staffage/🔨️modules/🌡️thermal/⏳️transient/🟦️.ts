/** ⏳️ Transient response: first-order relaxation of a lumped region towards its steady state with time constant `τ = C / G` (heat capacity over restoring conductance).
 *
 * The exponential integrator `T(t + Δt) = T* + (T − T*) exp(−Δt/τ)` is exact for constant forcing, unconditionally stable and monotone: it cannot overshoot or
 * oscillate. The response at time `t` is a pure function of the input history: the history starts at `t₀ = max(0, t − window)` in the steady state of the inputs at
 * `t₀` (at `t = 0` this is the stated assumption; for looping clips the window of several time constants covers many periods, giving the periodic steady state
 * up to `exp(−window/τ)`).
 *
 * @see ../../../🧬️schema/🟦️.ts — ThermalRegion.heatCapacityJM2K
 */

/** ⏳️ Relaxes `current` towards `target` over `dtS` with time constant `tauS`. */
export const relax = (current: number, target: number, dtS: number, tauS: number): number => (tauS <= 1e-9 ? target : target + (current - target) * Math.exp(-dtS / tauS));

/** ⏳️ A history plan: the absolute time the integration starts at (in the steady state of the inputs of that time) and the strictly increasing times it steps through, ending at the requested time. */
export type HistoryPlan = { readonly startS: number; readonly times: readonly number[] };

/** ⏳️ Plans the integration up to `timeS`: window `7 τ_max` capped at `maxWindowS` (3 h), at most `maxSteps` steps (180). The step is `minStepS · 2^k` (0.5 s base) for the smallest `k` that fits,
 * and the grid points are multiples of the step on the absolute timeline, so the plan, and with it the solution, is a pure function of `(timeS, τ_max)` and the grid does not shift while time advances
 * within one step size: scrubbing to a time and playing to it give the same temperatures. The last step is the partial step from the last grid point to `timeS`. */
export function historyPlan(timeS: number, tauMaxS: number, options: { readonly windowS?: number; readonly maxWindowS?: number; readonly minStepS?: number; readonly maxSteps?: number } = {}): HistoryPlan {
  const window = options.windowS ?? Math.min(options.maxWindowS ?? 10800, 7 * tauMaxS);
  const base = options.minStepS ?? 0.5;
  const maxSteps = options.maxSteps ?? 180;
  const stepS = base * 2 ** Math.max(0, Math.ceil(Math.log2(Math.max(1e-9, window / maxSteps / base))));
  const first = Math.max(0, timeS - window);
  const startS = first === 0 ? 0 : Math.floor(first / stepS) * stepS;
  const times: number[] = [];
  for (let k = Math.floor(startS / stepS) + 1; k * stepS < timeS - 1e-9; k++) times.push(k * stepS);
  if (timeS > startS) times.push(timeS);
  return { startS, times };
}
