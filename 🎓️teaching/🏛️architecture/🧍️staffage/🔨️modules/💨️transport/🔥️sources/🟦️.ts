/** 🔥️ Category-specific sources of the transport model: what each emitter releases at an instant and per emission interval.
 *
 * - `breath` (nose, mouth): the exact exhaled volume of the interval (integral of the half-sine flow, routed by the pattern's nose/mouth split) at the authored exhaled temperature and humidity; zero during inhalation and pauses. Never used by other categories.
 * - `exhaust`, `vent`, `flue`, `fan`: level `L = operating[operatingKey] × emitters[id]` scales volume flow, temperature rise and vapour excess (`volumeFlowM3S`, `temperatureRiseK`, `vapourKgPerKg` are the values at level 1), so ignition and shutdown ramps follow the tracks.
 * - `plume`: free convection above a warm (or cold) surface; the convective heat `Q` of the `region` (or of all regions) solved by the thermal model is released continuously as air with a fixed source excess `PLUME_SOURCE_EXCESS_K` and volume flow `Q/(ρ c_p ΔT₀)`; strength `emitters[id]`; buoyancy sets the direction (up when warmer than the ambient air, down when colder), so a plume above a bowed head still rises vertically and the emitter's `directionDeg` and bone angle do not tilt it.
 * - `exhaust`, `vent`, `flue`, `fan` with a `region`: the vapour mass rate of that region's evaporation (surface evaporation plus phase change, `RegionField.evaporationKgS`) leaves as air with the source vapour ratio of `vapourKgPerKg`, heated by `temperatureRiseK`; the dry-air flow is `ṁ_v / (w_source − w_ambient)`, scaled by the emitter strength and operating level (a kettle's steam follows its boiling).
 * - `transpiration`: vapour mass flow `latent flux of the region / h_fg` spread equally over the `anchors`, carried by air at ambient temperature with zero momentum (no sensible jet); the source air carries half the leaf-saturation vapour excess.
 *
 * Emission grid: a parcel `k` of an emitter is born at step `k·S + S/2` of the global 1/120 s grid (S steps per interval) and represents the interval `[kS, (k+1)S)`.
 *
 * @see ../../../🧬️schema/🟦️.ts — `Emitter`, `OperatingKey`
 * @see ../🫁️respiration/🟦️.ts — flow, exhaled volume, routing
 */
import type { Asset, Emitter, Slug } from "../../../🧬️schema/🟦️.ts";
import type { EmitterState, EvaluatedState, EvaluationQuery, ThermalField, Vec3 } from "../../🔗️contracts/🟦️.ts";
import type { Ambient } from "../🌬️ambient/🟦️.ts";
import { CP_DRY_AIR, enthalpy, latentHeat, moistDensity, ratioOfSpecific, saturationRatio, specificHumidity, volumePerDryKg } from "../🌫️air/🟦️.ts";
import { breathPatternAt, emitterOwner, exhaledAir, exhaledVolumeM3, respirationState, routeShares } from "../🫁️respiration/🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** ⏱️ Integration steps per second of the global step grid (h = 1/120 s). */
export const STEPS_PER_S = 120;

/** 🌡️ Source excess temperature (K) of the free-convection plume parcels (numerical; sets the source volume flow, not the heat). */
export const PLUME_SOURCE_EXCESS_K = 2;

/** 🌿️ Smallest source vapour excess (kg/kg) of transpiration parcels. */
export const TRANSPIRATION_MIN_EXCESS = 1e-3;

/** 🧊️ Smallest source vapour excess (kg/kg dry air) of evaporation-driven vents. */
export const EVAPORATION_MIN_EXCESS = 1e-3;

/** 🌍️ Everything a source needs from the solver. */
export type World = {
  readonly asset: Asset;
  readonly query: EvaluationQuery;
  readonly stateAt: (timeS: number) => EvaluatedState;
  readonly queryAt: (timeS: number) => EvaluationQuery;
  readonly ambientAt: (timeS: number) => Ambient;
  readonly thermalAt: (timeS: number) => ThermalField;
};

/** 💨️ One parcel to be born: placement, momentum and the conserved thermodynamic content. */
export type Emission = { readonly emitter: Slug; readonly emitterIndex: number; readonly anchorIndex: number; readonly at: Vec3; readonly ux: number; readonly uy: number; readonly uz: number; readonly md: number; readonly h: number; readonly wt: number; readonly carries: "airflow" | "vapour" };

/** ⏲️ Emission interval of an emitter in grid steps: breath 1/30 s, exhaust-like 1/15 s, plume 1/10 s, transpiration 1/2 s. */
export const emissionSteps = (emitter: Emitter): number => (emitter.kind === "breath" ? 4 : emitter.kind === "plume" ? 12 : emitter.kind === "transpiration" ? 60 : 8);

/** 📊️ The thermal quantities are sampled on a 0.5 s grid (plume and transpiration strength change slowly). */
export const THERMAL_SAMPLE_S = 0.5;

