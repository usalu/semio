/** 🧊 DWG AC1018 editor — Main window: typed twin of
 * `🦀️.rs`'s view-model. Editable mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface DwgAc1018EditInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface DwgAc1018EditViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: DwgAc1018EditInstance[];
}

export const DWG_AC1018_EDIT_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const DWG_AC1018_EDIT_BODY_KEY = "framework.window.mesh" as const;
