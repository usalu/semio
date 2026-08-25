/** 🧬️ EnergyModel artifact schema — every field with its state class. */

export interface EnergyModelArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  model: unknown;
  /** @state artifact @child s.stdio.semio.value */
  structure: { childId: string; target: string };
  /** @state artifact @child s.stdio.semio.table */
  zones: { childId: string; target: string };
  /** @state artifact @link model */
  referencedModel?: unknown;
  /** @state artifact */
  resultsJson: string;
}
