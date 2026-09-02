/** 🧬️ S Home artifact schema — every field with its state class. */

export interface SHomeArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  catalogGeneration: number;
  /** @state config */
  activePanelTab: string;
  /** @state config */
  locale: string;
}
