/** 🌡️ Thermal solver of the staffage archive: steady-state surface energy balances per region, deterministic jet/plume streamlines and tracer parcels per emitter.
 *
 * Models (all documented in the ticket file `thermal-model.md`; none is CFD and none is physiologically validated):
 * - radiation: grey-body exchange with the mean radiant temperature, q = εσ(Tₛ⁴ − T_mrt⁴);
 * - convection: ISO 7730 (Fanger) for clothed bodies, flat-plate / Churchill–Bernstein cylinder forced and laminar/turbulent vertical-plate natural correlations elsewhere, h = max(natural, forced);
 * - evaporation: ASHRAE skin wettedness with the Lewis relation hₑ = 16.5·h_c, Penman–Monteith leaf transpiration with a joint leaf energy balance;
 * - respiration: ASHRAE Fundamentals ch. 9 latent and sensible loss, assigned to breath emitters;
 * - emitters: free round turbulent jet (centreline u ∝ K·d/x, ΔT ∝ 5·d/x) advected by the relative wind and lifted by a momentum-conserving buoyant acceleration.
 *
 * Wind convention: `windDirectionDeg` is the direction the air moves towards, counter-clockwise from +x in the elevation plane; `relativeHumidity` is a fraction 0…1.
 *
 * @see ../../🧬️schema/🟦️.ts — ThermalModel, ThermalRegion, Emitter, Environment
 * @see ../🔗️contracts/🟦️.ts — EvaluatedState, ThermalField
 */
import type { Emitter, Environment, Slug, Text, ThermalRegion } from "../../🧬️schema/🟦️.ts";
import type { DisplayRange, EvaluatedState, Legend, Parcel, Point, RegionField, Streamline, ThermalField, ThermalQuantity } from "../🔗️contracts/🟦️.ts";

//#region 🔖️Constants
const SIGMA = 5.670374419e-8;
const KELVIN = 273.15;
const GRAVITY = 9.80665;
const R_DRY = 287.058;
const R_VAPOUR = 461.495;
const CP_AIR = 1006;
const PRANDTL = 0.71;
const SCHMIDT = 0.6;
const LEWIS_BODY = 16.5;
const STANDARD_PRESSURE = 101325;
const MET_WM2 = 58.15;
const CORE_RISE_K_PER_MET = 0.15;
const CORE_RISE_MAX_K = 1.5;
const SKIN_MIN_WETTEDNESS = 0.06;
const DEFAULT_INSULATION = 0.155;
const DEFAULT_STOMATAL = 0.005;
const JET_VELOCITY_K = 6.2;
const JET_SCALAR_K = 5;
const JET_SPREAD = 0.1;
const EXCESS_STOP_K = 0.2;
const SPEED_STOP_MS = 0.05;
const VAPOUR_EXCESS_STOP = 0.02;
const PUFF_FORMATION_NUMBER = 4;
const MAX_REACH_M = 10;
const MAX_TIME_S = 60;
const STREAMLINE_POINTS = 24;
const PARCELS_PER_S = 8;
const MAX_PARCELS_PER_EMITTER = 240;
const DEG = Math.PI / 180;

type ConvectionClass = "body" | "cylinder" | "plate";

const CONVECTION_CLASS: Readonly<Record<ThermalRegion["kind"], ConvectionClass>> = { skin: "body", clothing: "body", hair: "body", fur: "cylinder", feather: "cylinder", bark: "cylinder", tyre: "cylinder", exhaust: "cylinder", foliage: "plate", surface: "plate", engine: "plate", glazing: "plate", wall: "plate" };
const SOLAR_AREA_FACTOR: Readonly<Record<ThermalRegion["kind"], number>> = { skin: 0.25, clothing: 0.25, hair: 0.25, fur: 0.25, feather: 0.25, bark: 0.25, tyre: 0.25, exhaust: 0.25, foliage: 0.5, engine: 0.5, surface: 1, wall: 1, glazing: 1 };
//#endregion 🔖️Constants

//#region 💧️Psychrometrics
/** 💧️ Saturation vapour pressure over water in pascal, Magnus form with the Buck (1996) coefficients; within 0.05 % of IAPWS-95 for 0 … 60 °C.
 * @see Buck, A. L. (1996) Buck Research CR-1A manual */
export const saturationVapourPressurePa = (tempC: number): number => 611.21 * Math.exp((18.678 - tempC / 234.5) * (tempC / (257.14 + tempC)));

/** 🌫️ Partial vapour pressure of the ambient air in pascal (relative humidity is a fraction). */
export const vapourPressurePa = (environment: Environment): number => environment.relativeHumidity * saturationVapourPressurePa(environment.airTempC);

/** 🫧️ Specific humidity (kg vapour per kg moist air) from vapour pressure and total pressure. */
export const specificHumidity = (vapourPa: number, pressurePa: number): number => (0.622 * vapourPa) / (pressurePa - 0.378 * vapourPa);

/** 🔥️ Latent heat of vaporisation of water in J/kg (linear fit, 0 … 50 °C). */
export const latentHeatJPerKg = (tempC: number): number => 2.501e6 - 2361 * tempC;

/** ☁️ Moist-air density in kg/m³. */
export const airDensity = (tempC: number, pressurePa: number, vapourPa: number): number => (pressurePa - vapourPa) / (R_DRY * (tempC + KELVIN)) + vapourPa / (R_VAPOUR * (tempC + KELVIN));
//#endregion 💧️Psychrometrics

