import { parseViewport3dOrbit, type Viewport3dOrbit } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🟦️.ts";

/** 🧬️ Remodeling model-window layer visibility. */
export interface RemodelingLayerVisibility {
  mesh: boolean;
  dense: boolean;
  sparse: boolean;
  cameras: boolean;
  gcps: boolean;
}

/** 🧬️ Exact Remodeling Model window configuration. */
export interface RemodelingModelWindowConfig {
  camera: Viewport3dOrbit;
  layers: RemodelingLayerVisibility;
}

/** 🧬️ Atomic replacement of one exact Model window configuration. */
export type RemodelingModelWindowConfigMutation = { kind: "snapshot"; config: RemodelingModelWindowConfig };

/** 🧬️ Parses one exact Remodeling Model-window configuration. */
export function parseRemodelingModelWindowConfig(value: unknown): RemodelingModelWindowConfig {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("Remodeling Model window config must be an object");
  const row = value as Record<string, unknown>;
  for (const key of Object.keys(row)) if (key !== "camera" && key !== "layers") throw new TypeError(`Remodeling Model window config.${key} is unknown`);
  if (row.layers === null || typeof row.layers !== "object" || Array.isArray(row.layers)) throw new TypeError("Remodeling Model window layers must be an object");
  const layers = row.layers as Record<string, unknown>;
  const layerKeys = ["mesh", "dense", "sparse", "cameras", "gcps"] as const;
  for (const key of Object.keys(layers)) if (!layerKeys.includes(key as typeof layerKeys[number])) throw new TypeError(`Remodeling Model window layers.${key} is unknown`);
  for (const key of layerKeys) if (typeof layers[key] !== "boolean") throw new TypeError(`Remodeling Model window layers.${key} must be boolean`);
  return { camera: parseViewport3dOrbit(row.camera), layers: layers as unknown as RemodelingLayerVisibility };
}

/** 🔁️ Applies one Model window configuration mutation. */
export const applyRemodelingModelWindowConfigMutation = (_base: RemodelingModelWindowConfig, mutation: RemodelingModelWindowConfigMutation): RemodelingModelWindowConfig => structuredClone(mutation.config);
