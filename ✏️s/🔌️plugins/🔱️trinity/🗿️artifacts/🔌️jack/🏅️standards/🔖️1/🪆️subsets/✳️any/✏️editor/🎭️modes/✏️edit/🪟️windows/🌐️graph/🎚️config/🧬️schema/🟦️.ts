/** 🪟️ Persisted local view settings for one concrete Jack graph window. */
export interface JackGraphWindowConfig {
  /** 🎥️ @state config */
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number } | null;
  /** 🔬️ @state config */
  readonly lodMode: string;
}
