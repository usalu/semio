/** mutation payload — mirrors `MoveNode`. */
export interface MoveNode {
  id: { value: string };
  newPosition: { x: number; y: number };
}
