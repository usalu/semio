/** 🧬️ DxfArtifact schema facet — full artifact state, mirrors `🧬️schema/🦀️component.rs`
 * (same persisted fields as `DxfSnapshot`, see `📸️snapshot/🟦️.ts`). */

import type { DxfBlock, DxfEntity, DxfHeaderVar, DxfOtherTable, DxfTables } from './📸️snapshot/🟦️.ts';

export interface DxfArtifact {
  schema: string;
  headerVars: DxfHeaderVar[];
  tables: DxfTables;
  otherTables: DxfOtherTable[];
  blocks: DxfBlock[];
  entities: DxfEntity[];
}
