/** 🏊️ Sourcing curate app — the pool window: the full stock catalogue with filter chrome + drag source.
 *  Typed twin of the Rust `render(document: &CurateSnapshot, cfg: &SourcingCurateConfig, labels) -> UiNode`
 *  boundary (`🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs`).
 */

export const windowKindId = "sourcing-pool";
export const bodyKey = "sourcing.pool";
export const surfaceId = "sourcing.pool.table";

/** 🔍️ Mirrors the Rust `Filters` record — the pool table's active filter/search/sort state. */
export interface PoolFilters {
  query: string;
  moduleIds: string[];
  typologyPath: string[];
  minAvailability: number;
  sort?: { columnId: string; direction: "asc" | "desc" } | null;
}

/** 🧱️ One pool table row — mirrors `build_pool_table`'s per-`ObjectKind` `TableCell` columns. */
export interface PoolRow {
  id: string;
  name: string;
  module: string;
  typology: string;
  availability: number;
  /** Current curated count for this object kind — rendered as a stepper cell (0..=availability). */
  curated: number;
}

/** 🪟️ The pool window's typed view model. */
export interface PoolViewModel {
  filters: PoolFilters;
  rows: PoolRow[];
}
