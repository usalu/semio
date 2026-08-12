/** mutation payload — mirrors `DeleteNode`. `at.path` must be non-empty (a layer root is deleted
 * via `DeleteLayer`, not this mutation). */
export interface DeleteNode {
  at: { layer: number; path: number[] };
}
