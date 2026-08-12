/** mutation payload — mirrors `MoveNode`. No-op on a `Path` node (no origin field). */
export interface MoveNode {
  at: { layer: number; path: number[] };
  newOrigin: { x: number; y: number };
}
