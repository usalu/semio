/** ↩️ inverse for `DragNodes` — always `DragNodes` with the negated offset. */
export interface DragNodesInverseDragNodes {
  ats: { layer: number; path: number[] }[];
  offset: { x: number; y: number };
}
