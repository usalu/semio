/** 🧮️ Lagrangian parcels in world space on a global fixed step grid (h = 1/120 s): birth, integration, culling and an incremental cache that is bitwise identical to a cold evaluation.
 *
 * Parcel model (integral jet/plume "slice" model). A parcel is a slice of a stream of duration `τ` (its emission interval) with dry-air mass `m_d`, relative velocity `u` (to the local wind),
 * total enthalpy `h` and total water `w_t` per kg dry air. Volume `V = m_d·v(T, w_v)`, top-hat radius `b = √(V/(π u_e τ))` with `u_e = √(|u|² + σ_u²)`, which for a steady stream is its cross-section radius.
 * - Entrainment (Morton, Taylor & Turner 1956; Fischer et al. 1979): lateral inflow `dV_e/dt = 2π b ℓ α u_e = 2 α u_e V / b`, so `db/dt = 2α|u|`; for a momentum jet this is `u = U₀/(1 + 2αx/r₀)` and `Q ∝ x`, the round turbulent jet (Pope 2000 ch. 5).
 *   `α(Ri) = α_j + (α_p − α_j)·min(1, Ri/Ri_p)`, `Ri = |g'| b/u_e²`, `Ri_p = 8α_p/5` (the pure plume, MTT top-hat), after Kaminski, Tait & Carazzo (2005); `α_j = 0.076`, `α_p = 0.116` (top-hat values of the Gaussian 0.0535 and 0.083 times √2).
 * - Momentum relative to the wind: `du/dt = g' ŷ − (ṁ_t/m_t) u` with the total (moist) masses `m_t = m_d (1 + w_t)`, `ṁ_t = ṁ_d (1 + w_a)`; entrained ambient air arrives at rest relative to the wind; added mass and shear are neglected.
 * - Mixing conserves `h` and `w_t` per kg dry air: `d(h, w_t)/dt = (ṁ_d/m_d)(ambient − parcel)`; temperature, vapour and condensate come from saturation adjustment. Buoyancy `g' = g((ρ_a V − m)/m)` uses the true densities including vapour deficit and liquid loading.
 * - Position (three-dimensional: ahead x, up y, left z): `dx/dt = u_wind(y) + u` (RK4), plus a random-walk increment `σ_u √(2 T_L h)·ξ` per axis and step with `ξ` hashed from (seed, parcel, axis, step index). The wind is horizontal, the jet momentum decays isotropically and buoyancy acts along y, so every view projects the same parcels.
 * - Ground: `y ≥ 0` (the parcel slides along it with `u_y ≥ 0`).
 * Culling: age above `maxAgeS`, or temperature excess < 0.05 K and vapour excess < 0.05 g/kg and no condensate (indistinguishable from ambient).
 * Pure function of absolute time: the parcels of time `t` are those born on the emission grid within `maxAgeS`; each is integrated from its birth with the same arithmetic whatever the cache holds.
 *
 * @see https://doi.org/10.1017/CBO9780511840531 — Turner 1973, Buoyancy Effects in Fluids
 * @see https://doi.org/10.1098/rspa.1956.0011 — Morton, Taylor & Turner 1956
 * @see https://doi.org/10.1016/j.jvolgeores.2005.03.003 — Kaminski, Tait & Carazzo 2005, Turbulent entrainment in jets with arbitrary buoyancy
 */
import type { Asset, Environment } from "../../../🧬️schema/🟦️.ts";
import type { EnvironmentTimeline, EvaluatedState, EvaluationQuery, Parcel, Pathline, ThermalField, Vec3 } from "../../🔗️contracts/🟦️.ts";
import { type Ambient, type BoundaryLayer, type Turbulence, ambientAt } from "../🌬️ambient/🟦️.ts";
import { GRAVITY_MS2, moistFrom, specificHumidity, volumePerDryKg } from "../🌫️air/🟦️.ts";
import { gaussianPair } from "../🎲️noise/🟦️.ts";
import { type Emission, type World, STEPS_PER_S, emissionSteps, emissionsAt } from "../🔥️sources/🟦️.ts";

