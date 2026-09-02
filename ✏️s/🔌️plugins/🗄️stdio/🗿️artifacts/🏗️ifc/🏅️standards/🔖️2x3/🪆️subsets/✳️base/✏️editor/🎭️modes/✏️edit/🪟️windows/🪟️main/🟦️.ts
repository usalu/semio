/** 🧊 IFC 2x3 Any editor — Main window: typed twin of
 * `🦀️.rs`'s view-model. Editable mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface Ifc2x3AnyEditInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface Ifc2x3AnyEditViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: Ifc2x3AnyEditInstance[];
}

export const IFC2X3_ANY_EDIT_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const IFC2X3_ANY_EDIT_BODY_KEY = "framework.window.mesh" as const;
