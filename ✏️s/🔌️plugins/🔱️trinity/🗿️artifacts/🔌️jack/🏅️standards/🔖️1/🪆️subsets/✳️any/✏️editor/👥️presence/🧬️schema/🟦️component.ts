/** 🧬️ JackPresence */
export interface JackPresence {
  /** @state presence */
  activeFixtureId: string;
  /** @state presence */
  jackQuery: string;
  /** @state presence */
  camera: Camera;
  /** @state presence */
  lodModeByWindow: Record<string, string>;
}

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}
