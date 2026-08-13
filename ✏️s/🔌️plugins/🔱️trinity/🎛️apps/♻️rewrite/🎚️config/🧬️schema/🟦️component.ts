/** 🧬️ RewriteConfig */
export interface RewriteConfig {
  /** @state config */
  selectedNodeIds: string[];
  /** @state config */
  beforePaneCamera: Camera;
  /** @state config */
  reorganizeEpoch: number;
  /** @state config */
  activeHoverVar: string;
  /** @state config */
  hoverEpoch: number;
  /** @state config */
  activeSelectVar: string;
  /** @state config */
  selectEpoch: number;
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
