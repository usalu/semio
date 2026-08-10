/** 🧬️ EnergyModel diff schema — sparse field delta. */

export interface EnergyModelDiff {
  /** @state persistent */
  artifact?: EnergyModelArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  modelJson?: string;
  /** @state preview */
  resultsJson?: string;
}

export interface EnergyModelArtifact {
  schema: string;
  modelJson: string;
  resultsJson: string;
}
