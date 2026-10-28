/** 🧬️ EnergyModel snapshot schema — artifact-lane fields only. */
import type {EnergyModel} from "./⚡️model/🟦️.ts";
import type { ArtifactLink } from "@semio-tech/framework/schema/artifact-reference";
export * from "./⚡️model/🟦️.ts";

export interface EnergyModelSnapshot {
  /** 📐️ @state artifact */
  schema: string;
  /** 🏛️ @state artifact */
  model: EnergyModel;
  /** 📎️ @state artifact @link model */
  referencedModel: ArtifactLink|null;
  /** 🌬️ @state artifact @link weather */
  weatherLink: ArtifactLink|null;
}
