/** 🧬️ SemioCadArtifact schema — full artifact state, mirrors `SemioCadSnapshot` field for field
 * (see `📸️snapshot/🟦️component.ts` for the nested `CadLayer`/`CadBlock`/`CadEntityRecord` shapes). */
import type { CadLayer, CadBlock, CadEntityRecord } from "./📸️snapshot/🟦️component";

export interface SemioCadArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ layers: CadLayer[];
  /** @state persistent */ blocks: CadBlock[];
  /** @state persistent */ entities: CadEntityRecord[];
}
