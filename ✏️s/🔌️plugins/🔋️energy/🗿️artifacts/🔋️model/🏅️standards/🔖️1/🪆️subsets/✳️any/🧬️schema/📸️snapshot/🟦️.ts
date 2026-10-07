/** 🧬️ EnergyModel snapshot schema — artifact-lane fields only. */
import type {EnergyModel} from "./⚡️model/🟦️.ts";
import type { ArtifactRef } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts";
import type {ArtifactLink} from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
export * from "./⚡️model/🟦️.ts";

export interface EnergyModelSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  model: EnergyModel;
  /** @state artifact @child s.stdio.semio.value */
  structure: { childId: string; target: ArtifactRef };
  /** @state artifact @child s.stdio.semio.table */
  zones: { childId: string; target: ArtifactRef };
  /** @state artifact @link model */
  referencedModel: ArtifactLink|null;
  /** @state artifact @link weather */
  weatherLink: ArtifactLink|null;
}
