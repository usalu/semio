/** 🔬️ Physical building blocks of the thermal solver: constants, psychrometrics, air properties, grey-body radiation and convection correlations by geometry.
 *
 * Temperatures in the API are °C and are converted to kelvin inside; fluxes are W/m² and positive when leaving the surface.
 *
 * @see ../../../🧬️schema/🟦️.ts — Environment, ThermalRegion
 * @see https://doi.org/10.1002/9781119175957 — Incropera & DeWitt, Fundamentals of Heat and Mass Transfer (correlations)
 */
import type { Environment, ThermalRegion } from "../../../🧬️schema/🟦️.ts";

/** ⚛️ Stefan–Boltzmann constant, W/(m² K⁴) (CODATA 2018). */
export const SIGMA = 5.670374419e-8;

/** 🌡️ 0 °C in kelvin. */
export const KELVIN = 273.15;

/** 🪂️ Standard gravity, m/s². */
export const GRAVITY = 9.80665;

/** 🌬️ Specific heat of dry air, J/(kg K). */
export const CP_AIR = 1006;

/** 🌬️ Specific gas constant of dry air, J/(kg K). */
export const R_DRY = 287.058;

/** 💧️ Specific gas constant of water vapour, J/(kg K). */
export const R_VAPOUR = 461.495;

/** 🌬️ Standard pressure, Pa. */
export const STANDARD_PRESSURE = 101325;

/** 🧮️ Lewis relation of ISO 13731 / ASHRAE: evaporative over convective coefficient, W/(m² kPa) per W/(m² K). */
export const LEWIS_RATIO = 16.5;

/** 🔥️ 1 met in W/m² (ASHRAE 55). */
export const MET_WM2 = 58.15;

/** 🌡️ Converts °C to K (clamped above 1 K so power-law air properties stay real during root bracketing). */
export const kelvin = (tC: number): number => Math.max(1, tC + KELVIN);

/** 💧️ Saturation vapour pressure of water over liquid, Pa: Magnus form of Buck (1996), within 0.05 % of IAPWS-95 between 0 and 60 °C. */
export function saturationPressurePa(tC: number): number {
  return 611.21 * Math.exp((18.678 - tC / 234.5) * (tC / (257.14 + tC)));
}

/** 💧️ Actual vapour pressure of the ambient air, Pa. */
export const vapourPressurePa = (env: Pick<Environment, "airTempC" | "relativeHumidity">): number => env.relativeHumidity * saturationPressurePa(env.airTempC);

/** 💧️ Specific humidity (kg vapour per kg moist air) at vapour pressure `pv` and total pressure `p`. */
export const specificHumidity = (pv: number, p: number): number => (0.622 * pv) / (p - 0.378 * pv);

/** 💧️ Humidity ratio (kg vapour per kg dry air). */
export const humidityRatio = (pv: number, p: number): number => (0.622 * pv) / (p - pv);

/** 💧️ Latent heat of vaporisation of water, J/kg. */
export const latentHeatJkg = (tC: number): number => 2.501e6 - 2361 * tC;

/** 🌬️ Density of moist air, kg/m³. */
export const airDensity = (tC: number, p: number, pv: number): number => (p - pv) / (R_DRY * kelvin(tC)) + pv / (R_VAPOUR * kelvin(tC));

/** 🌬️ Thermophysical properties of air at film temperature `tC` and pressure `p`: kinematic viscosity, thermal diffusivity, conductivity, Prandtl number.
 * Sutherland viscosity and a power-law conductivity fit, within 1 % of tabulated values from 250 to 400 K. */
export function airProperties(tC: number, p: number): { readonly nu: number; readonly alpha: number; readonly k: number; readonly pr: number; readonly rho: number } {
  const t = kelvin(tC);
  const mu = (1.458e-6 * t ** 1.5) / (t + 110.4);
  const k = 0.0241 * (t / KELVIN) ** 0.9;
  const rho = p / (R_DRY * t);
  return { nu: mu / rho, alpha: k / (rho * CP_AIR), k, pr: (mu * CP_AIR) / k, rho };
}

/** ☀️ Grey-body longwave exchange, W/m²: `ε σ (T_s⁴ − T_env⁴)`, positive = the surface loses heat. */
export const radiationFlux = (emissivity: number, surfaceC: number, envC: number): number => emissivity * SIGMA * (kelvin(surfaceC) ** 4 - kelvin(envC) ** 4);

