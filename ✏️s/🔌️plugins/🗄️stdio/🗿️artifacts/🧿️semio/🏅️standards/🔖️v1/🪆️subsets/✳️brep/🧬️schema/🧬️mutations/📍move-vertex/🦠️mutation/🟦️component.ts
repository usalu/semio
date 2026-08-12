/** mutation payload — mirrors `MoveVertex`. */
export interface MoveVertex {
  vertexId: string;
  newPoint: { x: number; y: number; z: number };
}
