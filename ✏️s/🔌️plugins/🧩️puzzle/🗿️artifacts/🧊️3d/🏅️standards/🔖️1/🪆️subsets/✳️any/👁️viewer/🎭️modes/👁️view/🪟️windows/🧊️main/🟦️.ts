/** 🧊️ Puzzle 3d viewer — Mesh window: typed twin of `🦀️.rs`'s view-model. Read-only mirror
 * of the framework `MeshWindowKit`'s `MeshView` payload — no mutation-shaped fields (no gumball, no
 * brush/fill session, no engagement input), matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One placed object's read-only instance record, as emitted by `puzzle3dViewInstancesJson`. */
export interface Puzzle3dViewInstance {
  id: string;
  meshId: string;
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  disabled: boolean;
}

/** 👁️ The Mesh window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs (a
 * bare `Puzzle3dSnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface Puzzle3dViewMeshViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: Puzzle3dViewInstance[];
}

export const PUZZLE3D_VIEW_MESH_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const PUZZLE3D_VIEW_MESH_BODY_KEY = "framework.window.mesh" as const;
