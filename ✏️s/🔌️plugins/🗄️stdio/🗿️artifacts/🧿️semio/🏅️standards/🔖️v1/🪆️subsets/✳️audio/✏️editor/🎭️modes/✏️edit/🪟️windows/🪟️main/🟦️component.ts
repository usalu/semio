/** 🧊 Semio Audio editor — Main window: typed twin of
 * `🦀️component.rs`'s view-model. Editable mirror of the shared `framework.window.mesh` scene payload
 * `render()` produces. */

export interface SemioAudioEditInstance {
  id: string;
  meshId: "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
  smoothShading: boolean;
}

export interface SemioAudioEditViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: SemioAudioEditInstance[];
}

export const SEMIO_AUDIO_EDIT_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const SEMIO_AUDIO_EDIT_BODY_KEY = "framework.window.mesh" as const;
