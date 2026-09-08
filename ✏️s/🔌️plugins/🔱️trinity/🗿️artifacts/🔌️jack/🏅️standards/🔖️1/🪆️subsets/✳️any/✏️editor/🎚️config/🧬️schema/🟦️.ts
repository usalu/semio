/** 🧬️ JackConfig */
export interface JackConfig {
  /** @state config */
  camera: Camera;
  /** @state config */
  jackQuery: string;
  /** @state config */
  jackResultJson: string;
  /** @state config */
  editorSelection?: JackEditorSelection;
  /** @state config */
  lodModeByWindow: Record<string, string>;
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
