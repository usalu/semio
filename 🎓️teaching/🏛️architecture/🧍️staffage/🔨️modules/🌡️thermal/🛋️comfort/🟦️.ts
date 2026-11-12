/** 🛋️ Thermal comfort of a clothed, active person from the six ISO 7730 parameters: air temperature, mean radiant temperature, relative air speed, humidity, activity (met) and clothing (clo) → PMV and PPD.
 *
 * Absolute, steady-state, whole-body comfort estimate by Fanger's heat balance exactly as in ISO 7730:2005 Annex D (iterative clothing surface temperature, convective coefficient `max(forced, natural)`); it is a
 * population statistic for indoor, near-sedentary conditions, NOT the skin and core temperatures of the two-node model (🫀️physiology) and NOT a prediction for an individual.
 * Units: `clo` (1 clo = 0.155 m² K/W, ISO 9920), `met` (1 met = 58.15 W/m², ISO 8996), temperatures °C, speeds m/s, pressure Pa.
 * Validity (ISO 7730:2005 §4 and Annex D): 10 ≤ t_a ≤ 30 °C, 10 ≤ t_r ≤ 40 °C, 0 ≤ v_ar ≤ 1 m/s, 0.8 ≤ met ≤ 4, 0 ≤ clo ≤ 2, 0 ≤ p_a ≤ 2700 Pa and −2 ≤ PMV ≤ +2; outside these `valid` is false and the numbers are
 * an extrapolation. Not provided: adaptive comfort, local discomfort (draught, radiant asymmetry), SET, UTCI.
 *
 * @see https://www.iso.org/standard/39155.html — ISO 7730:2005 Ergonomics of the thermal environment, analytical determination of thermal comfort
 * @see https://www.iso.org/standard/34250.html — ISO 8996:2021 metabolic rate, 1 met = 58.15 W/m²
 * @see https://www.iso.org/standard/59937.html — ISO 9920:2007 thermal insulation and water-vapour resistance of a clothing ensemble, 1 clo = 0.155 m² K/W
 * @see https://pythermalcomfort.readthedocs.io/en/latest/documentation/models.html — pythermalcomfort `pmv_ppd_iso`, the oracle of 🧪️tests/🛋️pmv-comfort
 */
import type { Asset, Environment, Slug } from "../../../🧬️schema/🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 👕️ Thermal resistance of one clo in m² K/W (ISO 9920). */
export const CLO_M2KW = 0.155;
/** 🏃️ Metabolic rate of one met in W/m² of body surface (ISO 8996). */
export const MET_WM2_ISO = 58.15;

/** 👕️ Intrinsic clothing insulation `I_cl` in m² K/W of `clo` clo. */
export const cloToM2KW = (clo: number): number => clo * CLO_M2KW;
/** 👕️ Clothing insulation in clo of `m2KW` m² K/W. */
export const m2KWToClo = (m2KW: number): number => m2KW / CLO_M2KW;
/** 🏃️ Metabolic heat production in W/m² of `met` met. */
export const metToWM2 = (met: number): number => met * MET_WM2_ISO;

/** 💨️ Relative air speed `v_ar = v + 0.0052 (M − 58)` (ISO 7730 §Annex D note, M in W/m² above 58) of a person moving or working in air of speed `airSpeedMS`. */
export const relativeAirSpeedMS = (airSpeedMS: number, met: number): number => airSpeedMS + 0.0052 * Math.max(0, metToWM2(met) - 58);

/** 📥️ The inputs of {@link pmvPpd}. `externalWorkMet` is the mechanical power in met (default 0); `relativeAirSpeedMS` already contains the body motion. */
export type ComfortInputs = {
  readonly airTempC: number;
  readonly meanRadiantTempC: number;
  readonly relativeAirSpeedMS: number;
  readonly relativeHumidity: number;
  readonly met: number;
  readonly clo: number;
  readonly externalWorkMet?: number;
};

/** 🌡️ ISO 7730 / ASHRAE 55 seven-point sensation scale. */
export type Sensation = "cold" | "cool" | "slightly-cool" | "neutral" | "slightly-warm" | "warm" | "hot";

/** 🛋️ The comfort result: vote, dissatisfied share (%), sensation, the clothing surface temperature of the heat balance and the validity of the inputs. */
export type Comfort = {
  readonly pmv: number;
  readonly ppdPercent: number;
  readonly sensation: Sensation;
  readonly clothingSurfaceC: number;
  readonly convectiveCoefficientWM2K: number;
  readonly valid: boolean;
  readonly violations: readonly string[];
};

/** 💧️ Water-vapour partial pressure in Pa as used by ISO 7730 Annex D, `rh · 10 · exp(16.6536 − 4030.183/(t + 235))`. */
export const vapourPressureIsoPa = (airTempC: number, relativeHumidity: number): number => relativeHumidity * 100 * 10 * portable.fastExp(16.6536 - 4030.183 / (airTempC + 235));

/** 🔬️ Sensation class of a PMV (rounded to the nearest vote). */
export function sensationOf(pmv: number): Sensation {
  const v = Math.round(pmv);
  return v <= -3 ? "cold" : v === -2 ? "cool" : v === -1 ? "slightly-cool" : v === 0 ? "neutral" : v === 1 ? "slightly-warm" : v === 2 ? "warm" : "hot";
}

