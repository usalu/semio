/** 🧊 IFC 2x3 Cv20 editor — Main window: typed twin of
 * `🦀️component.rs`'s view-model. Editable mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface Ifc2x3Cv20EditInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface Ifc2x3Cv20EditViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: Ifc2x3Cv20EditInstance[];
}

export const IFC2X3_CV20_EDIT_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const IFC2X3_CV20_EDIT_BODY_KEY = "framework.window.mesh" as const;
