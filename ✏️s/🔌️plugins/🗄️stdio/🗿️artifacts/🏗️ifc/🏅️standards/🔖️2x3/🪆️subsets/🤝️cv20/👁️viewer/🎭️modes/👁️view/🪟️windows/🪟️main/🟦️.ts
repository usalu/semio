/** 🏗️ Source-derived IFC2x3 mesh identity and geometry payload. */

export interface Ifc2x3Cv20ViewInstance {
  id: string;
  meshId: string;
  interactionId: string;
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface Ifc2x3Cv20ViewViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: Ifc2x3Cv20ViewInstance[];
}

export const IFC2X3_CV20_VIEW_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const IFC2X3_CV20_VIEW_BODY_KEY = "framework.window.mesh" as const;
