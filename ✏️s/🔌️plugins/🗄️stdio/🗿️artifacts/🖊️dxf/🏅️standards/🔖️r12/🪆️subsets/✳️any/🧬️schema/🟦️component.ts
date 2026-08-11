/** 🧬️ DxfArtifact schema facet — full artifact state, mirrors `🧬️schema/🦀️component.rs`
 * (same persisted fields as `DxfSnapshot`, see `📸️snapshot/🟦️component.ts`). */

import type { DxfBlock, DxfEntity, DxfHeaderVar, DxfOtherTable, DxfTables } from './📸️snapshot/🟦️component.ts';

export interface DxfArtifact {
  schema: string;
  headerVars: DxfHeaderVar[];
  tables: DxfTables;
  otherTables: DxfOtherTable[];
  blocks: DxfBlock[];
  entities: DxfEntity[];
}
