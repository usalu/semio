/** 🧱️ The outer-surface energy balance of one thermal region and its steady-state solution.
 *
 * Per unit of outer area, for the exposed share `e` and the contact share `c` of the area (the rest is adiabatic):
 * `q_in(T_s) + e·q_sol = e′·(q_rad + q_conv + q_evap) + c·h_ct·(T_s − T_contact)` with `e′ = max(e, 0.05)` (enclosed regions couple weakly to the room air).
 * `q_in` is the conduction from inside: through the layers from the skin (bodies, handled in 🫀️physiology), from the interior (conditioned elements), the heat
 * released inside (operating surfaces) or zero (passive surfaces, leaves). Evaporation: leaves by Penman–Monteith series conductance (stomata and boundary layer), wet surfaces by the Lewis relation.
 * Leaves exchange through both faces (radiation, convection), transpire through one, and take up dew (negative latent flux through the boundary layer only) when colder than the dew point.
 *
 * @see ../🔬️physics/🟦️.ts — correlations
 * @see ../🍃️canopy/🟦️.ts — stomatal conductance
 * @see https://doi.org/10.1017/CBO9781107415324 — Monteith & Unsworth, Principles of Environmental Physics
 */
import type { Environment, Material, ThermalRegion } from "../../../🧬️schema/🟦️.ts";
import { CP_AIR, SIGMA, absorbedSolar, airDensity, convection, emittedFlux, evaporativeCoefficient, kelvin, latentHeatJkg, radiantEnvironmentC, radiantTemperatures, radiationCoefficient, saturationPressurePa, solveDecreasing, specificHumidity, vapourPressurePa, viewFactors } from "../🔬️physics/🟦️.ts";
import { leafLight, stomatalConductance } from "../🍃️canopy/🟦️.ts";

/** 🤝️ Surface conductance of a contact area (seat, ground, handle), W/(m² K); illustrative. */
export const CONTACT_CONDUCTANCE = 20;

/** 🧱️ Smallest share of the area that couples to the air in the solution of the temperature (stowed or enclosed regions with `exposedFraction` 0 sit in the enclosed air at about room temperature instead of having no balance); reported powers still use the bound exposed fraction. Authored-estimate. */
export const ENCLOSED_EXPOSURE = 0.05;

const PRANDTL_OVER_SCHMIDT = (0.71 / 0.6) ** (2 / 3);

/** 🧱️ The static, state-independent part of a region. */
export type RegionSetup = {
  readonly region: ThermalRegion;
  readonly emissivity: number;
  readonly absorptivity: number;
  readonly layerResistance: number;
  readonly layerVapourResistance: number;
  readonly faces: 1 | 2;
  readonly wetDefault: number;
};

/** 🧱️ The state-dependent inputs of one region at one instant. */
export type RegionFrame = {
  readonly setup: RegionSetup;
  readonly exposed: number;
  readonly contact: number;
  readonly contactTempC: number;
  readonly wettedness: number;
  readonly sunlit: number;
  readonly speed: number;
  readonly envC: number;
  readonly solar: number;
  readonly stomatalMS: number;
};

/** 🧱️ The ambient air of one instant. */
export type Ambient = { readonly env: Environment; readonly pv: number; readonly rho: number; readonly qa: number };

/** 🧱️ The exchange of a surface at temperature `T_s`, per unit of exposed area (`contactWM2` per contact area). */
export type Exchange = { readonly radiative: number; readonly sensible: number; readonly latent: number; readonly contact: number; readonly hc: number; readonly hr: number };

/** 🧱️ Builds the static setup of a region from its outermost layer's material. */
export function regionSetup(region: ThermalRegion, materials: readonly Material[]): RegionSetup {
  const outer = region.layers.length > 0 ? region.layers[region.layers.length - 1]!.material : region.material;
  const material = materials.find((m) => m.id === outer) ?? materials.find((m) => m.id === region.material);
  return {
    region,
    emissivity: material?.emissivity ?? 0.9,
    absorptivity: material?.solarAbsorptivity ?? 0,
    layerResistance: region.layers.reduce((s, l) => s + l.resistanceM2KW, 0),
    layerVapourResistance: region.layers.reduce((s, l) => s + (l.vapourResistanceM2PaW ?? 0), 0),
    faces: region.model === "leaf" ? 2 : 1,
    wetDefault: region.kind === "water" ? 1 : 0,
  };
}

/** 🧱️ Ambient quantities of an environment. */
export function ambientOf(env: Environment): Ambient {
  const pv = vapourPressurePa(env);
  return { env, pv, rho: airDensity(env.airTempC, env.pressurePa, pv), qa: specificHumidity(pv, env.pressurePa) };
}

