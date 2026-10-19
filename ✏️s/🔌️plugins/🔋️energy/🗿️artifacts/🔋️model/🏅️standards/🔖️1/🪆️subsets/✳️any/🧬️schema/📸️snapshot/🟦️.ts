/** 🧬️ EnergyModel snapshot schema — artifact-lane fields only. */
import type {EnergyModel} from "./⚡️model/🟦️.ts";
import type { ArtifactRef, ArtifactLink } from "@semio-tech/framework/schema/artifact-reference";
export * from "./⚡️model/🟦️.ts";

export interface EnergyModelSnapshot {
  /** 📐️ @state artifact */
  schema: string;
  /** 🏛️ @state artifact */
  model: EnergyModel;
  /** 🔩️ @state artifact @child s.stdio.semio.value */
  structure: { childId: string; target: ArtifactRef };
  /** 🛋️ @state artifact @child s.stdio.semio.table */
  zones: { childId: string; target: ArtifactRef };
  /** 📎️ @state artifact @link model */
  referencedModel: ArtifactLink|null;
  /** 🌬️ @state artifact @link weather */
  weatherLink: ArtifactLink|null;
}
