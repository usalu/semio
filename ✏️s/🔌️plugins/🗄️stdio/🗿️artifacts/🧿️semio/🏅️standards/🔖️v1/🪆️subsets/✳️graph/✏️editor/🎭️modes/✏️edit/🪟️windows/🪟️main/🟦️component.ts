/** 🧊 Semio Graph editor — Main window: typed twin of
 * `🦀️component.rs`'s view-model. Editable mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface SemioGraphEditInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface SemioGraphEditViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: SemioGraphEditInstance[];
}

export const SEMIO_GRAPH_EDIT_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const SEMIO_GRAPH_EDIT_BODY_KEY = "framework.window.mesh" as const;
