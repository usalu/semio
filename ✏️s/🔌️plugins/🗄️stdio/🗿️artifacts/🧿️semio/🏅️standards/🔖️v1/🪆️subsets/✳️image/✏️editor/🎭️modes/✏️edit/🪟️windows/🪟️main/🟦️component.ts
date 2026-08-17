/** 🧊 Semio Image editor — Main window: typed twin of
 * `🦀️component.rs`'s view-model. Editable mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface SemioImageEditInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface SemioImageEditViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: SemioImageEditInstance[];
}

export const SEMIO_IMAGE_EDIT_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const SEMIO_IMAGE_EDIT_BODY_KEY = "framework.window.mesh" as const;
