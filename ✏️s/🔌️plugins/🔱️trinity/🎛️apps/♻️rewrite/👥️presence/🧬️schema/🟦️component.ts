/** 🧬️ RewritePresence */
export interface RewritePresence {
  /** @state presence */
  selectedNodeIds: string[];
  /** @state presence */
  activeHoverVar: string;
  /** @state presence */
  activeSelectVar: string;
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