//#region 🌬️Transfer
type AirProperties = { readonly conductivity: number; readonly kinematicViscosity: number };

const airProperties = (filmC: number, pressurePa: number): AirProperties => {
  const t = filmC + KELVIN;
  const mu = 1.716e-5 * (t / KELVIN) ** 1.5 * (KELVIN + 110.4) / (t + 110.4);
  return { conductivity: 0.02428 + 7.7e-5 * filmC, kinematicViscosity: mu / airDensity(filmC, pressurePa, 0) };
};

/** 🌬️ Convective coefficient in W/(m²·K), h = max(natural, forced).
 * - body: ISO 7730 / Fanger, natural 2.38·ΔT^0.25, forced 12.1·√v;
 * - plate: natural 1.42·(ΔT/L)^¼ (Ra < 10⁹) or 1.31·ΔT^⅓, forced Nu = 0.664·Re^½·Pr^⅓ (Re < 5·10⁵) or (0.037·Re^0.8 − 871)·Pr^⅓;
 * - cylinder: same natural, forced Churchill–Bernstein cross-flow.
 * @see ISO 7730:2005 annex D; Incropera & DeWitt ch. 7, 9; ASHRAE Handbook — Fundamentals ch. 4 */
export function convectionCoefficient(kind: ConvectionClass, deltaK: number, speedMS: number, lengthM: number, filmC: number, pressurePa: number): number {
  const dT = Math.abs(deltaK);
  if (kind === "body") return Math.max(2.38 * dT ** 0.25, 12.1 * Math.sqrt(Math.max(0, speedMS)));
  const { conductivity, kinematicViscosity } = airProperties(filmC, pressurePa);
  const rayleigh = (GRAVITY * dT * lengthM ** 3 * PRANDTL) / ((filmC + KELVIN) * kinematicViscosity ** 2);
  const natural = dT === 0 ? 0 : rayleigh < 1e9 ? 1.42 * (dT / lengthM) ** 0.25 : 1.31 * dT ** (1 / 3);
  const re = (Math.max(0, speedMS) * lengthM) / kinematicViscosity;
  const nu = re === 0 ? 0 : kind === "plate"
    ? (re < 5e5 ? 0.664 * Math.sqrt(re) : 0.037 * re ** 0.8 - 871) * PRANDTL ** (1 / 3)
    : 0.3 + (0.62 * Math.sqrt(re) * PRANDTL ** (1 / 3) * (1 + (re / 282000) ** (5 / 8)) ** (4 / 5)) / (1 + (0.4 / PRANDTL) ** (2 / 3)) ** 0.25;
  return Math.max(natural, (nu * conductivity) / lengthM);
}

/** ☢️ Net grey-body radiant flux in W/m² leaving a surface, q = εσ(Tₛ⁴ − T_mrt⁴); positive = loss. */
export const radiationFlux = (emissivity: number, surfaceC: number, mrtC: number): number => emissivity * SIGMA * ((surfaceC + KELVIN) ** 4 - (mrtC + KELVIN) ** 4);

/** 📉️ Secant radiation coefficient in W/(m²·K) with q = h_r·(Tₛ − T_mrt); the 4εσT³ tangent where both temperatures coincide. */
export const radiationCoefficient = (emissivity: number, surfaceC: number, mrtC: number): number =>
  Math.abs(surfaceC - mrtC) < 1e-9 ? 4 * emissivity * SIGMA * (surfaceC + KELVIN) ** 3 : radiationFlux(emissivity, surfaceC, mrtC) / (surfaceC - mrtC);
//#endregion 🌬️Transfer

//#region 🧮️Balance
type Ambient = { readonly env: Environment; readonly vapourPa: number; readonly humidity: number; readonly density: number; readonly wind: Point; readonly relativeSpeed: number };

const ambientOf = (env: Environment, state: EvaluatedState): Ambient => {
  const vapourPa = vapourPressurePa(env);
  const phi = env.windDirectionDeg * DEG;
  const velocity = state.kinematics?.comVelocity ?? { x: 0, y: 0 };
  const wind = { x: env.windMS * Math.cos(phi) - velocity.x, y: env.windMS * Math.sin(phi) - velocity.y };
  return { env, vapourPa, humidity: specificHumidity(vapourPa, env.pressurePa), density: airDensity(env.airTempC, env.pressurePa, vapourPa), wind, relativeSpeed: Math.hypot(wind.x, wind.y) };
};

type Exchange = { readonly qr: number; readonly qc: number; readonly qe: number; readonly hc: number; readonly hr: number };

type RegionContext = { readonly region: ThermalRegion; readonly emissivity: number; readonly absorbedSolarWM2: number; readonly wettedness: number; readonly ambient: Ambient };

