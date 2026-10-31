/** 🫀️ Thermoregulation of living bodies: a Gagge two-node model (core and skin) distributed over the body's thermal regions, plus respiratory heat exchange.
 *
 * State: one core temperature and one skin temperature per `body` region. Controls follow Gagge, Stolwijk & Nishi (1971) as implemented by pythermalcomfort
 * `two_nodes_gagge` (secondary source, ASHRAE 55 / ISO 7730 lineage): neutral set points `T_cr,0`, `T_sk,0` from the asset, `α = 0.1`,
 * `T_b = α T_sk + (1 − α) T_cr`, signals `WARM/COLD` of core, skin and body, skin blood flow `m_bl = (6.3 + 120 WARM_cr)/(1 + 0.5 κ COLD_sk)` clamped to the asset's
 * limits, regulatory sweating `m_rsw = 170 WARM_b exp(WARM_sk/10.7)` capped at 500 g/(m² h), evaporation `E_rsw = 0.68 m_rsw` W/m², skin wettedness
 * `w = 0.06 + 0.94 E_rsw/E_max` limited by `w_max = 0.38 v^−0.29` (bare) or `0.59 v^−0.08` (clothed), shivering `M_shiv = 19.4 COLD_sk COLD_cr` W/m².
 * Heat flows core → skin through tissue and blood: `(5.28 f_t + 1.163 m_bl,i)(T_cr − T_sk,i)` W/m² of skin; skin → outer surface through the region's layers
 * (resistances referred to the outer area, `I_cl f_cl`; outer area = `f` × skin area) and from there to the environment (🧱️balance); evaporation passes the vapour
 * resistance `R_v + 1/h_e` of the outer area, `E_max = f e Δp/(R_v + 1/h_e)` per skin area. Regional differences (distal vasoconstriction κ, tissue conductance f_t, sweat share, 5 % of the heat production released locally in proportion to the regions' metabolic shares) are
 * authored estimates; with one `trunk` region the model reduces to Gagge's. Respiration: `ṁ = ρ V̇` with `V̇ = rate × tidal volume` of the bound breathing
 * pattern (the same numbers drive the breath emitters and the chest motion), sensible `ṁ c_p (T_ex − T_a)` and latent `ṁ (w_ex − w_a) λ`; panting animals add
 * ventilation `min(7 m, 3 m WARM_cr)` L/min (authored-estimate).
 *
 * @see https://doi.org/10.1016/0020-7519(71)90001-2 — Gagge, Stolwijk & Nishi 1971, An effective temperature scale based on a simple model of human physiological regulatory response
 * @see https://pythermalcomfort.readthedocs.io/en/latest/_modules/pythermalcomfort/models/two_nodes.html
 */
import type { Asset, BodyPhysiology, BreathingPattern, Environment, Respiration, ThermalRegion } from "../../../🧬️schema/🟦️.ts";
import type { ThermalInputs } from "../../🔗️contracts/🟦️.ts";
import { CP_AIR, R_DRY, evaporativeCoefficient, humidityRatio, latentHeatJkg, saturationPressurePa, solveDecreasing } from "../🔬️physics/🟦️.ts";
import { type Ambient, type RegionFrame, type RegionSetup, exchange, outflow } from "../🧱️balance/🟦️.ts";

const ALPHA = 0.1;
const BLOOD_FLOW_NEUTRAL = 6.3;
const DILATION = 120;
const CONSTRICTION = 0.5;
const SWEAT_GAIN = 170;
const SWEAT_MAX = 500;
const SWEAT_LATENT = 0.68;
const SHIVER_GAIN = 19.4;
const TISSUE_CONDUCTANCE = 5.28;
const BLOOD_CAPACITY = 1.163;
const LOCAL_METABOLIC_SHARE = 0.05;
const TISSUE_HEAT_CAPACITY = 3490;
const VAPOUR_CAP_HEAT = 1860;
const PANT_GAIN = 3;
const PANT_MAX = 7;

type Segment = NonNullable<ThermalRegion["body"]>["segment"];