/** ☀️ Emitted radiant flux `ε σ T⁴`, W/m². */
export const emittedFlux = (emissivity: number, surfaceC: number): number => emissivity * SIGMA * kelvin(surfaceC) ** 4;

/** ☀️ Secant radiation coefficient `q_r / (T_s − T_env)`, W/(m² K); the tangent `4 ε σ T³` at equality. */
export function radiationCoefficient(emissivity: number, surfaceC: number, envC: number): number {
  const d = surfaceC - envC;
  return Math.abs(d) < 1e-9 ? 4 * emissivity * SIGMA * kelvin(surfaceC) ** 3 : radiationFlux(emissivity, surfaceC, envC) / d;
}

/** ☀️ The view factors of a surface towards the sky, the ground and the surroundings (other surfaces at the mean radiant temperature), summing to 1.
 * Ground share of the non-sky hemisphere: 1 for down-facing plates, 0 for up-facing plates, 0.5 for vertical plates, cylinders and spheres (authored-estimate). */
export function viewFactors(geometry: ThermalRegion["geometry"], skyViewFactor: number): { readonly sky: number; readonly ground: number; readonly surroundings: number } {
  const groundShare = geometry === "horizontal-plate-down" ? 1 : geometry === "horizontal-plate-up" ? 0 : 0.5;
  const ground = (1 - skyViewFactor) * groundShare;
  return { sky: skyViewFactor, ground, surroundings: 1 - skyViewFactor - ground };
}

/** ☀️ The three radiant temperatures (°C) of an environment: sky, ground and surroundings; `skyTempC` and `groundTempC` default to the mean radiant temperature. */
export const radiantTemperatures = (env: Environment): { readonly sky: number; readonly ground: number; readonly surroundings: number } => ({ sky: env.skyTempC ?? env.meanRadiantTempC, ground: env.groundTempC ?? env.meanRadiantTempC, surroundings: env.meanRadiantTempC });

/** ☀️ The effective radiant environment temperature (°C) a region sees: sky, ground and surroundings weighted by their view factors in T⁴, so that
 * `ε σ (T_s⁴ − T_env⁴)` equals the sum of the three exchanges. */
export function radiantEnvironmentC(env: Environment, geometry: ThermalRegion["geometry"], skyViewFactor: number): number {
  const f = viewFactors(geometry, skyViewFactor);
  const t = radiantTemperatures(env);
  return (f.sky * kelvin(t.sky) ** 4 + f.ground * kelvin(t.ground) ** 4 + f.surroundings * kelvin(t.surroundings) ** 4) ** 0.25 - KELVIN;
}

/** ☀️ Share of the beam irradiance (normal to the sun) a unit of surface absorbs on average, by geometry: sphere ¼ and cylinder 1/π (projected over surface area),
 * vertical plate ½ and up-facing plate 0.7 (mean over sun elevations, authored-estimate), down-facing plate 0.1 (reflected ground light only). */
export const SOLAR_GEOMETRY_FACTOR: Readonly<Record<ThermalRegion["geometry"], number>> = { sphere: 0.25, cylinder: 1 / Math.PI, "vertical-plate": 0.5, "horizontal-plate-up": 0.7, "horizontal-plate-down": 0.1 };

/** ☀️ Solar power absorbed per unit of surface, W/m²: `α G f_geo (sunlit + diffuseFraction (1 − sunlit))`. */
export const absorbedSolar = (alpha: number, env: Pick<Environment, "solarWM2" | "diffuseFraction">, geometry: ThermalRegion["geometry"], sunlit: number): number => alpha * env.solarWM2 * SOLAR_GEOMETRY_FACTOR[geometry] * (sunlit + env.diffuseFraction * (1 - sunlit));

/** 🌀️ Convective heat transfer of one surface to air. */
export type Convection = { readonly h: number; readonly natural: number; readonly forced: number };

function rayleigh(dT: number, length: number, film: ReturnType<typeof airProperties>, tFilmC: number): number {
  return (GRAVITY * Math.abs(dT) * length ** 3) / (kelvin(tFilmC) * film.nu * film.alpha);
}

