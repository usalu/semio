/** 🧬️ S Home artifact schema — every field with its state class. */

export interface SHomeArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  catalogGeneration: number;
  /** @state local-ui */
  activePanelTab: string;
  /** @state local-ui */
  locale: string;
}
