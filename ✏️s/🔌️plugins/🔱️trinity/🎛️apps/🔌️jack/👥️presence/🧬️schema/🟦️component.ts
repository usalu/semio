/** 🧬️ JackPresence */
export interface JackPresence {
  /** @state shared-ui */
  selectedNodeIds: string[];
  /** @state shared-ui */
  activeFixtureId: string;
  /** @state shared-ui */
  jackQuery: string;
  /** @state shared-ui */
  camera: Camera;
  /** @state shared-ui */
  lodModeByWindow: Record<string, string>;
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}