/** 🧮️ Top-hat entrainment coefficient of a jet. */
export const ALPHA_JET = 0.076;
/** 🧮️ Top-hat entrainment coefficient of a pure plume. */
export const ALPHA_PLUME = 0.116;
/** 🧮️ Local Richardson number of the pure plume, `8α_p/5`. */
export const RI_PLUME = (8 * ALPHA_PLUME) / 5;
/** 〰️ Pathline sampling stride in grid steps (20 Hz). */
export const PATH_STRIDE = 6;
/** 🌬️ Standard deviation (rad) of the direction of an exhaled parcel about the nozzle axis: the half-angle of a turbulent top-hat jet, `atan(2 α_j)`, so an exhale fans out like a dispersing jet instead of piling up on one ray. */
export const BREATH_SPREAD_RAD = Math.atan(2 * ALPHA_JET);
/** 🧮️ Excess thresholds below which a parcel is culled. */
export const CULL_EXCESS_K = 0.05;
export const CULL_VAPOUR_KGKG = 5e-5;

/** 🎛️ Resolved solver settings. */
export type Resolved = { readonly maxAgeS: number; readonly seed: number; readonly layer: BoundaryLayer | null; readonly turbulence: Turbulence; readonly periodSteps: number };

type Rec = {
  readonly emission: Emission;
  readonly key: number;
  readonly birthStep: number;
  readonly tauS: number;
  readonly s: Float64Array;
  step: number;
  dead: number;
  path: number[];
};

const X = 0;
const Y = 1;
const Z = 2;
const UX = 3;
const UY = 4;
const UZ = 5;
const MD = 6;
const H = 7;
const WT = 8;
const SIZE = 9;
const STEP_S = 1 / STEPS_PER_S;
const LATERAL_KEY = 1 << 20;

/** 🧮️ What a parcel looks like at a state: thermodynamics, radius and the derivative of the state vector. */
function derive(s: Float64Array, a: Ambient, tau: number, out: Float64Array): { readonly tC: number; readonly wv: number; readonly wl: number; readonly b: number } {
  const m = moistFrom(s[H]!, s[WT]!, a.pressurePa);
  const v = volumePerDryKg(m.tC, m.wv, a.pressurePa);
  const volume = s[MD]! * v;
  const speed = Math.hypot(s[UX]!, s[UY]!, s[UZ]!);
  const ue = Math.hypot(speed, a.sigmaMS);
  const b = Math.sqrt(volume / (Math.PI * ue * tau));
  const buoyancy = (GRAVITY_MS2 * ((1 + a.wv) * (v / a.v) - (1 + s[WT]!))) / (1 + s[WT]!);
  const ri = (Math.abs(buoyancy) * b) / (ue * ue);
  const alpha = ALPHA_JET + (ALPHA_PLUME - ALPHA_JET) * Math.min(1, ri / RI_PLUME);
  const dmd = (2 * alpha * ue * volume) / b / a.v;
  const rate = dmd / s[MD]!;
  const momentumRate = (dmd * (1 + a.wv)) / (s[MD]! * (1 + s[WT]!));
  const wind = a.windAt(Math.max(0, s[Y]!));
  out[X] = wind.x + s[UX]!;
  out[Y] = wind.y + s[UY]!;
  out[Z] = wind.z + s[UZ]!;
  out[UX] = -momentumRate * s[UX]!;
  out[UY] = buoyancy - momentumRate * s[UY]!;
  out[UZ] = -momentumRate * s[UZ]!;
  out[MD] = dmd;
  out[H] = rate * (a.h - s[H]!);
  out[WT] = rate * (a.wv - s[WT]!);
  return { ...m, b };
}

const SCRATCH = [0, 1, 2, 3, 4].map(() => new Float64Array(SIZE)) as [Float64Array, Float64Array, Float64Array, Float64Array, Float64Array];

/** 🧮️ Largest entrainment-rate × step product of one RK4 sub-step (the near field of a jet has a time scale `r₀/(2αU₀)` shorter than the 1/120 s grid step). */
export const MAX_RATE_STEP = 0.2;
/** 🧮️ Upper bound of the sub-steps per grid step. */
export const MAX_SUBSTEPS = 256;