function naturalNusselt(geometry: ThermalRegion["geometry"], ra: number, pr: number, heated: boolean): number {
  const psi = (c: number): number => (1 + (c / pr) ** (9 / 16)) ** (8 / 27);
  if (geometry === "vertical-plate") return (0.825 + (0.387 * ra ** (1 / 6)) / psi(0.492)) ** 2;
  if (geometry === "cylinder") return (0.6 + (0.387 * ra ** (1 / 6)) / psi(0.559)) ** 2;
  if (geometry === "sphere") return 2 + (0.589 * ra ** 0.25) / (1 + (0.469 / pr) ** (9 / 16)) ** (4 / 9);
  const facesFlow = (geometry === "horizontal-plate-up") === heated;
  if (!facesFlow) return 0.27 * ra ** 0.25;
  return ra < 1e7 ? 0.54 * ra ** 0.25 : 0.15 * ra ** (1 / 3);
}

function forcedNusselt(geometry: ThermalRegion["geometry"], re: number, pr: number): number {
  if (re <= 0) return 0;
  if (geometry === "cylinder") return 0.3 + ((0.62 * re ** 0.5 * pr ** (1 / 3)) / (1 + (0.4 / pr) ** (2 / 3)) ** 0.25) * (1 + (re / 282000) ** (5 / 8)) ** (4 / 5);
  if (geometry === "sphere") return 2 + (0.4 * re ** 0.5 + 0.06 * re ** (2 / 3)) * pr ** 0.4;
  return re < 5e5 ? 0.664 * re ** 0.5 * pr ** (1 / 3) : (0.037 * re ** 0.8 - 871) * pr ** (1 / 3);
}

/** 🌀️ Convection coefficient of a region surface, W/(m² K): natural convection (Churchill–Chu for vertical plates and horizontal cylinders, Churchill for spheres,
 * McAdams/Lloyd–Moran for horizontal plates) and forced convection (laminar/turbulent flat plate; Churchill–Bernstein for cylinders; Whitaker for spheres) in the
 * mixed-convection blend `h = (h_nat³ + h_forced³)^⅓` (Churchill 1977, transverse flow). `speed` is the air speed relative to the surface, `length` the characteristic
 * length (plate length, cylinder or sphere diameter).
 * @see https://doi.org/10.1002/9781119175957 */
export function convection(geometry: ThermalRegion["geometry"], length: number, surfaceC: number, airC: number, speed: number, pressurePa: number): Convection {
  const tFilm = (surfaceC + airC) / 2;
  const film = airProperties(tFilm, pressurePa);
  const dT = surfaceC - airC;
  const ra = rayleigh(dT, length, film, tFilm);
  const natural = ra > 0 ? (naturalNusselt(geometry, ra, film.pr, dT > 0) * film.k) / length : 0;
  const re = (speed * length) / film.nu;
  const forced = (forcedNusselt(geometry, re, film.pr) * film.k) / length;
  return { h: (natural ** 3 + forced ** 3) ** (1 / 3), natural, forced };
}

/** 💧️ Evaporative mass-transfer coefficient from the Lewis relation, W/(m² Pa): `16.5 h_c`, pressure corrected. */
export const evaporativeCoefficient = (hc: number, pressurePa: number): number => ((LEWIS_RATIO * hc) / 1000) * (STANDARD_PRESSURE / pressurePa);

/** 🧮️ Finds the root of a strictly decreasing function by Illinois regula falsi, bracketing by expansion from `guess`. */
export function solveDecreasing(f: (x: number) => number, guess: number, step = 5, tolerance = 1e-10): number {
  let lo = guess - step;
  let hi = guess + step;
  let flo = f(lo);
  let fhi = f(hi);
  for (let i = 0; i < 40 && flo < 0; i++) {
    hi = lo;
    fhi = flo;
    lo -= step * 2 ** i;
    flo = f(lo);
  }
  for (let i = 0; i < 40 && fhi > 0; i++) {
    lo = hi;
    flo = fhi;
    hi += step * 2 ** i;
    fhi = f(hi);
  }
  let side = 0;
  for (let i = 0; i < 100 && hi - lo > tolerance; i++) {
    const x = (lo * fhi - hi * flo) / (fhi - flo);
    const fx = f(x);
    if (Math.abs(fx) < tolerance * 1e-3) return x;
    if (fx > 0) {
      lo = x;
      flo = fx;
      if (side === 1) fhi /= 2;
      side = 1;
    } else {
      hi = x;
      fhi = fx;
      if (side === -1) flo /= 2;
      side = -1;
    }
  }
  return (lo + hi) / 2;
}
