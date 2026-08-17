/** 🧊 Step Any editor — Main window: typed twin of
 * `🦀️component.rs`'s view-model. Editable mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface StepAnyEditInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface StepAnyEditViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: StepAnyEditInstance[];
}

export const STEP_ANY_EDIT_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const STEP_ANY_EDIT_BODY_KEY = "framework.window.mesh" as const;