function exchange(c: RegionContext, tsC: number): Exchange {
  const { region, ambient: a } = c;
  const env = a.env;
  const hc = convectionCoefficient(CONVECTION_CLASS[region.kind], tsC - env.airTempC, a.relativeSpeed, region.characteristicLengthM, (tsC + env.airTempC) / 2, env.pressurePa);
  const qr = radiationFlux(c.emissivity, tsC, env.meanRadiantTempC);
  const qc = hc * (tsC - env.airTempC);
  const surfaceVapourPa = saturationVapourPressurePa(tsC);
  let qe: number;
  if (region.model === "leaf") {
    const gb = (hc / (a.density * CP_AIR)) * (PRANDTL / SCHMIDT) ** (1 / 3);
    const gs = Math.max(0, region.stomatalConductanceMS ?? DEFAULT_STOMATAL);
    const g = gs + gb === 0 ? 0 : (gs * gb) / (gs + gb);
    qe = Math.max(0, latentHeatJPerKg(tsC) * a.density * g * (specificHumidity(surfaceVapourPa, env.pressurePa) - a.humidity));
  } else {
    qe = Math.max(0, c.wettedness * LEWIS_BODY * hc * (STANDARD_PRESSURE / env.pressurePa) * ((surfaceVapourPa - a.vapourPa) / 1000));
  }
  return { qr, qc, qe, hc, hr: radiationCoefficient(c.emissivity, tsC, env.meanRadiantTempC) };
}

function solveDecreasing(f: (x: number) => number, lo: number, hi: number): number {
  let span = Math.max(1, hi - lo);
  for (let i = 0; i < 60 && f(hi) > 0; i++, span *= 2) hi += span;
  span = Math.max(1, hi - lo);
  for (let i = 0; i < 60 && f(lo) < 0; i++, span *= 2) lo -= span;
  let [a, b, fa, fb, side] = [lo, hi, f(lo), f(hi), 0];
  for (let i = 0; i < 400; i++) {
    const c = fb === fa ? (a + b) / 2 : (a * fb - b * fa) / (fb - fa);
    const fc = f(c);
    if (Math.abs(fc) < 1e-11 || Math.abs(b - a) < 1e-12) return c;
    if (fc > 0) {
      [a, fa] = [c, fc];
      if (side === 1) fb /= 2;
      side = 1;
    } else {
      [b, fb] = [c, fc];
      if (side === -1) fa /= 2;
      side = -1;
    }
  }
  return (a + b) / 2;
}

function surfaceTemperature(c: RegionContext, override: { surfaceTempC?: number; coreTempC?: number }, coreRiseK: number, state: EvaluatedState): number {
  const { region, ambient: a } = c;
  const env = a.env;
  if (override.surfaceTempC !== undefined) return override.surfaceTempC;
  const bounds = [env.airTempC, env.meanRadiantTempC];
  switch (region.model) {
    case "fixed":
      return region.surfaceTempC ?? env.airTempC;
    case "operating":
      return env.airTempC + (region.riseK ?? 0) * (state.thermal.operating[region.operatingKey ?? ""] ?? 0);
    case "passive":
    case "leaf": {
      const f = (t: number) => { const e = exchange(c, t); return c.absorbedSolarWM2 - e.qr - e.qc - e.qe; };
      return solveDecreasing(f, Math.min(...bounds) - 1, Math.max(...bounds) + 1);
    }
    case "core-resistance": {
      const core = (override.coreTempC ?? region.coreTempC ?? 37) + (override.coreTempC === undefined ? coreRiseK : 0);
      const r = Math.max(1e-6, region.insulationM2KW ?? DEFAULT_INSULATION);
      const f = (t: number) => { const e = exchange(c, t); return (core - t) / r - e.qr - e.qc - e.qe + c.absorbedSolarWM2; };
      return solveDecreasing(f, Math.min(...bounds, core) - 1, Math.max(...bounds, core) + 1);
    }
  }
}

/** 🧍️ Body surface area in m² after DuBois & DuBois (1916), A = 0.202·m^0.425·h^0.725. */
export const duboisAreaM2 = (massKg: number, heightM: number): number => (massKg > 0 && heightM > 0 ? 0.202 * massKg ** 0.425 * heightM ** 0.725 : 0);

/** 🫁️ Respiratory heat loss per body area in W/m² after ASHRAE Fundamentals ch. 9: latent 0.0173·M·(5.87 − pₐ[kPa]), sensible 0.0014·M·(34 − Tₐ), M in W/m². */
export const respirationWM2 = (metabolicWM2: number, airC: number, vapourPa: number): { readonly latent: number; readonly sensible: number } => ({
  latent: Math.max(0, 0.0173 * metabolicWM2 * (5.87 - vapourPa / 1000)),
  sensible: 0.0014 * metabolicWM2 * (34 - airC),
});

