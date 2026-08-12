/** mutation payload — mirrors `UngroupNode`. No-op unless `at` resolves to a `Group`. */
export interface UngroupNode {
  at: { layer: number; path: number[] };
}
