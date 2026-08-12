/** ↩️ inverse for `DeleteNode` — always `CreateNode`. */
export interface DeleteNodeInverseCreateNode {
  parent: { layer: number; path: number[] };
  index: number;
  node: unknown;
}
