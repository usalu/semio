/** 🧊 IFC 2x3 Sav viewer — Main window: typed twin of
 * `🦀️component.rs`'s view-model. Read-only mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface Ifc2x3SavViewInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface Ifc2x3SavViewViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: Ifc2x3SavViewInstance[];
}

export const IFC2X3_SAV_VIEW_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const IFC2X3_SAV_VIEW_BODY_KEY = "framework.window.mesh" as const;
