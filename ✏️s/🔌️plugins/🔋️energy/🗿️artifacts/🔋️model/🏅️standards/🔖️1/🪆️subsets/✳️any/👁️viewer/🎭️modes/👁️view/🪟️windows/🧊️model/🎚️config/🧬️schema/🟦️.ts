export interface EnergyModelViewerCameraPose { position: [number, number, number]; target: [number, number, number]; zoom: number }
export interface EnergyModelViewerWindowConfig { camera: EnergyModelViewerCameraPose }

function triple(value: unknown, field: string): [number, number, number] {
  if (!Array.isArray(value) || value.length !== 3 || value.some((entry) => typeof entry !== "number" || !Number.isFinite(entry))) throw new TypeError(`camera.${field} must be three finite numbers`);
  return value as [number, number, number];
}

export function parseEnergyModelViewerWindowConfig(value: unknown): EnergyModelViewerWindowConfig {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new TypeError("EnergyModelViewerWindowConfig must be an object");
  const camera = (value as Record<string, unknown>).camera;
  if (typeof camera !== "object" || camera === null || Array.isArray(camera)) throw new TypeError("EnergyModelViewerWindowConfig requires camera");
  const row = camera as Record<string, unknown>;
  triple(row.position, "position");
  triple(row.target, "target");
  if (typeof row.zoom !== "number" || !Number.isFinite(row.zoom) || row.zoom <= 0) throw new TypeError("camera.zoom must be a positive finite number");
  return value as EnergyModelViewerWindowConfig;
}