const SEGMENTS: Readonly<Record<Segment, { readonly tissue: number; readonly kappa: number; readonly sweat: number }>> = {
  head: { tissue: 1.3, kappa: 0, sweat: 1.3 },
  trunk: { tissue: 1, kappa: 1, sweat: 1 },
  arm: { tissue: 0.8, kappa: 1.5, sweat: 0.8 },
  hand: { tissue: 0.6, kappa: 3, sweat: 0.5 },
  leg: { tissue: 0.8, kappa: 1.5, sweat: 0.8 },
  foot: { tissue: 0.6, kappa: 3, sweat: 0.8 },
  tail: { tissue: 0.5, kappa: 3, sweat: 0.2 },
  wing: { tissue: 0.5, kappa: 3, sweat: 0.2 },
};

/** 🫀️ One body region's constant coupling to the body. */
export type BodyRegionSetup = { readonly setup: RegionSetup; readonly skinAreaM2: number; readonly f: number; readonly tissue: number; readonly kappa: number; readonly sweatShare: number; readonly localShare: number; readonly capacityJK: number };

/** 🫀️ The constant part of a body. */
export type BodySetup = { readonly body: BodyPhysiology; readonly respiration?: Respiration; readonly regions: readonly BodyRegionSetup[]; readonly coreCapacityJK: number; readonly baseMet: number };

/** 🫀️ The dynamic state of a body: core temperature and one skin temperature per body region (°C). */
export type BodyState = { readonly core: number; readonly skin: readonly number[] };

/** 🫀️ One region's solved state. */
export type BodyRegionDiag = {
  readonly surfaceC: number;
  readonly skinC: number;
  readonly dryWM2: number;
  readonly evapWM2: number;
  readonly wettedness: number;
  readonly bloodFlow: number;
  readonly coreToSkinWM2: number;
  readonly skinNetWM2: number;
  readonly localWM2: number;
  readonly conductedOuterWM2: number;
  readonly exposure: ReturnType<typeof exchange>;
  readonly evapOuterWM2: number;
  readonly conductanceWM2K: number;
};

/** 🫀️ The solved body at one state. */
export type BodyDiag = {
  readonly regions: readonly BodyRegionDiag[];
  readonly metabolicW: number;
  readonly shiverW: number;
  readonly coreNetW: number;
  readonly storageW: number;
  readonly meanSkinC: number;
  readonly meanBloodFlow: number;
  readonly meanWettedness: number;
  readonly respiratory: RespiratoryHeat;
  readonly ventilationLMin: number;
};

/** 🫁️ Respiratory heat exchange. */
export type RespiratoryHeat = { readonly sensibleW: number; readonly latentW: number; readonly moistureGH: number };

/** 🫁️ Respiratory heat of a ventilation `ventilationLMin` (inhaled volume at ambient conditions) exhaling air at `exhaledTempC` and `exhaledRelativeHumidity`.
 * The same ventilation numbers drive the breath emitters of 💨️transport. */
export function respiratoryHeat(env: Environment, exhaledTempC: number, exhaledRelativeHumidity: number, ventilationLMin: number): RespiratoryHeat {
  const pv = env.relativeHumidity * saturationPressurePa(env.airTempC);
  const win = humidityRatio(pv, env.pressurePa);
  const wex = humidityRatio(exhaledRelativeHumidity * saturationPressurePa(exhaledTempC), env.pressurePa);
  const mass = ((env.pressurePa - pv) / (R_DRY * (env.airTempC + 273.15))) * (ventilationLMin / 1000 / 60);
  const sensibleW = mass * (CP_AIR + win * VAPOUR_CAP_HEAT) * (exhaledTempC - env.airTempC);
  const latentW = mass * (wex - win) * latentHeatJkg(exhaledTempC);
  return { sensibleW, latentW, moistureGH: mass * (wex - win) * 3.6e6 };
}

/** 🫁️ The breathing pattern bound by the inputs, else the resting (first) pattern. */
export function patternOf(resp: Respiration | undefined, id: string | null): BreathingPattern | undefined {
  return resp?.patterns.find((p) => p.id === id) ?? resp?.patterns[0];
}

