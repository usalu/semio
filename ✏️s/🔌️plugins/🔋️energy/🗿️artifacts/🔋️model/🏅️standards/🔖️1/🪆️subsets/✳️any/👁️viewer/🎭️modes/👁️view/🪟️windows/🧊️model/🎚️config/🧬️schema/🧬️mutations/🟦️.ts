import type { EnergyModelViewerCameraPose, EnergyModelViewerWindowConfig } from "../🟦️";

export type EnergyModelViewerWindowConfigMutation = { kind: "set-camera"; camera: EnergyModelViewerCameraPose };

export function applyEnergyModelViewerWindowConfigMutation(_state: EnergyModelViewerWindowConfig, mutation: EnergyModelViewerWindowConfigMutation): EnergyModelViewerWindowConfig {
  return { camera: mutation.camera };
}
