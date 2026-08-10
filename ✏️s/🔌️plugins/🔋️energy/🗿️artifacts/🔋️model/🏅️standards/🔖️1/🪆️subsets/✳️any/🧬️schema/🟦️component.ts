/** 🧬️ EnergyModel artifact schema — every field with its state class. */

export interface EnergyModelArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  modelJson: string;
  /** @state preview */
  resultsJson: string;
}