function rk4(s: Float64Array, dt: number, a0: Ambient, a1: Ambient, a2: Ambient, tau: number): void {
  const probe = SCRATCH[0]!;
  derive(s, a0, tau, probe);
  const m = Math.min(MAX_SUBSTEPS, Math.max(1, Math.ceil((probe[MD]! / s[MD]! * dt) / MAX_RATE_STEP)));
  if (m === 1) return rk4Step(s, dt, a0, a1, a2, tau);
  for (let i = 0; i < m; i++) rk4Step(s, dt / m, i === 0 ? a0 : a1, a1, i === m - 1 ? a2 : a1, tau);
}

function rk4Step(s: Float64Array, dt: number, a0: Ambient, a1: Ambient, a2: Ambient, tau: number): void {
  const [k1, k2, k3, k4, tmp] = SCRATCH;
  derive(s, a0, tau, k1!);
  for (let i = 0; i < SIZE; i++) tmp![i] = s[i]! + 0.5 * dt * k1![i]!;
  derive(tmp!, a1, tau, k2!);
  for (let i = 0; i < SIZE; i++) tmp![i] = s[i]! + 0.5 * dt * k2![i]!;
  derive(tmp!, a1, tau, k3!);
  for (let i = 0; i < SIZE; i++) tmp![i] = s[i]! + dt * k3![i]!;
  derive(tmp!, a2, tau, k4!);
  for (let i = 0; i < SIZE; i++) s[i] = s[i]! + (dt / 6) * (k1![i]! + 2 * k2![i]! + 2 * k3![i]! + k4![i]!);
  if (s[Y]! < 0) {
    s[Y] = 0;
    if (s[UY]! < 0) s[UY] = 0;
  }
}

/** 🧮️ Everything one (asset, query, environment, settings) solution caches; the caches only store pure functions of the key, so they never change results. */
export type Track = {
  readonly world: World;
  readonly settings: Resolved;
  readonly environment: Environment | EnvironmentTimeline;
  readonly ambientHalf: Map<number, Ambient>;
  readonly emissions: Map<string, readonly Emission[]>;
  readonly recs: Map<string, Rec>;
};

/** 🧮️ Creates a track around the evaluation and thermal functions. */
export function createTrack(asset: Asset, query: EvaluationQuery, environment: Environment | EnvironmentTimeline, settings: Resolved, evaluate: (asset: Asset, query: EvaluationQuery) => EvaluatedState, thermal: (state: EvaluatedState, environment: Environment) => ThermalField): Track {
  const states = new Map<number, EvaluatedState>();
  const thermals = new Map<number, ThermalField>();
  const queryAt = (timeS: number): EvaluationQuery => ({ ...query, timeS }) as EvaluationQuery;
  const world: World = {
    asset,
    query,
    queryAt,
    stateAt: (timeS) => {
      let state = states.get(timeS);
      if (state === undefined) {
        if (states.size > 8192) states.clear();
        state = evaluate(asset, queryAt(timeS));
        states.set(timeS, state);
      }
      return state;
    },
    ambientAt: (timeS) => ambientAt(environment, timeS, settings.layer, settings.turbulence),
    thermalAt: (timeS) => {
      let field = thermals.get(timeS);
      if (field === undefined) {
        if (thermals.size > 4096) thermals.clear();
        field = thermal(world.stateAt(timeS), ambientAt(environment, timeS, settings.layer, settings.turbulence).environment);
        thermals.set(timeS, field);
      }
      return field;
    },
  };
  return { world, settings, environment, ambientHalf: new Map(), emissions: new Map(), recs: new Map() };
}

function halfAmbient(track: Track, index: number): Ambient {
  let a = track.ambientHalf.get(index);
  if (a === undefined) {
    if (track.ambientHalf.size > 65536) track.ambientHalf.clear();
    a = track.world.ambientAt(index / (2 * STEPS_PER_S));
    track.ambientHalf.set(index, a);
  }
  return a;
}

function born(track: Track, emitterIndex: number, k: number): readonly Emission[] {
  const id = `${emitterIndex}:${k}`;
  let list = track.emissions.get(id);
  if (list === undefined) {
    if (track.emissions.size > 65536) track.emissions.clear();
    list = emissionsAt(track.world, emitterIndex, k);
    track.emissions.set(id, list);
  }
  return list;
}

const periodic = (track: Track, step: number): number => (track.settings.periodSteps > 0 ? step % track.settings.periodSteps : step);