function solveRegions(state: EvaluatedState, a: Ambient): { readonly fields: Record<Slug, RegionField>; readonly defaults: Slug[] } {
  const model = state.asset.thermal;
  const materials = new Map(state.asset.materials.map((m) => [m.id, m]));
  const baseMet = model.baseMet ?? 1;
  const shares = model.regions.filter((r) => r.model === "core-resistance" && r.metabolicShare !== undefined).map((r) => r.metabolicShare as number);
  const meanShare = shares.length ? shares.reduce((s, v) => s + v, 0) / shares.length : 1;
  const fields: Record<Slug, RegionField> = {};
  const defaults: Slug[] = [];
  for (const region of model.regions) {
    const override = state.thermal.regions[region.id] ?? { exposedFraction: 1 };
    const material = materials.get(region.material);
    if (!material || (region.model === "core-resistance" && region.insulationM2KW === undefined)) defaults.push(region.id);
    const share = region.metabolicShare === undefined ? 1 : Math.min(2, region.metabolicShare / meanShare);
    const coreRiseK = Math.min(CORE_RISE_MAX_K, CORE_RISE_K_PER_MET * Math.max(0, state.thermal.activityMet - baseMet) * share);
    const c: RegionContext = {
      region,
      emissivity: material?.emissivity ?? 0.9,
      absorbedSolarWM2: (material?.solarAbsorptivity ?? 0) * a.env.solarWM2 * SOLAR_AREA_FACTOR[region.kind],
      wettedness: override.wettedness ?? region.wettedness ?? (region.kind === "skin" ? SKIN_MIN_WETTEDNESS : 0),
      ambient: a,
    };
    const tsC = surfaceTemperature(c, override, coreRiseK, state);
    const e = exchange(c, tsC);
    const exposed = override.exposedFraction ?? 1;
    const area = region.areaM2 * exposed;
    fields[region.id] = {
      region: region.id,
      surfaceTempC: tsC,
      radiantFluxWM2: e.qr,
      convectiveFluxWM2: e.qc,
      latentFluxWM2: e.qe,
      netFluxWM2: e.qr + e.qc + e.qe - c.absorbedSolarWM2,
      radiantW: e.qr * area,
      convectiveW: e.qc * area,
      latentW: e.qe * area,
      convectionCoefficientWM2K: e.hc,
      radiationCoefficientWM2K: e.hr,
      exposedFraction: exposed,
    };
  }
  return { fields, defaults };
}
//#endregion 🧮️Balance

//#region 💨️Emitters
type Source = { readonly at: Point; readonly dirRad: number; readonly u0: number; readonly d: number; readonly dT0: number; readonly vapour: boolean; readonly q0: number; readonly qa: number; readonly crosswindMS: number; readonly puff: boolean };

type Path = { readonly t: number[]; readonly x: number[]; readonly y: number[]; readonly ell: number[] };

const dilution = (s: Source, ell: number, t: number): number => {
  const x = ell + s.crosswindMS * t;
  return s.puff ? Math.max(1, (x / (PUFF_FORMATION_NUMBER * s.d)) ** 3) : Math.max(1, x / (JET_SCALAR_K * s.d));
};

const relativeVelocity = (s: Source, t: number, ell: number, airK: number): Point => {
  const uj = s.puff ? s.u0 * Math.min(1, ((PUFF_FORMATION_NUMBER * s.d) / Math.max(ell, 1e-12)) ** 3) : s.u0 * Math.min(1, (JET_VELOCITY_K * s.d) / Math.max(ell, 1e-12));
  const vb = ((GRAVITY * s.dT0) / airK) * (t / dilution(s, ell, t));
  return { x: uj * Math.cos(s.dirRad), y: uj * Math.sin(s.dirRad) + vb };
};

/** 🌀️ Integrates one Lagrangian parcel path in the air frame plus wind drift, Heun's method with an adaptive step of 2 % of the jet length scale.
 * Dilution D = max(1, x/5d) with x = air-frame path + |relative wind|·t (cross-flow eddies mix like path length, Briggs bent-over plume); excess temperature and vapour excess both decay as 1/D.
 * Pulsed sources (period with duty < 1) are puffs: beyond 4d (vortex-ring formation number) D = (x/4d)³ and u = u₀·(4d/x)³ (Turner 1973); steady sources follow the round jet u = u₀·min(1, 6.2d/x).
 * The buoyant velocity v_b = g·ΔT₀/Tₐ·t/D conserves buoyancy impulse while entrainment dilutes the parcel (Morton, Taylor & Turner 1956).
 * The path ends when the temperature excess < 0.2 K and the vapour excess < 2 % of its source value, or the relative speed < 0.05 m/s, or after 10 m.
 * @see Pope, Turbulent Flows ch. 5 (round jet); Briggs 1969 (plume rise); Turner 1973, Buoyancy Effects in Fluids (puffs); Morton, Taylor & Turner 1956 */
function integratePlume(s: Source, wind: Point, airK: number): Path {
  const path: Path = { t: [0], x: [s.at.x], y: [s.at.y], ell: [0] };
  let [t, x, y, ell] = [0, s.at.x, s.at.y, 0];
  const stop = (tt: number, l: number) => {
    const rel = relativeVelocity(s, tt, l, airK);
    const speed = Math.hypot(rel.x, rel.y);
    const dil = dilution(s, l, tt);
    const needed = Math.max(Math.abs(s.dT0) >= EXCESS_STOP_K ? Math.abs(s.dT0) / EXCESS_STOP_K : 1, Math.abs(s.q0 - s.qa) > 0 ? 1 / VAPOUR_EXCESS_STOP : 1);
    return speed < (s.vapour ? SPEED_STOP_MS / 10 : SPEED_STOP_MS) || dil > needed;
  };
  for (let i = 0; i < 20000 && t < MAX_TIME_S && Math.hypot(x - s.at.x, y - s.at.y) < MAX_REACH_M && (i === 0 || !stop(t, ell)); i++) {
    const k1 = relativeVelocity(s, t, ell, airK);
    const l1 = Math.hypot(k1.x, k1.y);
    const dt = Math.min(0.05, Math.max(1e-4, (0.02 * (ell + 3 * s.d)) / (l1 + 1e-9)));
    const k2 = relativeVelocity(s, t + dt, ell + l1 * dt, airK);
    const l2 = Math.hypot(k2.x, k2.y);
    x += (wind.x + (k1.x + k2.x) / 2) * dt;
    y += (wind.y + (k1.y + k2.y) / 2) * dt;
    ell += ((l1 + l2) / 2) * dt;
    t += dt;
    path.t.push(t);
    path.x.push(x);
    path.y.push(y);
    path.ell.push(ell);
  }
  return path;
}

