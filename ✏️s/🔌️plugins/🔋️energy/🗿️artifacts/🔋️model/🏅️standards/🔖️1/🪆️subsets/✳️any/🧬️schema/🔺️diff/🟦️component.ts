/** 🧬️ EnergyModel diff schema — sparse field delta. */

export interface EnergyModelDiff {
  /** @state artifact */
  artifact?: EnergyModelArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  model?: unknown;
  structure?: unknown;
  zones?: unknown;
  referencedModel?: unknown | null;
  /** @state artifact */
  resultsJson?: string;
}

export interface EnergyModelArtifact {
  schema: string;
  model: unknown;
  structure: unknown;
  zones: unknown;
  referencedModel?: unknown;
  resultsJson: string;
}
