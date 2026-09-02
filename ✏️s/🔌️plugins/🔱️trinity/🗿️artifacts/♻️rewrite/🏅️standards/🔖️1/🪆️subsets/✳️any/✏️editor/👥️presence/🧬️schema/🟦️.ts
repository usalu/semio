/** 🧬️ RewritePresence */
export interface RewritePresence {
  /** @state presence */
  beforePaneCamera: Camera;
  /** @state presence */
  lodModeByWindow: Record<string, string>;
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}
