/** 🧊️ Remodeling editor — Model window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * pane's `render(scene: &RemodelingSnapshot, config: &RemodelingConfig)` boundary — the World3d scene
 * carrying the reconstructed mesh, the sparse/dense clouds, the recovered camera positions and the
 * ground control points (`world_meshes_json`/`world_instances_json`/`world_points_json`'s own
 * shapes). The viewer's read-only twin (`👁️viewer/…/🧊️model/🟦️.ts`) mirrors this same
 * scene shape minus `layers`/mutation-facing fields — never imports from here. */

/** 🎥️ Ephemeral viewport orbit camera — mirrors Rust `RemodelingWorldCamera`. */
export interface RemodelingWorldCamera {
  position: [number, number, number];
  target: [number, number, number];
  fov: number;
}

/** 👁️ Which point-cloud/mesh layers are visible — mirrors Rust `RemodelingLayerVisibility`. */
export interface RemodelingLayerVisibility {
  mesh: boolean;
  dense: boolean;
  sparse: boolean;
  cameras: boolean;
  gcps: boolean;
}

/** 🧊️ The one placeholder-or-reconstructed mesh instance — mirrors `world_instances_json`. */
export interface RemodelingMeshInstance {
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
export interface RemodelingPointLayer {
  id: "remodeling-sparse" | "remodeling-dense" | "remodeling-camera-poses" | "remodeling-gcps";
  positionsB64: string;
  colorsB64: string | null;
  size: number;
  sizeAttenuation: boolean;
}

/** 🧊️ The Model window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface RemodelingModelViewModel {
  windowKindId: "remodeling-main";
  bodyKey: "remodeling.play.main";
  surfaceId: "remodeling.play";
  camera: RemodelingWorldCamera;
  layers: RemodelingLayerVisibility;
  meshInstances: RemodelingMeshInstance[];
  pointLayers: RemodelingPointLayer[];
}

export const REMODELING_PLAY_WINDOW_MAIN = "remodeling-main" as const;
export const REMODELING_PLAY_BODY_MAIN = "remodeling.play.main" as const;
