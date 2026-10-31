/** 🍃️ Stomatal conductance of leaves: a Jarvis-type multiplicative model scaled to the authored maximum.
 *
 * `g_s = g_max · max(g_min, f_light · f_vpd)` for C3 and C4 plants; CAM plants open their stomata in the dark and close them in the light,
 * `g_s = g_max · (0.05 + 0.95 (1 − f_light))`, with no vapour-pressure-deficit response. `f_light = Q/(Q + Q50) (1 + Q50/Q_ref)` capped at 1 with incident shortwave
 * `Q` (W/m²), `Q50 = 100 W/m²`, `Q_ref = 500 W/m²`; `f_vpd = (1 + D_ref/D0)/(1 + D/D0)` capped at 1 with `D_ref = 0.5 kPa`, `D0 = 2 kPa` (C3) or 3 kPa (C4);
 * `g_min = 0.02`. The shapes follow Jarvis (1976); the parameter values are illustrative, not fitted to a species.
 * A leaf region's `areaM2` is its one-sided leaf area; `leafAreaIndex` (leaf area over crown ground projection) converts leaf fluxes to ground-area fluxes ({@link perGroundArea}).
 * The incident shortwave of a randomly oriented leaf is `G/2` per one-sided area (spherical leaf-angle distribution, Campbell & Norman 1998, ch. 15).
 *
 * @see https://doi.org/10.1098/rstb.1976.0035 — Jarvis 1976, The interpretation of the variations in leaf water potential and stomatal conductance
 * @see ../../../🧬️schema/🟦️.ts — ThermalRegion.leaf
 */
import type { ThermalRegion } from "../../../🧬️schema/🟦️.ts";
import { CP_AIR, airDensity, latentHeatJkg, saturationPressurePa } from "../🔬️physics/🟦️.ts";

const Q_HALF = 100;
const Q_REF = 500;
const D_REF_KPA = 0.5;
const G_MIN = 0.02;
const MEAN_LEAF_PROJECTION = 0.5;

/** 🍃️ The light response `f_light` of 0 (dark) to 1 (saturating light) for incident shortwave `q` W/m². */
export const lightResponse = (q: number): number => Math.min(1, (Math.max(0, q) / (Math.max(0, q) + Q_HALF)) * (1 + Q_HALF / Q_REF));

/** 🍃️ Incident shortwave on a leaf of random orientation, W/m²: `G · ½ · (sunlit + diffuseFraction (1 − sunlit))`. */
export const leafLight = (solarWM2: number, diffuseFraction: number, sunlit: number): number => solarWM2 * MEAN_LEAF_PROJECTION * (sunlit + diffuseFraction * (1 - sunlit));

/** 🍃️ Stomatal conductance to water vapour, m/s, at incident light `q` W/m², air temperature and relative humidity. */
export function stomatalConductance(leaf: NonNullable<ThermalRegion["leaf"]>, q: number, airTempC: number, relativeHumidity: number): number {
  const light = lightResponse(q);
  if (leaf.photosynthesis === "cam") return leaf.stomatalConductanceMaxMS * (0.05 + 0.95 * (1 - light));
  const d0 = leaf.photosynthesis === "c4" ? 3 : 2;
  const deficitKpa = (saturationPressurePa(airTempC) * (1 - relativeHumidity)) / 1000;
  const vpd = Math.min(1, (1 + D_REF_KPA / d0) / (1 + deficitKpa / d0));
  return leaf.stomatalConductanceMaxMS * Math.max(G_MIN, light * vpd);
}

/** 🍃️ Converts a flux per one-sided leaf area to a flux per ground-projection area: `flux × leafAreaIndex`. */
export const perGroundArea = (leaf: NonNullable<ThermalRegion["leaf"]>, fluxWM2: number): number => fluxWM2 * leaf.leafAreaIndex;

/** 🍃️ The inputs of the Penman–Monteith equation: net radiation and ground heat flux (W/m²), air temperature and pressure, actual vapour pressure (Pa), optionally the vapour pressure deficit `D` (Pa; default `e_s(T) − e_a`, FAO-56 uses the mean of the saturation pressures at the daily extremes) and the aerodynamic and surface resistance (s/m). */
export type PenmanMonteithInput = { readonly netRadiationWM2: number; readonly groundHeatWM2: number; readonly airTempC: number; readonly pressurePa: number; readonly vapourPressurePa: number; readonly vapourPressureDeficitPa?: number; readonly aerodynamicResistanceSM: number; readonly surfaceResistanceSM: number };

/** 🍃️ Penman–Monteith latent heat flux `λE = (Δ (R_n − G) + ρ c_p D / r_a) / (Δ + γ (1 + r_s / r_a))`, W/m² (FAO-56 equation 3), with `Δ` the slope of the saturation vapour pressure curve,
 * `γ = c_p P / (0.622 λ)` and `D = e_s(T) − e_a`. The surface balance of the 🧱️balance module is the same physics with the radiative exchange solved jointly with the leaf temperature.
 * @see https://www.fao.org/4/x0490e/x0490e06.htm — Allen, Pereira, Raes, Smith 1998, FAO Irrigation and Drainage Paper 56, chapter 3 */
export function penmanMonteith(i: PenmanMonteithInput): number {
  const delta = (saturationPressurePa(i.airTempC + 0.005) - saturationPressurePa(i.airTempC - 0.005)) / 0.01;
  const gamma = (CP_AIR * i.pressurePa) / (0.622 * latentHeatJkg(i.airTempC));
  const deficit = i.vapourPressureDeficitPa ?? saturationPressurePa(i.airTempC) - i.vapourPressurePa;
  const rhoCp = airDensity(i.airTempC, i.pressurePa, i.vapourPressurePa) * CP_AIR;
  return (delta * (i.netRadiationWM2 - i.groundHeatWM2) + (rhoCp * deficit) / i.aerodynamicResistanceSM) / (delta + gamma * (1 + i.surfaceResistanceSM / i.aerodynamicResistanceSM));
}