type Sample = { readonly at: Point; readonly ageS: number; readonly ell: number; readonly velocity: Point; readonly excessK: number };

function sampleAt(s: Source, path: Path, wind: Point, airK: number, index: number, frac: number): Sample {
  const j = Math.min(index + 1, path.t.length - 1);
  const mix = (v: readonly number[]) => (v[index] as number) * (1 - frac) + (v[j] as number) * frac;
  const [t, ell] = [mix(path.t), mix(path.ell)];
  const rel = relativeVelocity(s, t, ell, airK);
  return { at: { x: mix(path.x), y: mix(path.y) }, ageS: t, ell, velocity: { x: wind.x + rel.x, y: wind.y + rel.y }, excessK: s.dT0 / dilution(s, ell, t) };
}

const locate = (times: readonly number[], age: number): { index: number; frac: number } => {
  let [lo, hi] = [0, times.length - 1];
  while (hi - lo > 1) {
    const mid = (lo + hi) >> 1;
    if ((times[mid] as number) <= age) lo = mid; else hi = mid;
  }
  const t0 = times[lo] as number;
  const t1 = times[hi] as number;
  return { index: lo, frac: t1 > t0 ? Math.min(1, Math.max(0, (age - t0) / (t1 - t0))) : 0 };
};

/** #️⃣️ Deterministic hash of (id, index) mapped to [−1, 1] (FNV-1a followed by a murmur3 finaliser); the only "randomness" of the parcel spread. */
export function unitHash(id: string, index: number): number {
  let h = 2166136261;
  for (let i = 0; i < id.length; i++) h = Math.imul(h ^ id.charCodeAt(i), 16777619);
  h ^= index;
  h = Math.imul(h ^ (h >>> 16), 2246822507);
  h = Math.imul(h ^ (h >>> 13), 3266489909);
  h ^= h >>> 16;
  return ((h >>> 0) / 4294967295) * 2 - 1;
}

type EmitterResult = { readonly streamline: Streamline | null; readonly parcels: readonly Parcel[]; readonly sensibleW: number; readonly latentW: number };

function emitterField(state: EvaluatedState, emitter: Emitter, a: Ambient): EmitterResult {
  const empty: EmitterResult = { streamline: null, parcels: [], sensibleW: 0, latentW: 0 };
  const strength = Math.min(1, Math.max(0, state.thermal.emitters[emitter.id] ?? 0));
  const anchor = state.anchors[emitter.anchor];
  if (strength <= 0 || !anchor) return empty;
  const bone = state.bones[anchor.anchor.bone];
  const airK = a.env.airTempC + KELVIN;
  const period = emitter.periodS && emitter.periodS > 0 ? emitter.periodS : 0;
  const [from, to] = [emitter.dutyFrom ?? 0, emitter.dutyTo ?? 1];
  const phase = period ? (((state.timeS % period) + period) % period) / period : 0;
  const open = !period || (phase >= from && phase < to) ? 1 : 0;
  const tempC = emitter.kind === "transpiration" ? Math.min(emitter.temperatureC, a.env.airTempC) : emitter.temperatureC;
  const source: Source = {
    at: anchor.at,
    dirRad: (bone ? bone.angleRad : 0) + emitter.directionDeg * DEG,
    u0: emitter.velocityMS * strength,
    d: emitter.diameterM,
    dT0: tempC - a.env.airTempC,
    vapour: emitter.carries === "vapour",
    q0: emitter.vapourKgPerKg ?? (emitter.carries === "vapour" ? specificHumidity(saturationVapourPressurePa(tempC), a.env.pressurePa) : a.humidity),
    qa: a.humidity,
    crosswindMS: a.relativeSpeed,
    puff: period > 0 && to - from < 1,
  };
  const path = integratePlume(source, a.wind, airK);
  const cumulative: number[] = [0];
  for (let i = 1; i < path.x.length; i++) cumulative.push((cumulative[i - 1] as number) + Math.hypot((path.x[i] as number) - (path.x[i - 1] as number), (path.y[i] as number) - (path.y[i - 1] as number)));
  const total = cumulative[cumulative.length - 1] as number;
  const points: Point[] = [];
  const excessK: number[] = [];
  const speedMS: number[] = [];
  for (let k = 0; k < STREAMLINE_POINTS; k++) {
    const { index, frac } = locate(cumulative, (total * k) / (STREAMLINE_POINTS - 1));
    const sample = sampleAt(source, path, a.wind, airK, index, frac);
    points.push(sample.at);
    excessK.push(sample.excessK);
    speedMS.push(Math.hypot(sample.velocity.x, sample.velocity.y));
  }
  const streamline: Streamline = { emitter: emitter.id, points, excessK, speedMS, carries: emitter.carries, active: strength * open };
  const slots = period ? Math.max(PARCELS_PER_S, Math.ceil(PARCELS_PER_S * period)) : 0;
  const step = period ? period / slots : 1 / PARCELS_PER_S;
  const endS = path.t[path.t.length - 1] as number;
  const newest = Math.floor(state.timeS / step + 1e-9);
  const oldest = Math.max(newest - MAX_PARCELS_PER_EMITTER + 1, Math.ceil((state.timeS - endS) / step - 1e-9));
  const parcels: Parcel[] = [];
  for (let k = newest; k >= oldest; k--) {
    const slot = period ? ((k % slots) + slots) % slots : ((k % 64) + 64) % 64;
    if (period && !(slot / slots >= from - 1e-12 && slot / slots < to - 1e-12)) continue;
    const age = Math.min(endS, Math.max(0, state.timeS - k * step));
    const { index, frac } = locate(path.t, age);
    const s = sampleAt(source, path, a.wind, airK, index, frac);
    const speed = Math.hypot(s.velocity.x, s.velocity.y);
    const normal = speed > 1e-9 ? { x: -s.velocity.y / speed, y: s.velocity.x / speed } : { x: -Math.sin(source.dirRad), y: Math.cos(source.dirRad) };
    const lateral = 0.7 * unitHash(emitter.id, slot) * Math.max(source.d / 2, JET_SPREAD * s.ell);
    parcels.push({
      emitter: emitter.id,
      at: { x: s.at.x + normal.x * lateral, y: s.at.y + normal.y * lateral },
      ageS: age,
      velocity: s.velocity,
      excessK: s.excessK,
      vapourKgPerKg: a.humidity + (source.q0 - a.humidity) / dilution(source, s.ell, s.ageS),
      carries: emitter.carries,
    });
  }
  const massFlow = a.density * source.u0 * (Math.PI * source.d ** 2 / 4) * open;
  const sensibleW = emitter.kind === "breath" ? 0 : massFlow * CP_AIR * source.dT0;
  const latentW = emitter.kind === "breath" || emitter.kind === "transpiration" ? 0 : massFlow * (source.q0 - a.humidity) * latentHeatJPerKg(tempC);
  return { streamline, parcels, sensibleW, latentW };
}
//#endregion 💨️Emitters

