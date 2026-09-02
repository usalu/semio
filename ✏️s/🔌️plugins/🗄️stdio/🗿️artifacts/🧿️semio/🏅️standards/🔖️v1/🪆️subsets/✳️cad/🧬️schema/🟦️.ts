/** 🧬️ SemioCadArtifact schema — full artifact state, mirrors `SemioCadSnapshot` field for field
 * (see `📸️snapshot/🟦️.ts` for the nested `CadLayer`/`CadBlock`/`CadEntityRecord` shapes). */
import type { CadLayer, CadBlock, CadEntityRecord } from "./📸️snapshot/🟦️component";

export interface SemioCadArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ layers: CadLayer[];
  /** @state artifact */ blocks: CadBlock[];
  /** @state artifact */ entities: CadEntityRecord[];
}
