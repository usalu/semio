/** 🧬️ RewritePresence */
export interface RewritePresence {
  /** @state shared-ui */
  selectedNodeIds: string[];
  /** @state shared-ui */
  activeHoverVar: string;
  /** @state shared-ui */
  activeSelectVar: string;
  /** @state shared-ui */
  beforePaneCamera: Camera;
  /** @state shared-ui */
  lodModeByWindow: Record<string, string>;
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}
