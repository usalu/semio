/** 🧊 Step CC5 editor — Main window: typed twin of
 * `🦀️.rs`'s view-model. Editable mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface StepCc5EditInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface StepCc5EditViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: StepCc5EditInstance[];
}

export const STEP_CC5_EDIT_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const STEP_CC5_EDIT_BODY_KEY = "framework.window.mesh" as const;
