/** ↩️ inverse for `MoveVertex` — undoes to another `MoveVertex` restoring the prior point. */
export interface MoveVertexInverseMoveVertex {
  vertexId: string;
  newPoint: { x: number; y: number; z: number };
}
