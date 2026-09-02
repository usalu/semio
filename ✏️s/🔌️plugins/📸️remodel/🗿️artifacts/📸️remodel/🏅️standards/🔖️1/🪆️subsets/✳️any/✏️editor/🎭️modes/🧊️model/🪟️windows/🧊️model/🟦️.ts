/** 🧊️ Remodel editor — Model window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * pane's `render(scene: &RemodelSnapshot, config: &RemodelConfig)` boundary — the World3d scene
 * carrying the reconstructed mesh, the sparse/dense clouds, the recovered camera positions and the
 * ground control points (`world_meshes_json`/`world_instances_json`/`world_points_json`'s own
 * shapes). The viewer's read-only twin (`👁️viewer/…/🧊️model/🟦️.ts`) mirrors this same
 * scene shape minus `layers`/mutation-facing fields — never imports from here. */

/** 🎥️ Ephemeral viewport orbit camera — mirrors Rust `RemodelWorldCamera`. */
export interface RemodelWorldCamera {
  position: [number, number, number];
  target: [number, number, number];
  fov: number;
}

/** 👁️ Which point-cloud/mesh layers are visible — mirrors Rust `RemodelLayerVisibility`. */
export interface RemodelLayerVisibility {
  mesh: boolean;
  dense: boolean;
  sparse: boolean;
  cameras: boolean;
  gcps: boolean;
}

/** 🧊️ The one placeholder-or-reconstructed mesh instance — mirrors `world_instances_json`. */
export interface RemodelMeshInstance {
  id: string;
  meshId: string;
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  selected: boolean;
  hovered: boolean;
}

/** ☁️ One `World3dScene.points_json` layer — sparse/dense clouds, camera poses, or GCPs, each its
 * own base64-packed positions/colors buffer — mirrors `world_points_json`'s four layer shapes. */
export interface RemodelPointLayer {
  id: "remodel-sparse" | "remodel-dense" | "remodel-camera-poses" | "remodel-gcps";
  positionsB64: string;
  colorsB64: string | null;
  size: number;
  sizeAttenuation: boolean;
}

/** 🧊️ The Model window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface RemodelModelViewModel {
  windowKindId: "remodel-main";
  bodyKey: "remodel.play.main";
  surfaceId: "remodel.play";
  camera: RemodelWorldCamera;
  layers: RemodelLayerVisibility;
  meshInstances: RemodelMeshInstance[];
  pointLayers: RemodelPointLayer[];
}

export const REMODEL_PLAY_WINDOW_MAIN = "remodel-main" as const;
export const REMODEL_PLAY_BODY_MAIN = "remodel.play.main" as const;
