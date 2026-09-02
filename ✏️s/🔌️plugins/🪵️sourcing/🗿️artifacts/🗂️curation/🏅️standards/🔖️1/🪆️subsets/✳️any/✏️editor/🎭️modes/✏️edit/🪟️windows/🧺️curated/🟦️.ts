/** 🧺️ Sourcing curation app — the curated window: the currently-picked objects and their counts.
 *  Typed twin of the Rust `render(document: &CurationSnapshot, labels) -> UiNode` boundary
 *  (`🎭️modes/✏️edit/🪟️windows/🧺️curated/🦀️.rs`).
 */

export const windowKindId = "sourcing-curated";
export const bodyKey = "sourcing.curated";
export const surfaceId = "sourcing.curated.table";

/** 🧱️ One curated table row — mirrors `render`'s per-`CuratedItem` `TableCell` columns (name/
 *  availability/count-stepper/remove-button), joined against its matching `ObjectKind`.
 */
export interface CuratedRow {
  objectId: string;
  name: string;
  availability: number;
  count: number;
}

/** 🪟️ The curated window's typed view model. */
export interface CuratedViewModel {
  rows: CuratedRow[];
}
