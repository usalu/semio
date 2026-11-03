/** 🌬️ The ambient air of the transport model along the absolute timeline: environment (constant or keyed timeline), wind profile, turbulence and the moist-air state the parcels mix with.
 *
 * Wind direction is the horizontal bearing the air moves towards, from +x (ahead) towards +z (the asset's left; the frame is left-handed); the wind has no vertical component, so plan, front and side views read the same 3D vector. Wind speed is the speed above the boundary layer; with a log profile
 * `u(y) = u·ln(max(y, z0)/z0)/ln(zRef/z0)` it is reached at `zRef` (neutral surface layer, Stull 1988), otherwise it is uniform.
 *
 * @see ../../../🧬️schema/🟦️.ts — `Environment`
 * @see ../../🔗️contracts/🟦️.ts — `EnvironmentTimeline`
 * @see https://doi.org/10.1007/978-94-009-3027-8 — Stull, An Introduction to Boundary Layer Meteorology
 */
import type { Environment } from "../../../🧬️schema/🟦️.ts";
import type { EnvironmentTimeline, Vec3 } from "../../🔗️contracts/🟦️.ts";
import { environmentAt } from "../../🌡️thermal/🌤️environment/🟦️.ts";
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
  readonly windAt: (heightM: number) => Vec3;
};

/** 🌬️ The ambient air at an absolute time under the boundary layer and turbulence settings. */
export function ambientAt(source: Environment | EnvironmentTimeline, timeS: number, layer: BoundaryLayer | null, turbulence: Turbulence): Ambient {
  const environment = environmentAt(source, timeS);
  const wv = humidityRatio(environment.relativeHumidity * saturationPressurePa(environment.airTempC), environment.pressurePa);
  const phi = (environment.windDirectionDeg * Math.PI) / 180;
  const ux = environment.windMS * Math.cos(phi);
  const uz = environment.windMS * Math.sin(phi);
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
      return { x: ux * g, y: 0, z: uz * g };
    },
  };
}
