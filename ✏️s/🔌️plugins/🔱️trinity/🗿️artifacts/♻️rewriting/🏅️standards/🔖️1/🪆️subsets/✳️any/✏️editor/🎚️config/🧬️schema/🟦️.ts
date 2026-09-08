/** 🧬️ RewritingConfig */
export interface RewritingConfig {
  /** @state config */
  beforePaneCamera: Camera;
  /** @state config */
  /** @state config */
  lodModeByWindow: Record<string, string>;
  /** @state config */
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}