/** 🧱️ The exchange of a surface at `tsC` with its surroundings, per unit of exposed area (leaves per one-sided leaf area, two faces included). */
export function exchange(f: RegionFrame, a: Ambient, tsC: number): Exchange {
  const s = f.setup;
  const r = s.region;
  const conv = convection(r.geometry, r.characteristicLengthM, tsC, a.env.airTempC, f.speed, a.env.pressurePa);
  const hc = conv.h * s.faces;
  const radiative = s.faces * s.emissivity * SIGMA * (kelvin(tsC) ** 4 - kelvin(f.envC) ** 4);
  const sensible = hc * (tsC - a.env.airTempC);
  let latent = 0;
  if (r.model === "leaf") {
    const gb = (conv.h / (a.rho * CP_AIR)) * PRANDTL_OVER_SCHMIDT;
    const qs = specificHumidity(saturationPressurePa(tsC), a.env.pressurePa);
    const g = qs < a.qa ? gb : (f.stomatalMS * gb) / (f.stomatalMS + gb || 1);
    latent = latentHeatJkg(tsC) * a.rho * g * (qs - a.qa);
  } else if (f.wettedness > 0) latent = Math.max(0, f.wettedness * evaporativeCoefficient(hc, a.env.pressurePa) * (saturationPressurePa(tsC) - a.pv));
  return { radiative, sensible, latent, contact: f.contact > 0 ? CONTACT_CONDUCTANCE * (tsC - f.contactTempC) : 0, hc, hr: s.faces * radiationCoefficient(s.emissivity, tsC, f.envC) };
}

/** 🧱️ Net heat leaving the outer surface per unit of outer area, W/m²: exposed exchange minus absorbed solar plus contact conduction. */
export const outflow = (f: RegionFrame, x: Exchange): number => Math.max(f.exposed, ENCLOSED_EXPOSURE) * (x.radiative + x.sensible + x.latent) - f.exposed * f.solar + f.contact * x.contact;

/** 🧱️ The solar power absorbed per unit of exposed area for a region frame input. */
export function solarOf(s: RegionSetup, env: Environment, sunlit: number): number {
  return s.region.model === "leaf" ? s.absorptivity * leafLight(env.solarWM2, env.diffuseFraction, sunlit) : absorbedSolar(s.absorptivity, env, s.region.geometry, sunlit);
}

/** 🧱️ Radiant environment temperature of a region. */
export const envOf = (s: RegionSetup, env: Environment): number => radiantEnvironmentC(env, s.region.geometry, s.region.skyViewFactor);

/** 🧱️ Stomatal conductance of a leaf region under the frame's light. */
export function stomataOf(s: RegionSetup, env: Environment, sunlit: number): number {
  return s.region.leaf ? stomatalConductance(s.region.leaf, leafLight(env.solarWM2, env.diffuseFraction, sunlit), env.airTempC, env.relativeHumidity) : 0;
}

/** 🧱️ Heat arriving at the outer surface from inside, W/m² of outer area, as a function of the surface temperature, and its inside temperature.
 * Passive regions and leaves: none. Operating: the released heat `heatW × level / area`. Conditioned: `(T_interior − T_s) / (ΣR + R_film)`. */
export function inflowOf(f: RegionFrame, level: number): (tsC: number) => number {
  const r = f.setup.region;
  if (r.model === "operating" && r.operating) {
    const q = (r.operating.heatW * level) / r.areaM2;
    return () => q;
  }
  if (r.model === "conditioned" && r.conditioned) {
    const resistance = f.setup.layerResistance + r.conditioned.interiorFilmM2KW;
    return (ts) => (r.conditioned!.interiorTempC - ts) / resistance;
  }
  return () => 0;
}

/** 🧱️ Steady-state surface temperature of a non-body region: root of `inflow + e·q_sol − outflow = 0`, W/m². */
export function solveSteady(f: RegionFrame, a: Ambient, inflow: (tsC: number) => number, guessC: number): number {
  if (f.setup.region.model === "fixed") return f.setup.region.fixed?.surfaceTempC ?? guessC;
  return solveDecreasing((ts) => inflow(ts) - outflow(f, exchange(f, a, ts)), guessC);
}

/** 🧱️ The residual power `inflow − outflow` of a region at surface temperature `tsC`, W/m² of outer area: positive = the region heats up, zero at the steady state. */
export const residualWM2 = (f: RegionFrame, a: Ambient, inflow: (tsC: number) => number, tsC: number): number => inflow(tsC) - outflow(f, exchange(f, a, tsC));

/** 🧱️ The restoring conductance `−∂(inflow − outflow)/∂T_s` of a region at `tsC`, W/(m² K): divides the heat capacity into the time constant. */
export function restoringConductance(f: RegionFrame, a: Ambient, inflow: (tsC: number) => number, tsC: number): number {
  const d = 0.05;
  return Math.max(1e-3, (residualWM2(f, a, inflow, tsC - d) - residualWM2(f, a, inflow, tsC + d)) / (2 * d));
}

/** 🧱️ The net longwave exchange of a surface at `tsC` split by counterpart, W/m² of exposed area, positive = the surface loses heat; the three parts sum to {@link Exchange.radiative}. */
export type RadiationSplit = { readonly sky: number; readonly ground: number; readonly surroundings: number };

/** 🧱️ Splits the net longwave exchange of region `s` at surface temperature `tsC` into sky, ground and surroundings: `faces ε σ F_k (T_s⁴ − T_k⁴)` with the view factors of the geometry. */
export function radiationSplit(s: RegionSetup, env: Environment, tsC: number): RadiationSplit {
  const f = viewFactors(s.region.geometry, s.region.skyViewFactor);
  const t = radiantTemperatures(env);
  const part = (view: number, tC: number): number => s.faces * s.emissivity * SIGMA * view * (kelvin(tsC) ** 4 - kelvin(tC) ** 4);
  return { sky: part(f.sky, t.sky), ground: part(f.ground, t.ground), surroundings: part(f.surroundings, t.surroundings) };
}

/** 🧱️ The emitted flux of a frame's surface, W/m². */
export const emittedOf = (f: RegionFrame, tsC: number): number => emittedFlux(f.setup.emissivity, tsC);