//#region 📝️Notes
const note = (en: string, de: string): Text => ({ en, de });

const classify = (state: EvaluatedState): Text => {
  const count: Record<string, number> = { measured: 0, literature: 0, "authored-estimate": 0, illustrative: 0 };
  for (const item of [...state.asset.thermal.regions, ...state.asset.thermal.emitters]) count[item.source] = (count[item.source] ?? 0) + 1;
  const list = Object.entries(count).map(([k, v]) => `${k} ${v}`).join(", ");
  return note(`Parameter sources of the ${state.asset.thermal.regions.length} regions and ${state.asset.thermal.emitters.length} emitters: ${list}.`, `Parameterquellen der ${state.asset.thermal.regions.length} Regionen und ${state.asset.thermal.emitters.length} Emitter: ${list}.`);
};

/** 🏷️ The sentence a legend states about what the parcels of a plume carry. */
export const parcelCarrierText = (carries: "airflow" | "vapour"): Text =>
  carries === "airflow" ? note("Parcels carry air flow (excess temperature along the jet).", "Teilchen tragen Luftströmung (Übertemperatur entlang des Strahls).") : note("Parcels carry water vapour (humidity excess along the plume), not heat.", "Teilchen tragen Wasserdampf (Feuchteüberschuss entlang der Fahne), keine Wärme.");

function notesOf(state: EvaluatedState, a: Ambient, defaults: readonly Slug[], skipped: readonly Slug[]): Text[] {
  const env = a.env;
  const notes: Text[] = [
    note(
      `Steady-state surface energy balance per region at Tair ${env.airTempC} °C, Tmrt ${env.meanRadiantTempC} °C, RH ${Math.round(env.relativeHumidity * 100)} %, wind ${env.windMS} m/s towards ${env.windDirectionDeg}°, solar ${env.solarWM2} W/m². Radiation: grey body; convection: ISO 7730 for clothed bodies, plate/cylinder correlations otherwise (h = max(natural, forced)); evaporation: ASHRAE wettedness with Lewis relation or Penman–Monteith leaf balance.`,
      `Stationäre Oberflächen-Energiebilanz je Region bei Tluft ${env.airTempC} °C, Tmrt ${env.meanRadiantTempC} °C, rF ${Math.round(env.relativeHumidity * 100)} %, Wind ${env.windMS} m/s Richtung ${env.windDirectionDeg}°, Solar ${env.solarWM2} W/m². Strahlung: grauer Körper; Konvektion: ISO 7730 für bekleidete Körper, sonst Platten-/Zylinderkorrelationen (h = max(natürlich, erzwungen)); Verdunstung: ASHRAE-Benetzung mit Lewis-Beziehung oder Penman-Monteith-Blattbilanz.`,
    ),
    note(
      "Emitters: free round turbulent jet (u ∝ 6.2·d/x, ΔT and vapour excess ∝ 5·d/x) or, for pulsed sources, a puff (∝ x⁻³), advected by the wind relative to the figure and lifted by entrainment-diluted buoyancy; parcels are deterministic tracers on that path with a hash-seeded lateral spread and end when temperature and vapour excess are indistinguishable from ambient. Plumes are cut at 10 m. Respiration follows ASHRAE Fundamentals ch. 9. No CFD, not physiologically validated; core-temperature rise with activity (0.15 K/met, max 1.5 K) is an authored estimate.",
      "Emitter: freier runder turbulenter Strahl (u ∝ 6,2·d/x, ΔT und Feuchteüberschuss ∝ 5·d/x) oder, bei gepulsten Quellen, ein Puff (∝ x⁻³), vom Wind relativ zur Figur getragen und durch verdünnungsbegrenzten Auftrieb gehoben; Teilchen sind deterministische Tracer auf diesem Pfad mit hash-basierter seitlicher Streuung und enden, wenn Temperatur- und Feuchteüberschuss nicht mehr von der Umgebung zu unterscheiden sind. Fahnen enden bei 10 m. Atmung nach ASHRAE Fundamentals Kap. 9. Keine CFD, nicht physiologisch validiert; der Kerntemperaturanstieg mit der Aktivität (0,15 K/met, max. 1,5 K) ist eine Autorenschätzung.",
    ),
    classify(state),
  ];
  if (defaults.length) notes.push(note(`Regions using default insulation (0.155 m²K/W) or default material (ε 0.9): ${defaults.join(", ")}.`, `Regionen mit Standard-Dämmung (0,155 m²K/W) oder Standardmaterial (ε 0,9): ${defaults.join(", ")}.`));
  if (skipped.length) notes.push(note(`Emitters without a placed anchor were skipped: ${skipped.join(", ")}.`, `Emitter ohne platzierten Anker wurden übersprungen: ${skipped.join(", ")}.`));
  return [...notes, ...state.asset.thermal.assumptions];
}
//#endregion 📝️Notes

