/** 🧊️ Energy model editor — `model` window: typed twin of `🦀️.rs`'s World3d payload. Mirrors the
 * shapes the react `World3dHost` parses out of the scene's `meshes`/`instances` lanes, so a host-side
 * consumer never has to re-derive them from the Rust builder (`crate::scene`). */

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

/** 🪪️ One pickable instance. `id` is the RAW entity id — the same vocabulary the artifact tree panel
 * and the inspector address, which is what makes tree pick ⇄ 3d pick work through the framework's
 * reserved `interactionSelect`/`interactionHover`. */
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

export type EnergyModelSceneObjectKind = "surface" | "fenestration" | "shading";

/** ✏️ The `model` window's typed view-model — the TS mirror of the Rust `render()` boundary's output. */
export interface EnergyModelWorldViewModel {
  windowKindId: "energy.model.3d";
  bodyKey: "energy.model.3d";
  domainId: "energyModel";
  domainGranularityId: "surface";
  meshes: EnergyModelSceneMesh[];
  instances: EnergyModelSceneInstance[];
  /** 🏷️ The results legend drawn above the viewport, absent for the plain model view. */
  caption?: string;
}

export const ENERGY_MODEL_WORLD_WINDOW_KIND_ID = "energy.model.3d" as const;
export const ENERGY_MODEL_WORLD_BODY_KEY = "energy.model.3d" as const;
/** 🪟️ Metres the glazing is lifted along its host surface's outward normal to avoid z-fighting. */
export const ENERGY_MODEL_WINDOW_OFFSET_M = 0.005 as const;
