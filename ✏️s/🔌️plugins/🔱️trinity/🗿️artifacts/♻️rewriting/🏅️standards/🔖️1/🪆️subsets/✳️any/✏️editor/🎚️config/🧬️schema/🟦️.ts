/** 🧬️ RewritingConfig */
export interface RewritingConfig {
  /** @state config */
  beforePaneCamera: Camera;
  /** @state config */
  reorganizeEpoch: number;
  /** @state config */
  lodModeByWindow: Record<string, string>;
  /** @state config */
  locale: string;
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}
