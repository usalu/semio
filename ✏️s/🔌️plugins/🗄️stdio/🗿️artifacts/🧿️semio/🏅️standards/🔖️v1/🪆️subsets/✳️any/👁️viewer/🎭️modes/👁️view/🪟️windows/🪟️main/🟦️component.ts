/** 🧊 Semio Any viewer — Main window: typed twin of
 * `🦀️component.rs`'s view-model. Read-only mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface SemioAnyViewInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface SemioAnyViewViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: SemioAnyViewInstance[];
}

export const SEMIO_ANY_VIEW_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const SEMIO_ANY_VIEW_BODY_KEY = "framework.window.mesh" as const;