const cross = (a: readonly [number, number, number], b: readonly [number, number, number]): [number, number, number] => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];

/** 🌬️ The velocity of an exhaled parcel: the nozzle velocity turned by independent Gaussian angles (σ = {@link BREATH_SPREAD_RAD}, clipped at 2σ) in two perpendicular planes through the nozzle axis, so a view along any axis sees the same half-angle. */
function fanned(emission: Emission, spreadA: number, spreadB: number): [number, number, number] {
  const speed = Math.hypot(emission.ux, emission.uy, emission.uz);
  if (speed === 0) return [0, 0, 0];
  const d: [number, number, number] = [emission.ux / speed, emission.uy / speed, emission.uz / speed];
  const reference: [number, number, number] = Math.hypot(d[0], d[1]) > 1e-9 ? [0, 0, 1] : [0, 1, 0];
  const p1 = cross(reference, d);
  const n1 = Math.hypot(...p1);
  const e1: [number, number, number] = [p1[0] / n1, p1[1] / n1, p1[2] / n1];
  const e2 = cross(d, e1);
  const ca = Math.cos(spreadA);
  const sa = Math.sin(spreadA);
  const cb = Math.cos(spreadB);
  const sb = Math.sin(spreadB);
  const turned = [0, 1, 2].map((i) => d[i]! * ca + e1[i]! * sa) as [number, number, number];
  return [0, 1, 2].map((i) => speed * (turned[i]! * cb + e2[i]! * sb)) as [number, number, number];
}

function spawn(track: Track, emission: Emission, k: number, steps: number): Rec {
  const birthStep = k * steps + steps / 2;
  const breath = track.world.asset.thermal.emitters[emission.emitterIndex]?.kind === "breath";
  const [ga, gb] = breath ? gaussianPair(track.settings.seed, emission.emitterIndex * 64 + emission.anchorIndex + 32, periodic(track, birthStep)) : [0, 0];
  const clip = (g: number): number => BREATH_SPREAD_RAD * Math.max(-2, Math.min(2, g));
  const [ux, uy, uz] = breath ? fanned(emission, clip(ga), clip(gb)) : [emission.ux, emission.uy, emission.uz];
  const s = new Float64Array([emission.at.x, emission.at.y, emission.at.z, ux, uy, uz, emission.md, emission.h, emission.wt]);
  return { emission, key: emission.emitterIndex * 64 + emission.anchorIndex, birthStep, tauS: steps / STEPS_PER_S, s, step: birthStep, dead: -1, path: [emission.at.x, emission.at.y, emission.at.z] };
}

function advance(track: Track, rec: Rec): void {
  const n = rec.step;
  const a0 = halfAmbient(track, 2 * n);
  rk4(rec.s, STEP_S, a0, halfAmbient(track, 2 * n + 1), halfAmbient(track, 2 * n + 2), rec.tauS);
  const t = track.settings.turbulence;
  const sigma = a0.sigmaMS * Math.sqrt(2 * t.lagrangianTimeS * STEP_S);
  const [gx, gy] = gaussianPair(track.settings.seed, rec.key, periodic(track, n));
  const [gz] = gaussianPair(track.settings.seed, rec.key + LATERAL_KEY, periodic(track, n));
  rec.s[X] = rec.s[X]! + sigma * gx;
  rec.s[Y] = Math.max(0, rec.s[Y]! + sigma * gy);
  rec.s[Z] = rec.s[Z]! + sigma * gz;
  rec.step = n + 1;
  if ((rec.step - rec.birthStep) % PATH_STRIDE === 0) rec.path.push(rec.s[X]!, rec.s[Y]!, rec.s[Z]!);
  const a = halfAmbient(track, 2 * rec.step);
  const m = moistFrom(rec.s[H]!, rec.s[WT]!, a.pressurePa);
  const quiet = Math.abs(m.tC - a.tC) < CULL_EXCESS_K && Math.abs(specificHumidity(m.wv) - specificHumidity(a.wv)) < CULL_VAPOUR_KGKG && m.wl < 1e-7;
  if (quiet || rec.step - rec.birthStep >= track.settings.maxAgeS * STEPS_PER_S || !Number.isFinite(rec.s[X]! + rec.s[Y]! + rec.s[Z]!)) rec.dead = rec.step;
}

