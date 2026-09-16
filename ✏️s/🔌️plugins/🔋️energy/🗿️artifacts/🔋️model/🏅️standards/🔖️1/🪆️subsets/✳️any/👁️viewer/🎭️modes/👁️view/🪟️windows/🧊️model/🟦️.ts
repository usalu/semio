/** 🧊️ Energy model viewer — `model` window: read-only typed twin of `🦀️.rs`'s World3d payload. Same
 * geometry shapes as the editor's window, minus the picking vocabulary — a viewer binds no
 * interaction domain, so no instance ever reports a selection back. */

/** 🔺️ One data mesh: flat per-triangle-corner arrays, three floats per corner. */
export interface EnergyModelSceneMeshData {
  positions: number[];
  normals: number[];
  colors: number[];
  indices: number[];
}

export interface EnergyModelSceneMesh {
  id: string;
  data: EnergyModelSceneMeshData;
}

export type EnergyModelSceneObjectKind = "surface" | "fenestration" | "shading";

/** 👁️ One rendered instance. `id` is the raw entity id; `selected`/`hovered` are always `false` on a
 * read-only surface. */
export interface EnergyModelSceneInstance {
  id: string;
  meshId: string;
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  objectKind: EnergyModelSceneObjectKind;
  selected: boolean;
  hovered: boolean;
}

/** 👁️ The `model` window's typed view-model — the TS mirror of the Rust `render()` boundary's output. */
export interface EnergyModelWorldViewModel {
  windowKindId: "energy.model.3d";
  bodyKey: "energy.model.3d";
  meshes: EnergyModelSceneMesh[];
  instances: EnergyModelSceneInstance[];
}

export const ENERGY_MODEL_WORLD_WINDOW_KIND_ID = "energy.model.3d" as const;
export const ENERGY_MODEL_WORLD_BODY_KEY = "energy.model.3d" as const;