/** 🫀️ Builds the constant body setup of `subject` (`null` = the asset's own body, else a passenger id) from the `body` regions that name it. */
export function bodySetup(asset: Asset, setups: readonly RegionSetup[], subject: string | null = null): BodySetup | null {
  const passenger = subject === null ? undefined : asset.thermal.passengers.find((p) => p.id === subject);
  const body = subject === null ? asset.thermal.body : passenger?.physiology;
  const respiration = subject === null ? asset.thermal.respiration : passenger?.respiration;
  const regions = setups.filter((s) => s.region.model === "body" && s.region.body && (s.region.body.subject ?? null) === subject);
  if (!body || regions.length === 0) return null;
  const skinTotal = regions.reduce((s, r) => s + r.region.body!.skinFraction, 0);
  const shareTotal = regions.reduce((s, r) => s + r.region.body!.metabolicShare, 0);
  const sweatMean = regions.reduce((s, r) => s + (r.region.body!.skinFraction / skinTotal) * SEGMENTS[r.region.body!.segment].sweat, 0);
  const built = regions.map((setup): BodyRegionSetup => {
    const b = setup.region.body!;
    const skinAreaM2 = (b.skinFraction / skinTotal) * body.skinAreaM2;
    const seg = SEGMENTS[b.segment];
    return { setup, skinAreaM2, f: setup.region.areaM2 / skinAreaM2, tissue: seg.tissue, kappa: seg.kappa, sweatShare: seg.sweat / sweatMean, localShare: b.metabolicShare / shareTotal, capacityJK: setup.region.heatCapacityJM2K * setup.region.areaM2 };
  });
  const skinCapacity = built.reduce((s, r) => s + r.capacityJK, 0);
  return { body, ...(respiration ? { respiration } : {}), regions: built, coreCapacityJK: Math.max(0.5 * TISSUE_HEAT_CAPACITY * body.massKg, TISSUE_HEAT_CAPACITY * body.massKg - skinCapacity), baseMet: subject === null ? (asset.thermal.baseMet ?? 1) : 1 };
}

/** 🫀️ The neutral state: core and skin at the set points. */
export const neutralState = (b: BodySetup): BodyState => ({ core: b.body.coreSetPointC, skin: b.regions.map(() => b.body.skinSetPointC) });

const clamp = (x: number, lo: number, hi: number): number => Math.min(hi, Math.max(lo, x));

type Controls = { readonly warmCore: number; readonly coldCore: number; readonly warmSkin: number; readonly coldSkin: number; readonly warmBody: number; readonly meanSkin: number; readonly sweat: number };

function controls(b: BodySetup, st: BodyState): Controls {
  const area = b.regions.reduce((s, r) => s + r.skinAreaM2, 0);
  const meanSkin = b.regions.reduce((s, r, i) => s + r.skinAreaM2 * st.skin[i]!, 0) / area;
  const sk = meanSkin - b.body.skinSetPointC;
  const cr = st.core - b.body.coreSetPointC;
  const bodyT = ALPHA * meanSkin + (1 - ALPHA) * st.core;
  const bodyNeutral = ALPHA * b.body.skinSetPointC + (1 - ALPHA) * b.body.coreSetPointC;
  const warmSkin = Math.max(0, sk);
  const warmBody = Math.max(0, bodyT - bodyNeutral);
  return { warmCore: Math.max(0, cr), coldCore: Math.max(0, -cr), warmSkin, coldSkin: Math.max(0, -sk), warmBody, meanSkin, sweat: b.body.sweating ? Math.min(SWEAT_MAX, SWEAT_GAIN * warmBody * Math.exp(warmSkin / 10.7)) : 0 };
}

