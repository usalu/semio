/** 🏊️ Sourcing viewer — the pool window: a read-only table of the full stock catalogue, built on the
 *  framework `TableWindowKit` (contract §2.6). Typed twin of the Rust `view_model(document) -> TableView`
 *  boundary (`👁️viewer/🎭️modes/👁️view/🪟️windows/🏊️pool/🦀️.rs`).
 */

export const windowKindId = "framework.window.table";
export const bodyKey = "framework.window.table";

/** 🧱️ Plain string cells — the `TableWindowKit` view-model shape, not the editor's typed `TableCell`. */
export interface PoolTableView {
  columns: string[];
  rows: string[][];
}
