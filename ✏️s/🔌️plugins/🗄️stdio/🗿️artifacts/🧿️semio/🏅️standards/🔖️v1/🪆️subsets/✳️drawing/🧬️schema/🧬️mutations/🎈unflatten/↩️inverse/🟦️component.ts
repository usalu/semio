/** ↩️ inverse for `UnflattenNode` — always `FlattenNode` at the same address. */
export interface UnflattenNodeInverseFlattenNode {
  at: { layer: number; path: number[] };
}