function skinNode(b: BodySetup, i: number, tsk: number, st: BodyState, c: Controls, frame: RegionFrame, a: Ambient, dryFrame: RegionFrame): BodyRegionDiag {
  const r = b.regions[i]!;
  const R = r.setup.layerResistance;
  const e = frame.exposed;
  const ts = R < 1e-9 ? tsk : solveDecreasing((t) => (tsk - t) / R - outflow(dryFrame, exchange(dryFrame, a, t)), tsk);
  const x = exchange(dryFrame, a, ts);
  const dry = r.f * outflow(dryFrame, x);
  const he = evaporativeCoefficient(x.hc, a.env.pressurePa);
  const eMax = (r.f * e * Math.max(0, saturationPressurePa(tsk) - a.pv)) / (r.setup.layerVapourResistance + 1 / he);
  const v = Math.max(frame.speed, 0.1);
  const wMax = r.setup.region.layers.length > 0 ? 0.59 * v ** -0.08 : 0.38 * v ** -0.29;
  const sweat = SWEAT_LATENT * c.sweat * r.sweatShare;
  const w = Math.max(frame.wettedness, Math.min(wMax, 0.06 + (eMax > 0 ? (0.94 * sweat) / eMax : 0)));
  const evap = w * eMax;
  const bloodRaw = (BLOOD_FLOW_NEUTRAL + DILATION * c.warmCore) / (1 + CONSTRICTION * r.kappa * c.coldSkin);
  const blood = clamp(bloodRaw, b.body.minSkinBloodFlowLM2H, b.body.maxSkinBloodFlowLM2H);
  const toSkin = (TISSUE_CONDUCTANCE * r.tissue + BLOOD_CAPACITY * blood) * (st.core - tsk);
  return { surfaceC: ts, skinC: tsk, dryWM2: dry, evapWM2: evap, wettedness: w, bloodFlow: blood, coreToSkinWM2: toSkin, skinNetWM2: toSkin - dry - evap, localWM2: 0, conductedOuterWM2: dry / r.f, exposure: x, evapOuterWM2: e > 0 ? evap / (r.f * e) : 0, conductanceWM2K: TISSUE_CONDUCTANCE * r.tissue + BLOOD_CAPACITY * blood };
}

/** 🫀️ Solves every region's surface and the body's balances at a state; `frames` align with `b.regions` (their `wettedness` is the binding override). */
export function evaluateBody(b: BodySetup, st: BodyState, inputs: ThermalInputs, frames: readonly RegionFrame[], a: Ambient): BodyDiag {
  const c = controls(b, st);
  const area = b.body.skinAreaM2;
  const met = inputs.activityMet > 0 ? inputs.activityMet : b.baseMet;
  const metabolic = Math.max(b.body.basalMetabolismW, met * b.body.metWM2 * area);
  const shiver = SHIVER_GAIN * c.coldSkin * c.coldCore * area;
  const heat = Math.max(0, metabolic + shiver - Math.min(inputs.externalWorkW, metabolic));
  const resp = b.respiration;
  const pattern = patternOf(resp, inputs.breathing);
  const pant = b.body.panting ? Math.min(PANT_MAX * b.body.massKg, PANT_GAIN * b.body.massKg * c.warmCore) : 0;
  const ventilation = (pattern ? pattern.rateBpm * pattern.tidalVolumeL : 0) + pant;
  const respiratory = resp
    ? respiratoryHeat(a.env, resp.exhaledTempC, resp.exhaledRelativeHumidity, ventilation)
    : fallbackRespiration(a.env, metabolic, area);
  const regions = b.regions.map((_, i) => {
    const f = frames[i]!;
    return skinNode(b, i, st.skin[i]!, st, c, f, a, { ...f, wettedness: 0 });
  });
  const local = b.regions.map((r) => (LOCAL_METABOLIC_SHARE * heat * r.localShare) / r.skinAreaM2);
  const loss = regions.reduce((s, d, i) => s + (d.dryWM2 + d.evapWM2) * b.regions[i]!.skinAreaM2, 0);
  const toSkinW = regions.reduce((s, d, i) => s + d.coreToSkinWM2 * b.regions[i]!.skinAreaM2, 0);
  const withLocal = regions.map((d, i) => ({ ...d, localWM2: local[i]!, skinNetWM2: d.skinNetWM2 + local[i]! }));
  const respW = respiratory.sensibleW + respiratory.latentW;
  const skinWeights = b.regions.map((r) => r.skinAreaM2 / area);
  return {
    regions: withLocal,
    metabolicW: metabolic + shiver,
    shiverW: shiver,
    coreNetW: heat * (1 - LOCAL_METABOLIC_SHARE) - respW - toSkinW,
    storageW: heat - respW - loss,
    meanSkinC: c.meanSkin,
    meanBloodFlow: regions.reduce((s, d, i) => s + skinWeights[i]! * d.bloodFlow, 0),
    meanWettedness: regions.reduce((s, d, i) => s + skinWeights[i]! * d.wettedness, 0),
    respiratory,
    ventilationLMin: ventilation,
  };
}