/** 🧮️ The live parcels and pathlines of time `timeS`; cooperative cancellation through `signal`, progress per parcel. */
export function parcelsAt(track: Track, timeS: number, wantPaths: boolean, signal?: AbortSignal, progress?: (done: number, total: number) => void): { readonly parcels: readonly Parcel[]; readonly pathlines: readonly Pathline[] } {
  const n = Math.floor(timeS * STEPS_PER_S + 1e-9);
  const frac = Math.max(0, timeS - n / STEPS_PER_S);
  const maxSteps = track.settings.maxAgeS * STEPS_PER_S;
  const emitters = track.world.asset.thermal.emitters;
  const jobs: { readonly e: number; readonly k: number; readonly steps: number }[] = [];
  emitters.forEach((emitter, e) => {
    const steps = emissionSteps(emitter);
    const last = Math.floor((n - steps / 2) / steps);
    const first = Math.max(0, Math.ceil((n - maxSteps - steps / 2) / steps));
    for (let k = first; k <= last; k++) jobs.push({ e, k, steps });
  });
  const parcels: Parcel[] = [];
  const pathlines: Pathline[] = [];
  let done = 0;
  const scratch = new Float64Array(SIZE);
  for (const { e, k, steps } of jobs) {
    if (signal?.aborted) throw new DOMException("transport solve cancelled", "AbortError");
    progress?.(done++, jobs.length);
    const emissions = born(track, e, k);
    emissions.forEach((emission, index) => {
      const id = `${e}:${k}:${index}`;
      let rec = track.recs.get(id);
      if (rec === undefined || rec.step > n) {
        rec = spawn(track, emission, k, steps);
        track.recs.set(id, rec);
      }
      while (rec.step < n && rec.dead < 0) advance(track, rec);
      if (rec.dead >= 0 && rec.dead <= n) return;
      scratch.set(rec.s);
      const t0 = n / STEPS_PER_S;
      const a0 = track.world.ambientAt(t0);
      if (frac > 0) rk4(scratch, frac, a0, track.world.ambientAt(t0 + frac / 2), track.world.ambientAt(t0 + frac), rec.tauS);
      const a = track.world.ambientAt(timeS);
      const state = derive(scratch, a, rec.tauS, new Float64Array(SIZE));
      const wind = a.windAt(Math.max(0, scratch[Y]!));
      const parcelId = emissions.length > 1 || emitters[e]!.kind === "transpiration" ? `${emission.emitter}@${index}#${k}` : `${emission.emitter}#${k}`;
      parcels.push({
        id: parcelId,
        emitter: emission.emitter,
        bornS: rec.birthStep / STEPS_PER_S,
        ageS: (n - rec.birthStep) / STEPS_PER_S + frac,
        at: { x: scratch[X]!, y: scratch[Y]!, z: scratch[Z]! },
        velocity: { x: wind.x + scratch[UX]!, y: wind.y + scratch[UY]!, z: wind.z + scratch[UZ]! },
        radiusM: state.b,
        excessK: state.tC - a.tC,
        vapourExcessKgPerKg: specificHumidity(state.wv) - specificHumidity(a.wv),
        condensateKgPerKg: state.wl / (1 + scratch[WT]!),
        carries: emission.carries,
      });
      if (wantPaths) {
        const points: Vec3[] = [];
        const ages: number[] = [];
        for (let i = 0; i < rec.path.length; i += 3) {
          const stepIndex = (i / 3) * PATH_STRIDE;
          if (rec.birthStep + stepIndex >= n) break;
          points.push({ x: rec.path[i]!, y: rec.path[i + 1]!, z: rec.path[i + 2]! });
          ages.push(stepIndex / STEPS_PER_S);
        }
        points.push({ x: scratch[X]!, y: scratch[Y]!, z: scratch[Z]! });
        ages.push((n - rec.birthStep) / STEPS_PER_S + frac);
        pathlines.push({ parcel: parcelId, emitter: emission.emitter, points, ages });
      }
    });
  }
  for (const [id, rec] of track.recs) if (rec.birthStep + maxSteps + 2 * STEPS_PER_S < n) track.recs.delete(id);
  progress?.(jobs.length, jobs.length);
  return { parcels, pathlines };
}