//#region 🌡️Solve
/** 🌡️ Solves the thermal field of one evaluated state under one environment (default: the asset's reference environment).
 * Deterministic in (state, environment); the unsolved inputs are `state.thermal` (activity, operating levels, region overrides, emitter strengths), anchor world positions, bone world angles, `timeS` and the figure velocity. */
export function solveThermal(state: EvaluatedState, environment: Environment = state.asset.thermal.environment): ThermalField {
  const a = ambientOf(environment, state);
  const { fields, defaults } = solveRegions(state, a);
  const regions = Object.values(fields);
  const sum = (pick: (f: RegionField) => number) => regions.reduce((s, f) => s + pick(f), 0);
  const emitters = state.asset.thermal.emitters;
  const results = emitters.map((e) => emitterField(state, e, a));
  const skipped = emitters.filter((e) => !state.anchors[e.anchor] && (state.thermal.emitters[e.id] ?? 0) > 0).map((e) => e.id);
  const breath = emitters.filter((e) => e.kind === "breath");
  const strength = breath.length ? breath.reduce((s, e) => s + Math.min(1, Math.max(0, state.thermal.emitters[e.id] ?? 0)), 0) / breath.length : 0;
  const area = duboisAreaM2(state.asset.dimensions.massKg, state.asset.dimensions.heightM);
  const res = respirationWM2(state.thermal.activityMet * MET_WM2, environment.airTempC, a.vapourPa);
  return {
    environment,
    regions: fields,
    totals: {
      radiantW: sum((f) => f.radiantW),
      convectiveW: sum((f) => f.convectiveW),
      latentW: sum((f) => f.latentW),
      emitterSensibleW: results.reduce((s, r) => s + r.sensibleW, 0) + res.sensible * area * strength,
      emitterLatentW: results.reduce((s, r) => s + r.latentW, 0) + res.latent * area * strength,
    },
    streamlines: results.flatMap((r) => (r.streamline ? [r.streamline] : [])),
    parcels: results.flatMap((r) => r.parcels),
    notes: notesOf(state, a, defaults, skipped),
  };
}
//#endregion 🌡️Solve

//#region 🎨️Legend
/** 🎨️ A thermal legend: the contract {@link Legend} plus its `centre` value; stop positions `at` are normalised 0 … 1 along min → max. */
export type ThermalLegend = Legend & { readonly centre: number };

const COOL_FAR = "#2c7bb6";
const COOL_NEAR = "#abd9e9";
const NEUTRAL = "#e6e6e6";
const WARM_NEAR = "#fdae61";
const WARM_FAR = "#d7301f";

const QUANTITY_META: Readonly<Record<ThermalQuantity, { readonly unit: string; readonly title: Text; readonly pick: (f: RegionField) => number }>> = {
  "surface-temperature": { unit: "°C", title: note("Surface temperature", "Oberflächentemperatur"), pick: (f) => f.surfaceTempC },
  "radiant-flux": { unit: "W/m²", title: note("Radiant heat flux", "Strahlungswärmestrom"), pick: (f) => f.radiantFluxWM2 },
  "convective-flux": { unit: "W/m²", title: note("Convective heat flux", "Konvektiver Wärmestrom"), pick: (f) => f.convectiveFluxWM2 },
  "latent-flux": { unit: "W/m²", title: note("Latent heat flux", "Latenter Wärmestrom"), pick: (f) => f.latentFluxWM2 },
  "net-flux": { unit: "W/m²", title: note("Net heat flux", "Netto-Wärmestrom"), pick: (f) => f.netFluxWM2 },
};

