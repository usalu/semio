/** 🧬️ RewriteConfig */
export interface RewriteConfig {
  /** @state local-ui */
  selectedNodeIds: string[];
  /** @state local-ui */
  beforePaneCamera: Camera;
  /** @state local-ui */
  reorganizeEpoch: number;
  /** @state local-ui */
  activeHoverVar: string;
  /** @state local-ui */
  hoverEpoch: number;
  /** @state local-ui */
  activeSelectVar: string;
  /** @state local-ui */
  selectEpoch: number;
  /** @state local-ui */
  lodModeByWindow: Record<string, string>;
  /** @state local-ui */
  locale: string;
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}
