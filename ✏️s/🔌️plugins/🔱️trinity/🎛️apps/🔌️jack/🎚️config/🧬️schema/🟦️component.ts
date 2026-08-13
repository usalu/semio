/** 🧬️ JackConfig */
export interface JackConfig {
  /** @state config */
  selectedNodeIds: string[];
  /** @state config */
  camera: Camera;
  /** @state config */
  activeFixtureId: string;
  /** @state config */
  jackQuery: string;
  /** @state config */
  jackResultJson: string;
  /** @state config */
  editorEngagementInput: string;
  /** @state config */
  graphEngagementInput: string;
  /** @state config */
  resultsEngagementInput: string;
  /** @state config */
  reorganizeEpoch: number;
  /** @state config */
  editorSelection?: JackEditorSelection;
  /** @state config */
  lodModeByWindow: Record<string, string>;
  /** @state config */
  revision: number;
  /** @state config */
  locale: string;
}

export interface JackEditorSelection {
  start: number;
  end: number;
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}
