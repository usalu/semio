/** 🧬️ LasArtifact schema. */
import type { LasHeader, LasVlr, LasPoint } from './📸️snapshot/🟦️.ts';

export interface LasArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ header: LasHeader;
  /** @state artifact */ vlrs: LasVlr[];
  /** @state artifact */ points: LasPoint[];
}