const area = (emitter: Emitter): number => (Math.PI * emitter.diameterM * emitter.diameterM) / 4;
const clamp01 = (x: number): number => Math.min(1, Math.max(0, x));

function anchorWorld(world: World, state: EvaluatedState, emitter: Emitter, anchorId: Slug, rises: number | null): { readonly at: Vec3; readonly direction: Vec3 } {
  const placed = state.anchors[anchorId];
  if (placed === undefined) throw new Error(`emitter "${emitter.id}" of asset "${world.asset.id}" names anchor "${anchorId}" which the evaluated state does not place`);
  const at = { x: placed.at.x, y: placed.at.y, z: placed.z };
  if (rises !== null) return { at, direction: { x: 0, y: rises, z: 0 } };
  const frame = state.bones[placed.anchor.bone]?.frame;
  const a = (emitter.directionDeg * Math.PI) / 180;
  const c = portable.cos(a);
  const s = portable.sin(a);
  if (frame === undefined) return { at, direction: { x: c, y: s, z: 0 } };
  return { at, direction: { x: c * frame.u.x + s * frame.v.x, y: c * frame.u.y + s * frame.v.y, z: c * frame.u.z + s * frame.v.z } };
}

const risesOf = (emitter: Emitter, hot: boolean): number | null => (emitter.kind === "plume" ? (hot ? 1 : -1) : null);

function level(state: EvaluatedState, emitter: Emitter): number {
  const strength = clamp01(state.thermal.emitters[emitter.id] ?? 1);
  const key = emitter.operatingKey === undefined ? 1 : Math.max(0, state.thermal.operating[emitter.operatingKey] ?? 0);
  return strength * key;
}

function thermalSample(world: World, timeS: number): ThermalField {
  return world.thermalAt(Math.floor(timeS / THERMAL_SAMPLE_S + 1e-9) * THERMAL_SAMPLE_S);
}

function plumeHeatW(world: World, emitter: Emitter, timeS: number): number {
  const field = thermalSample(world, timeS);
  return emitter.region === undefined ? field.totals.sensibleW : (field.regions[emitter.region]?.sensibleW ?? 0);
}

function transpirationKgS(world: World, emitter: Emitter, timeS: number, tC: number): { readonly kgS: number; readonly leafTempC: number } {
  const field = thermalSample(world, timeS);
  const region = emitter.region === undefined ? undefined : field.regions[emitter.region];
  return { kgS: Math.max(0, region?.latentW ?? 0) / latentHeat(tC), leafTempC: region?.surfaceTempC ?? tC };
}

/** 🌬️ The air an emitter releases at an instant: volume flow (m³/s, ≥ 0), exit velocity, temperature (°C) and vapour ratio; breath uses the instantaneous flow, others the operating level. */
export function flowAt(world: World, emitterIndex: number, timeS: number): { readonly flowM3S: number; readonly tC: number; readonly wv: number; readonly state: EvaluatedState } {
  const emitter = world.asset.thermal.emitters[emitterIndex]!;
  const state = world.stateAt(timeS);
  const ambient = world.ambientAt(timeS);
  if (emitter.kind === "breath") {
    const owner = emitterOwner(world.asset, emitter.id);
    if (owner === null) return { flowM3S: 0, tC: ambient.tC, wv: ambient.wv, state };
    const query = world.queryAt(timeS);
    const rs = respirationState(world.asset, query, owner.subject);
    const pattern = breathPatternAt(world.asset, query, owner.subject);
    const shares = pattern === null ? { nose: 0, mouth: 0 } : routeShares(owner.respiration, pattern);
    const share = owner.respiration.noseEmitter === emitter.id ? shares.nose : shares.mouth;
    const air = exhaledAir(owner.respiration, ambient.pressurePa);
    return { flowM3S: Math.max(0, rs?.flowM3S ?? 0) * share, tC: air.tC, wv: air.wv, state };
  }
  const l = level(state, emitter);
  if (emitter.kind === "plume") {
    const q = plumeHeatW(world, emitter, timeS) * clamp01(state.thermal.emitters[emitter.id] ?? 1);
    const excess = Math.sign(q) * PLUME_SOURCE_EXCESS_K;
    return { flowM3S: Math.abs(q) / (moistDensity(ambient.tC, ambient.wv, ambient.pressurePa) * CP_DRY_AIR * PLUME_SOURCE_EXCESS_K), tC: ambient.tC + excess, wv: ambient.wv, state };
  }
  if (emitter.kind === "transpiration") {
    const { kgS, leafTempC } = transpirationKgS(world, emitter, timeS, ambient.tC);
    const excess = Math.max(TRANSPIRATION_MIN_EXCESS, 0.5 * (saturationRatio(leafTempC, ambient.pressurePa) - ambient.wv));
    return { flowM3S: kgS * l === 0 ? 0 : (kgS * l * volumePerDryKg(ambient.tC, ambient.wv, ambient.pressurePa)) / excess, tC: ambient.tC, wv: ambient.wv + excess, state };
  }
  const wvSource = emitter.vapourKgPerKg === undefined ? ambient.wv : ratioOfSpecific(emitter.vapourKgPerKg);
  if (emitter.region !== undefined) {
    const excess = Math.max(EVAPORATION_MIN_EXCESS, wvSource - ambient.wv);
    const kgS = Math.max(0, thermalSample(world, timeS).regions[emitter.region]?.evaporationKgS ?? 0) * l;
    return { flowM3S: kgS === 0 ? 0 : (kgS * volumePerDryKg(ambient.tC + (emitter.temperatureRiseK ?? 0), wvSource, ambient.pressurePa)) / excess, tC: ambient.tC + (emitter.temperatureRiseK ?? 0), wv: wvSource, state };
  }
  return { flowM3S: (emitter.volumeFlowM3S ?? 0) * l, tC: ambient.tC + (emitter.temperatureRiseK ?? 0) * l, wv: ambient.wv + (wvSource - ambient.wv) * l, state };
}

