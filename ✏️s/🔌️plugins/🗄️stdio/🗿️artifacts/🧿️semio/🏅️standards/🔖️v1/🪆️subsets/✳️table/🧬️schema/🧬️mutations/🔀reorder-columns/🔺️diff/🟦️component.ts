/** 🔺️ diff fragment for `ReorderColumns` — identical remove-then-insert applied to columns and
 * every row's cells (alignment invariant). */
export interface ReorderColumnsDiff {
  columns?: unknown[];
  rows?: unknown[];
}
