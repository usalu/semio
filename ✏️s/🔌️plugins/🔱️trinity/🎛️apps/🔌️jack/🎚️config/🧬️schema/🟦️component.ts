/** 🧬️ JackConfig */
export interface JackConfig {
  /** @state local-ui */
  selectedNodeIds: string[];
  /** @state local-ui */
  camera: Camera;
  /** @state local-ui */
  activeFixtureId: string;
  /** @state local-ui */
  jackQuery: string;
  /** @state local-ui */
  jackResultJson: string;
  /** @state local-ui */
  editorEngagementInput: string;
  /** @state local-ui */
  graphEngagementInput: string;
  /** @state local-ui */
  resultsEngagementInput: string;
  /** @state local-ui */
  reorganizeEpoch: number;
  /** @state local-ui */
  editorSelection?: JackEditorSelection;
  /** @state local-ui */
  lodModeByWindow: Record<string, string>;
  /** @state local-ui */
  revision: number;
  /** @state local-ui */
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
