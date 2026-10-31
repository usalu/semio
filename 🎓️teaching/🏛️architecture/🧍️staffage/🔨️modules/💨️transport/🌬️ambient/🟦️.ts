/** 🌬️ The ambient air of the transport model along the absolute timeline: environment (constant or keyed timeline), wind profile, turbulence and the moist-air state the parcels mix with.
 *
 * Wind direction is the direction the air moves towards, CCW from +x in the elevation plane. Wind speed is the speed above the boundary layer; with a log profile
 * `u(y) = u·ln(max(y, z0)/z0)/ln(zRef/z0)` it is reached at `zRef` (neutral surface layer, Stull 1988), otherwise it is uniform.
 *
 * @see ../../../🧬️schema/🟦️.ts — `Environment`
 * @see ../../🔗️contracts/🟦️.ts — `EnvironmentTimeline`
 * @see https://doi.org/10.1007/978-94-009-3027-8 — Stull, An Introduction to Boundary Layer Meteorology
 */
import type { Environment } from "../../../🧬️schema/🟦️.ts";
import type { EnvironmentTimeline, Point } from "../../🔗️contracts/🟦️.ts";
import { enthalpy, humidityRatio, saturationPressurePa, volumePerDryKg } from "../🌫️air/🟦️.ts";

/** 🌬️ A log boundary layer: aerodynamic roughness length and the height at which the environment's wind speed is reached. */
export type BoundaryLayer = { readonly z0M: number; readonly referenceHeightM: number };

/** 🌪️ Ambient turbulence: intensity (σ_u / wind speed), floor of σ_u (m/s) and Lagrangian integral time (s); the eddy diffusivity is `σ_u²·T_L`. */
export type Turbulence = { readonly intensity: number; readonly floorMS: number; readonly lagrangianTimeS: number };

/** 🌬️ The ambient state at one instant. */
export type Ambient = {
  readonly environment: Environment;
  readonly tC: number;
  readonly wv: number;
  readonly h: number;
  readonly v: number;
  readonly pressurePa: number;
  readonly sigmaMS: number;
  readonly windAt: (heightM: number) => Point;
};

const NUMERIC = ["airTempC", "meanRadiantTempC", "relativeHumidity", "solarWM2", "diffuseFraction", "pressurePa", "skyTempC", "groundTempC"] as const;

function blend(a: Environment, b: Environment, f: number): Environment {
  const mix = (x: number, y: number): number => x + (y - x) * f;
  const rad = (d: number): number => (d * Math.PI) / 180;
  const wx = mix(a.windMS * Math.cos(rad(a.windDirectionDeg)), b.windMS * Math.cos(rad(b.windDirectionDeg)));
  const wy = mix(a.windMS * Math.sin(rad(a.windDirectionDeg)), b.windMS * Math.sin(rad(b.windDirectionDeg)));
  const out: Record<string, number> = {};
  for (const key of NUMERIC) {
    const x = a[key];
    const y = b[key];
    if (x !== undefined && y !== undefined) out[key] = mix(x, y);
  }
  const speed = Math.hypot(wx, wy);
  return { ...a, ...out, windMS: speed, windDirectionDeg: speed === 0 ? a.windDirectionDeg : (Math.atan2(wy, wx) * 180) / Math.PI };
}

/** 🌤️ The environment at an absolute time: before the first key the first key; a `step` key holds until the next; a `linear` key blends numbers (wind as a vector) to the next key. */
export function environmentAt(source: Environment | EnvironmentTimeline, timeS: number): Environment {
  if (!("keys" in source)) return source;
  const keys = source.keys;
  if (keys.length === 0) throw new Error("an environment timeline needs at least one key");
  if (timeS <= keys[0]!.t) return keys[0]!.environment;
  let i = 0;
  while (i + 1 < keys.length && keys[i + 1]!.t <= timeS) i++;
  const key = keys[i]!;
  const next = keys[i + 1];
  if (next === undefined || key.ease === "step") return key.environment;
  return blend(key.environment, next.environment, (timeS - key.t) / (next.t - key.t));
}

/** 🌬️ The ambient air at an absolute time under the boundary layer and turbulence settings. */
export function ambientAt(source: Environment | EnvironmentTimeline, timeS: number, layer: BoundaryLayer | null, turbulence: Turbulence): Ambient {
  const environment = environmentAt(source, timeS);
  const wv = humidityRatio(environment.relativeHumidity * saturationPressurePa(environment.airTempC), environment.pressurePa);
  const phi = (environment.windDirectionDeg * Math.PI) / 180;
  const ux = environment.windMS * Math.cos(phi);
  const uy = environment.windMS * Math.sin(phi);
  const gain = (y: number): number => (layer === null ? 1 : Math.log(Math.max(y, layer.z0M) / layer.z0M) / Math.log(layer.referenceHeightM / layer.z0M));
  return {
    environment,
    tC: environment.airTempC,
    wv,
    h: enthalpy(environment.airTempC, wv, 0),
    v: volumePerDryKg(environment.airTempC, wv, environment.pressurePa),
    pressurePa: environment.pressurePa,
    sigmaMS: Math.max(turbulence.intensity * environment.windMS, turbulence.floorMS),
    windAt: (heightM) => {
      const g = gain(heightM);
      return { x: ux * g, y: uy * g };
    },
  };
}
