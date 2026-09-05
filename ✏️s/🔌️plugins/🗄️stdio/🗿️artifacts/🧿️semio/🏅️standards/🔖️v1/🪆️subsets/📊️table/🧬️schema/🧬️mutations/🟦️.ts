/** 🧬️ SemioTableMutation — real facet mirror of the Rust `🦀️.rs` sibling. Closed,
 * eight-variant dispatch. `SemioTableMutation` carries only `#[derive(dsl::Mutations)]` — no
 * `#[serde(tag = ...)]` — so it serializes with serde's default EXTERNALLY TAGGED shape:
 * `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, confirmed by the committed
 * `🔃reorder-rows/🧪️tests/*​/🦠️mutation/🔣️.json` fixture (`{"ReorderRows":
 * {"from":2,"to":0}}`) — NOT the `{ mutation: "...", payload: {...} }` envelope this previously
 * declared (no `payload` wrapper key exists on the wire at all). None of the 8 leaf structs carry
 * `#[serde(rename_all = ...)]` (confirmed by this artifact's own `🦀️.rs` doc comment), so every
 * leaf's own field names are the literal Rust snake_case names verbatim. */
import type { SemioTableCellKind, SemioTableRow, SemioValue } from "../📸️snapshot/🟦️.ts";

export interface CreateColumn {
  name: string;
  kind: SemioTableCellKind;
  index?: number | null;
}

export interface DeleteColumn {
  name: string;
}

export interface RenameColumn {
  name: string;
  new_name: string;
}

export interface ReorderColumns {
  name: string;
  to_index: number;
}

export interface InsertRow {
  index: number;
  row: SemioTableRow;
}

export interface RemoveRow {
  index: number;
}

export interface ReorderRows {
  from: number;
  to: number;
}

export interface EditCell {
  row_index: number;
  column_name: string;
  new_value: SemioValue;
}

export type SemioTableMutation =
  | { CreateColumn: CreateColumn }
  | { DeleteColumn: DeleteColumn }
  | { RenameColumn: RenameColumn }
  | { ReorderColumns: ReorderColumns }
  | { InsertRow: InsertRow }
  | { RemoveRow: RemoveRow }
  | { ReorderRows: ReorderRows }
  | { EditCell: EditCell };
