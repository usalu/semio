/** 🧬️ LasArtifact schema. */
import type { LasHeader, LasVlr, LasPoint } from './📸️snapshot/🟦️component.ts';

export interface LasArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ header: LasHeader;
  /** @state persistent */ vlrs: LasVlr[];
  /** @state persistent */ points: LasPoint[];
}