/** 📏️ The ISO 7730:2005 limits the inputs violate, as short English names; empty when the PMV applies. */
export function comfortViolations(i: ComfortInputs, pmv: number | null): readonly string[] {
  const out: string[] = [];
  const pa = vapourPressureIsoPa(i.airTempC, i.relativeHumidity);
  if (i.airTempC < 10 || i.airTempC > 30) out.push("air temperature outside 10..30 C");
  if (i.meanRadiantTempC < 10 || i.meanRadiantTempC > 40) out.push("mean radiant temperature outside 10..40 C");
  if (i.relativeAirSpeedMS < 0 || i.relativeAirSpeedMS > 1) out.push("relative air speed outside 0..1 m/s");
  if (i.met < 0.8 || i.met > 4) out.push("activity outside 0.8..4 met");
  if (i.clo < 0 || i.clo > 2) out.push("clothing outside 0..2 clo");
  if (pa < 0 || pa > 2700) out.push("vapour pressure outside 0..2700 Pa");
  if (pmv !== null && (pmv < -2 || pmv > 2)) out.push("PMV outside -2..+2");
  return out;
}

/** 🛋️ Predicted mean vote and predicted percentage dissatisfied by ISO 7730:2005 Annex D. Deterministic and pure; throws when the clothing temperature iteration does not converge (> 150 rounds). */
export function pmvPpd(i: ComfortInputs): Comfort {
  const ta = i.airTempC;
  const tra = i.meanRadiantTempC + 273;
  const taa = ta + 273;
  const m = metToWM2(i.met);
  const mw = m - metToWM2(i.externalWorkMet ?? 0);
  const pa = vapourPressureIsoPa(ta, i.relativeHumidity);
  const icl = cloToM2KW(i.clo);
  const fcl = icl <= 0.078 ? 1 + 1.29 * icl : 1.05 + 0.645 * icl;
  const hcf = 12.1 * Math.sqrt(Math.max(0, i.relativeAirSpeedMS));
  const p1 = icl * fcl;
  const p2 = p1 * 3.96;
  const p3 = p1 * 100;
  const p4 = p1 * taa;
  const p5 = 308.7 - 0.028 * mw + p2 * portable.fastPow((tra / 100), 4);
  let xn = (taa + (35.5 - ta) / (3.5 * icl + 0.1)) / 100;
  let xf = xn * 2;
  let hc = hcf;
  let rounds = 0;
  while (Math.abs(xn - xf) > 0.00015) {
    xf = (xf + xn) / 2;
    hc = Math.max(hcf, 2.38 * portable.fastPow(Math.abs(100 * xf - taa), 0.25));
    xn = (p5 + p4 * hc - p2 * portable.fastPow(xf, 4)) / (100 + p3 * hc);
    if (++rounds > 150) throw new RangeError("pmvPpd: the clothing surface temperature iteration did not converge");
  }
  const tcl = 100 * xn - 273;
  const hl1 = 3.05e-3 * (5733 - 6.99 * mw - pa);
  const hl2 = mw > MET_WM2_ISO ? 0.42 * (mw - MET_WM2_ISO) : 0;
  const hl3 = 1.7e-5 * m * (5867 - pa);
  const hl4 = 0.0014 * m * (34 - ta);
  const hl5 = 3.96 * fcl * (portable.fastPow(xn, 4) - portable.fastPow((tra / 100), 4));
  const hl6 = fcl * hc * (tcl - ta);
  const pmv = (0.303 * portable.fastExp(-0.036 * m) + 0.028) * (mw - hl1 - hl2 - hl3 - hl4 - hl5 - hl6);
  const violations = comfortViolations(i, pmv);
  return { pmv, ppdPercent: 100 - 95 * portable.fastExp(-0.03353 * portable.fastPow(pmv, 4) - 0.2179 * pmv ** 2), sensation: sensationOf(pmv), clothingSurfaceC: tcl, convectiveCoefficientWM2K: hc, valid: violations.length === 0, violations };
}

/** 👕️ Skin-area-weighted intrinsic clothing insulation in clo of the body of an asset (`subject` null) or one passenger: per body region the clothing layers' resistance referred to the outer area divided by the area factor `f_cl = outer area / skin area` (the `I_cl · f_cl` convention of the schema) over `0.155`. Fur, feathers and hair are not clothing. `null` when the asset has no such body. */
export function clothingInsulationClo(asset: Asset, subject: Slug | null = null): number | null {
  const physiology = subject === null ? asset.thermal.body : asset.thermal.passengers.find((p) => p.id === subject)?.physiology;
  if (physiology === undefined) return null;
  let skin = 0;
  let weighted = 0;
  for (const r of asset.thermal.regions) {
    if (r.model !== "body" || r.body === undefined || (r.body.subject ?? null) !== subject) continue;
    const skinArea = r.body.skinFraction * physiology.skinAreaM2;
    const fcl = r.areaM2 / skinArea;
    const resistance = r.layers.filter((l) => l.kind === "clothing").reduce((s, l) => s + l.resistanceM2KW, 0);
    skin += skinArea;
    weighted += skinArea * (resistance / fcl);
  }
  return skin > 0 ? m2KWToClo(weighted / skin) : null;
}

/** 🧍️ The ISO 7730 comfort of the body of an asset in an environment at an activity: `clo` from the asset's clothing layers ({@link clothingInsulationClo}), `t_r` and `t_a` of the environment, relative air speed from the wind and the activity. `null` without a body. */
export function bodyComfort(asset: Asset, environment: Environment, activityMet: number, subject: Slug | null = null): Comfort | null {
  const clo = clothingInsulationClo(asset, subject);
  if (clo === null) return null;
  return pmvPpd({ airTempC: environment.airTempC, meanRadiantTempC: environment.meanRadiantTempC, relativeAirSpeedMS: relativeAirSpeedMS(environment.windMS, activityMet), relativeHumidity: environment.relativeHumidity, met: activityMet, clo });
}
