/** 🧬️ EnergyModel diff schema — sparse field delta. */

/** 🔗️ A link slot delta; an absent field means the slot did not change at all. */
export type EnergyLinkSlotDelta = { readonly kind: "detached" } | { readonly kind: "attached"; readonly link: unknown };

export interface EnergyModelDiff {
  /** @state artifact */
  artifact?: EnergyModelArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  model?: unknown;
  structure?: unknown;
  zones?: unknown;
  referencedModel?: EnergyLinkSlotDelta;
  weatherLink?: EnergyLinkSlotDelta;
  /** @state artifact */
  resultsJson?: string;
}

export interface EnergyModelArtifact {
  schema: string;
  model: unknown;
  structure: unknown;
  zones: unknown;
  referencedModel?: unknown;
  weatherLink?: unknown;
  resultsJson: string;
}
