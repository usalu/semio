/** 🧬️ S Home diff schema — sparse field delta over the artifact. */

import type { SHomeArtifact } from "../../🧬️schema/🟦️component.ts";

export interface SHomeDiff {
  /** @state persistent */
  artifact?: SHomeArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  catalogGeneration?: number;
  /** @state local-ui */
  activePanelTab?: string;
  /** @state local-ui */
  locale?: string;
}