/** 🎚️ Min, max and centre plus warm/cool colour stops for a thermal quantity: centred on the air temperature for surface temperature and on 0 for fluxes;
 * `auto` takes the symmetric extent of the field (at least 1 K or 1 W/m²), `fixed` uses the given bounds. */
export function thermalLegend(quantity: ThermalQuantity, field: ThermalField, range: DisplayRange): ThermalLegend {
  const meta = QUANTITY_META[quantity];
  const centre = quantity === "surface-temperature" ? field.environment.airTempC : 0;
  const values = Object.values(field.regions).map(meta.pick);
  const half = Math.max(1, ...values.map((v) => Math.abs(v - centre)));
  const [min, max] = range.mode === "fixed" ? [range.min, range.max] : [centre - half, centre + half];
  const where = (min === max ? 0.5 : Math.min(1, Math.max(0, (centre - min) / (max - min))));
  const cool = where > 0 ? [{ at: 0, color: COOL_FAR }, { at: where / 2, color: COOL_NEAR }] : [];
  const warm = where < 1 ? [{ at: where + (1 - where) / 2, color: WARM_NEAR }, { at: 1, color: WARM_FAR }] : [];
  const stops = [...cool, { at: where, color: NEUTRAL }, ...warm];
  const unit = meta.unit;
  const isTemperature = quantity === "surface-temperature";
  return {
    title: meta.title,
    quantity,
    unit,
    centre,
    min,
    max,
    stops,
    convention: isTemperature
      ? note(`Neutral = air temperature ${centre} °C; warm colours above, cool colours below.`, `Neutral = Lufttemperatur ${centre} °C; warme Farben darüber, kühle darunter.`)
      : note("Neutral = 0 W/m²; warm = heat leaves the surface, cool = surface gains heat.", "Neutral = 0 W/m²; warm = Wärme verlässt die Oberfläche, kühl = Oberfläche nimmt Wärme auf."),
    interpretation: isTemperature
      ? note("Steady-state estimate per region, not a measurement.", "Stationäre Schätzung je Region, keine Messung.")
      : note("Steady-state flux per region area; positive values leave the surface (solar gain is subtracted in the net flux).", "Stationärer Strom je Regionsfläche; positive Werte verlassen die Oberfläche (Solargewinn wird im Nettostrom abgezogen)."),
  };
}

/** 🖍️ The sRGB colour of a value on a thermal legend: piecewise linear between the stops, clamped at the ends. */
export function thermalColour(legend: Legend, value: number): string {
  const x = legend.max === legend.min ? 0.5 : Math.min(1, Math.max(0, (value - legend.min) / (legend.max - legend.min)));
  const stops = legend.stops;
  let i = 1;
  while (i < stops.length - 1 && (stops[i] as { at: number }).at < x) i++;
  const [s0, s1] = [stops[i - 1], stops[i]] as [{ at: number; color: string }, { at: number; color: string }];
  const f = s1.at > s0.at ? Math.min(1, Math.max(0, (x - s0.at) / (s1.at - s0.at))) : 1;
  const channel = (hex: string, k: number) => parseInt(hex.slice(1 + 2 * k, 3 + 2 * k), 16);
  const out = [0, 1, 2].map((k) => Math.round(channel(s0.color, k) * (1 - f) + channel(s1.color, k) * f).toString(16).padStart(2, "0"));
  return `#${out.join("")}`;
}
//#endregion 🎨️Legend

//#region 💨️Plume legend
/** 💨️ The Legend of the parcels and streamlines of a field. Parcels carrying air flow are painted by temperature excess over ambient (K), parcels carrying vapour by
 * the vapour excess over ambient (g/kg), never the absolute humidity; the legend states what the parcels carry. `at` is normalised 0 … 1 along min (0) → max. */
export function plumeLegend(carries: "airflow" | "vapour", field: ThermalField, range: DisplayRange): ThermalLegend {
  const humidity = specificHumidity(vapourPressurePa(field.environment), field.environment.pressurePa);
  const vapour = carries === "vapour";
  const values = field.parcels.filter((p) => p.carries === carries).map((p) => (vapour ? (p.vapourKgPerKg - humidity) * 1000 : Math.abs(p.excessK)));
  const top = Math.max(vapour ? 0.5 : 1, ...values);
  const [min, max] = range.mode === "fixed" ? [range.min, range.max] : [0, top];
  return {
    title: vapour ? note("Vapour excess over ambient", "Dampfüberschuss gegenüber der Umgebung") : note("Air temperature excess over ambient", "Lufttemperatur-Überschuss gegenüber der Umgebung"),
    quantity: vapour ? "vapour-excess" : "excess-temperature",
    unit: vapour ? "g/kg" : "K",
    centre: 0,
    min,
    max,
    stops: vapour ? [{ at: 0, color: NEUTRAL }, { at: 0.5, color: COOL_NEAR }, { at: 1, color: COOL_FAR }] : [{ at: 0, color: NEUTRAL }, { at: 0.5, color: WARM_NEAR }, { at: 1, color: WARM_FAR }],
    convention: note("Neutral = ambient (0); stronger colour = larger excess.", "Neutral = Umgebung (0); kräftigere Farbe = größerer Überschuss."),
    interpretation: parcelCarrierText(carries),
  };
}
//#endregion 💨️Plume legend
