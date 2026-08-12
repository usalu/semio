/** ↩️ inverse for `FlattenNode` — always `UnflattenNode` carrying the captured original subtree. */
export interface FlattenNodeInverseUnflattenNode {
  at: { layer: number; path: number[] };
  original: unknown;
}
