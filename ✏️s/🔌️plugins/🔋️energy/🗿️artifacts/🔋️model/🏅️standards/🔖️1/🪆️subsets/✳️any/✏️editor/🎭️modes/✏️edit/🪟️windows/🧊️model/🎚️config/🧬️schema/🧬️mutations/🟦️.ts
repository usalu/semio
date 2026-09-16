import type { EnergyModelCameraPose, EnergyModelWindowConfig } from "../🟦️";

export type EnergyModelWindowConfigMutation = { kind: "set-camera"; camera: EnergyModelCameraPose };

export function applyEnergyModelWindowConfigMutation(_state: EnergyModelWindowConfig, mutation: EnergyModelWindowConfigMutation): EnergyModelWindowConfig {
  return { camera: mutation.camera };
}
