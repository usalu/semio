/** 🧬️ JackConfig */
export interface JackConfig {
  /** @state config */
  camera: Camera;
  /** @state config */
  jackQuery: string;
  /** @state config */
  lodModeByWindow: Record<string, string>;
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}
