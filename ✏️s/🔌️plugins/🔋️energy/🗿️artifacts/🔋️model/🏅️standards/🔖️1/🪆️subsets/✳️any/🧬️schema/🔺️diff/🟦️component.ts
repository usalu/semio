/** 🧬️ EnergyModel diff schema — sparse field delta. */

export interface EnergyModelDiff {
  /** @state artifact */
  artifact?: EnergyModelArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  modelJson?: string;
  /** @state artifact */
  resultsJson?: string;
}

export interface EnergyModelArtifact {
  schema: string;
  modelJson: string;
  resultsJson: string;
}