/** 📌️ The anchors an emitter releases from: its `anchors` (transpiration over the crown) or its single `anchor`. */
export const anchorsOf = (emitter: Emitter): readonly Slug[] => (emitter.kind === "transpiration" && emitter.anchors !== undefined && emitter.anchors.length > 0 ? emitter.anchors : [emitter.anchor]);

/** 🫧️ The parcels emitted by emitter `emitterIndex` in interval `k` (one per anchor), `[]` when nothing flows. */
export function emissionsAt(world: World, emitterIndex: number, k: number): readonly Emission[] {
  const emitter = world.asset.thermal.emitters[emitterIndex]!;
  const steps = emissionSteps(emitter);
  const begin = (k * steps) / STEPS_PER_S;
  const end = ((k + 1) * steps) / STEPS_PER_S;
  const birth = (k * steps + steps / 2) / STEPS_PER_S;
  const interval = end - begin;
  const { flowM3S, tC, wv, state } = flowAt(world, emitterIndex, birth);
  const ambient = world.ambientAt(birth);
  let volumeM3 = flowM3S * interval;
  if (emitter.kind === "breath") {
    const owner = emitterOwner(world.asset, emitter.id);
    const query = world.queryAt(birth);
    const pattern = owner === null ? null : breathPatternAt(world.asset, query, owner.subject);
    const shares = owner === null || pattern === null ? { nose: 0, mouth: 0 } : routeShares(owner.respiration, pattern);
    const share = owner?.respiration.noseEmitter === emitter.id ? shares.nose : shares.mouth;
    volumeM3 = owner === null ? 0 : exhaledVolumeM3(world.asset, query, begin, end, owner.subject) * share;
  }
  if (!(volumeM3 > 0)) return [];
  const anchors = anchorsOf(emitter);
  const vapourKgS = emitter.kind === "transpiration" ? transpirationKgS(world, emitter, birth, ambient.tC).kgS * level(state, emitter) : null;
  const out: Emission[] = [];
  anchors.forEach((anchorId, anchorIndex) => {
    const placed = anchorWorld(world, state, emitter, anchorId, risesOf(emitter, tC >= ambient.tC));
    const v = volumeM3 / (emitter.kind === "transpiration" ? anchors.length : 1);
    const speed = emitter.kind === "transpiration" ? 0 : v / interval / area(emitter);
    out.push({
      emitter: emitter.id,
      emitterIndex,
      anchorIndex,
      at: placed.at,
      ux: speed * placed.direction.x,
      uy: speed * placed.direction.y,
      uz: speed * placed.direction.z,
      md: vapourKgS === null ? v / volumePerDryKg(tC, wv, ambient.pressurePa) : (vapourKgS * interval) / anchors.length / (wv - ambient.wv),
      h: enthalpy(tC, wv, 0),
      wt: wv,
      carries: emitter.kind === "transpiration" ? "vapour" : "airflow",
    });
  });
  return out;
}

/** 🚦️ The state of every emitter at an instant (position, direction, flow, exit velocity, exit state). */
export function emitterStates(world: World, timeS: number): readonly EmitterState[] {
  return world.asset.thermal.emitters.map((emitter, index) => {
    const f = flowAt(world, index, timeS);
    const placed = anchorWorld(world, f.state, emitter, emitter.anchor, risesOf(emitter, f.tC >= world.ambientAt(timeS).tC));
    const split = emitter.kind === "transpiration" ? anchorsOf(emitter).length : 1;
    return {
      emitter: emitter.id,
      kind: emitter.kind,
      at: placed.at,
      direction: placed.direction,
      volumeFlowM3S: f.flowM3S,
      exitVelocityMS: emitter.kind === "transpiration" ? 0 : f.flowM3S / split / area(emitter),
      temperatureC: f.tC,
      vapourKgPerKg: specificHumidity(f.wv),
      active: f.flowM3S > 0,
    };
  });
}
