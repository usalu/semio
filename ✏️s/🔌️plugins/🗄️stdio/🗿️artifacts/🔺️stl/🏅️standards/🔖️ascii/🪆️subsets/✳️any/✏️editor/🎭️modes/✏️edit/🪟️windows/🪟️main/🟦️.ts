/** 🧊 STL editor — Main window: typed twin of
 * `🦀️.rs`'s view-model. Editable mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface StlAnyEditInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface StlAnyEditViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: StlAnyEditInstance[];
}

export const STL_ANY_EDIT_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const STL_ANY_EDIT_BODY_KEY = "framework.window.mesh" as const;
