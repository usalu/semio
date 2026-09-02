/** 🧊 LAS viewer — Main window: typed twin of
 * `🦀️.rs`'s view-model. Read-only mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface LasAnyViewInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface LasAnyViewViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: LasAnyViewInstance[];
}

export const LAS_ANY_VIEW_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const LAS_ANY_VIEW_BODY_KEY = "framework.window.mesh" as const;
