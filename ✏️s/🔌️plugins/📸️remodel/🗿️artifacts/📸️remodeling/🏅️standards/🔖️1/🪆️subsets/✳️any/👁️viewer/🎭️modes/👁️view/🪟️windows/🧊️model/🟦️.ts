/** 🧊️ Remodeling viewer — Model window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * read-only pane's `render(scene: &RemodelingSnapshot)` boundary — a World3d scene carrying the
 * reconstructed mesh, the sparse/dense clouds, the recovered camera positions and the ground
 * control points, all unconditionally visible (a viewer keeps no per-session layer-visibility
 * state, unlike the mutation-capable editor twin at
 * `✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🟦️.ts`, never imported from here). */

/** 🧊️ The one placeholder-or-reconstructed mesh instance — mirrors `world_instances_json`. */
export interface RemodelingViewMeshInstance {
  id: string;
  meshId: string;
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  selected: boolean;
  hovered: boolean;
}

/** ☁️ One `World3dScene.points_json` layer — sparse/dense clouds, camera poses, or GCPs, each
 * unconditionally visible — mirrors `world_points_json`'s four layer shapes. */
export interface RemodelingViewPointLayer {
  id: "remodeling-sparse" | "remodeling-dense" | "remodeling-camera-poses" | "remodeling-gcps";
  positionsB64: string;
  colorsB64: string | null;
  size: number;
  sizeAttenuation: boolean;
}

/** 🧊️ The Model window's typed view-model — mirrors the read-only Rust `render()` boundary's
 * inputs. There is no `layers`/`camera` field: both are hardcoded defaults on the Rust side
 * (`Config = NoConfig`), not session-carried state a host would ever supply. */
export interface RemodelingViewModelViewModel {
  windowKindId: "remodeling-view-model";
  bodyKey: "remodeling.view.model";
  surfaceId: "remodeling.view.scene3d/model";
  meshInstances: RemodelingViewMeshInstance[];
  pointLayers: RemodelingViewPointLayer[];
}

export const REMODELING_VIEW_WINDOW_MODEL = "remodeling-view-model" as const;
export const REMODELING_VIEW_BODY_MODEL = "remodeling.view.model" as const;
