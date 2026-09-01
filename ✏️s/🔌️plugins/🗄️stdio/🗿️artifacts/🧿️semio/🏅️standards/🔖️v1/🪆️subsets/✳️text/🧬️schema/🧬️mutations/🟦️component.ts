/** 🧬️ SemioTextMutation — real facet mirror of the Rust `🦀️component.rs` sibling. Closed,
 * seven-variant dispatch. `SemioTextMutation` carries only `#[derive(dsl::Mutations)]` — no
 * `#[serde(tag = ...)]` — so it serializes with serde's default EXTERNALLY TAGGED shape:
 * `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, confirmed by the committed
 * `➕add-mark/🧪️tests/*​/🦠️mutation/🔣️component.json` fixture (`{"AddMark":{"run_index":0,
 * "index":0,"mark":{"kind":"link","href":"..."}}}`) — NOT the `{ mutation: "...", payload: {...} }`
 * envelope this previously declared. None of the 7 leaf structs carry
 * `#[serde(rename_all = ...)]` (confirmed by this artifact's own `🦀️.rs` doc comment), so every
 * leaf's own field names are the literal Rust snake_case names verbatim. */
import type { SemioTextRun, SemioTextMark } from "../📸️snapshot/🟦️component.ts";

export interface InsertRun {
  index: number;
  run: SemioTextRun;
}

export interface RemoveRun {
  index: number;
}

export interface EditRun {
  index: number;
  new_content: string;
}

export interface ChangeRunLanguage {
  index: number;
  new_language: string;
}

export interface ReorderRuns {
  from: number;
  to: number;
}

export interface AddMark {
  run_index: number;
  index: number;
  mark: SemioTextMark;
}

export interface RemoveMark {
  run_index: number;
  index: number;
}

export type SemioTextMutation =
  | { InsertRun: InsertRun }
  | { RemoveRun: RemoveRun }
  | { EditRun: EditRun }
  | { ChangeRunLanguage: ChangeRunLanguage }
  | { ReorderRuns: ReorderRuns }
  | { AddMark: AddMark }
  | { RemoveMark: RemoveMark };