function fallbackRespiration(env: Environment, metabolicW: number, areaM2: number): RespiratoryHeat {
  const pa = (env.relativeHumidity * saturationPressurePa(env.airTempC)) / 1000;
  const m = metabolicW / areaM2;
  const sensibleW = 0.0014 * m * (34 - env.airTempC) * areaM2;
  const latentW = 0.0173 * m * (5.87 - pa) * areaM2;
  return { sensibleW, latentW, moistureGH: (latentW / latentHeatJkg(34)) * 3.6e6 };
}

/** 🫀️ Advances the body by `dtS` seconds: skin nodes relax exponentially towards their linearised quasi-steady value (unconditionally stable, monotone), the core
 * takes an explicit step with its thermal mass (`dtS` ≤ 60 s keeps it well inside the stability limit). Returns the new state and the diagnostics at the old one. */
export function stepBody(b: BodySetup, st: BodyState, inputs: ThermalInputs, frames: readonly RegionFrame[], a: Ambient, dtS: number): { readonly state: BodyState; readonly diag: BodyDiag } {
  const diag = evaluateBody(b, st, inputs, frames, a);
  const core = st.core + (dtS * diag.coreNetW) / b.coreCapacityJK;
  const d = 0.05;
  const c = controls(b, st);
  const skin = b.regions.map((r, i) => {
    const f = frames[i]!;
    const probe = skinNode(b, i, st.skin[i]! + d, st, c, f, a, { ...f, wettedness: 0 });
    const g = Math.max(1e-3, (diag.regions[i]!.skinNetWM2 - diag.regions[i]!.localWM2 - probe.skinNetWM2) / d);
    const target = st.skin[i]! + diag.regions[i]!.skinNetWM2 / g;
    const tau = (r.capacityJK / r.skinAreaM2) / g;
    return target + (st.skin[i]! - target) * Math.exp(-dtS / tau);
  });
  return { state: { core, skin }, diag };
}

/** 🫀️ The restoring conductance of every body region's skin node, W/(m² skin K), at a state with diagnostics `diag`. */
export function skinConductances(b: BodySetup, st: BodyState, frames: readonly RegionFrame[], a: Ambient, diag: BodyDiag): number[] {
  const c = controls(b, st);
  return b.regions.map((_, i) => {
    const f = frames[i]!;
    const probe = skinNode(b, i, st.skin[i]! + 0.05, st, c, f, a, { ...f, wettedness: 0 });
    return Math.max(1e-3, (diag.regions[i]!.skinNetWM2 - diag.regions[i]!.localWM2 - probe.skinNetWM2) / 0.05);
  });
}

/** 🫀️ The steady state of a body under constant inputs: skin nodes relax to their quasi-steady values while the core is advanced by a secant-Newton step on its
 * heat balance, until the residual power is below 1 mW. */
export function steadyBody(b: BodySetup, inputs: ThermalInputs, frames: readonly RegionFrame[], a: Ambient, start: BodyState = neutralState(b)): { readonly state: BodyState; readonly diag: BodyDiag } {
  let st = start;
  let slope = b.regions.reduce((s, r) => s + 15 * r.skinAreaM2, 0) + 30;
  let previous: { core: number; g: number } | null = null;
  for (let it = 0; it < 300; it++) {
    const { state: relaxed, diag } = stepBody(b, st, inputs, frames, a, 1e6);
    const skinChange = Math.max(...relaxed.skin.map((t, i) => Math.abs(t - st.skin[i]!)));
    const g = diag.coreNetW;
    if (Math.abs(g) < 1e-3 && skinChange < 1e-7) return { state: st, diag };
    if (previous && Math.abs(st.core - previous.core) > 1e-9) {
      const s = -(g - previous.g) / (st.core - previous.core);
      if (Number.isFinite(s) && s > 1) slope = s;
    }
    previous = { core: st.core, g };
    st = { core: st.core + clamp(g / slope, -1, 1), skin: relaxed.skin };
  }
  return { state: st, diag: evaluateBody(b, st, inputs, frames, a) };
}
