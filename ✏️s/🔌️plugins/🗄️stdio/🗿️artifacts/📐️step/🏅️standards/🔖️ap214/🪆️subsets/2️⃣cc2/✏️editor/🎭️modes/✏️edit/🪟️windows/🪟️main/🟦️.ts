/** 🧊 Step CC2 editor — Main window: typed twin of
 * `🦀️.rs`'s view-model. Editable mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface StepCc2EditInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface StepCc2EditViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: StepCc2EditInstance[];
}

export const STEP_CC2_EDIT_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const STEP_CC2_EDIT_BODY_KEY = "framework.window.mesh" as const;
