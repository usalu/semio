/** 🔺️ Sparse typed Energy field changes retain the snapshot's exact owned domains. */
import type {EnergyModelArtifact} from "../🟦️.ts";
import type {EnergyModelSnapshot} from "../📸️snapshot/🟦️.ts";

export type EnergyLinkSlotDelta = {readonly kind:"detached"}|{readonly kind:"attached";readonly link:NonNullable<EnergyModelSnapshot["referencedModel"]>};

export interface EnergyModelDiff {
  artifact?: EnergyModelArtifact;
  schema?: string;
  model?: EnergyModelSnapshot["model"];
  structure?: EnergyModelSnapshot["structure"];
  zones?: EnergyModelSnapshot["zones"];
  referencedModel?: EnergyLinkSlotDelta;
  weatherLink?: EnergyLinkSlotDelta;
  resultsJson?: string;
}
