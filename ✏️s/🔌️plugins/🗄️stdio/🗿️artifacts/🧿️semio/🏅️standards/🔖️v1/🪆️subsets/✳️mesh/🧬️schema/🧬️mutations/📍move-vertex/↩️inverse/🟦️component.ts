/** ↩️ inverse for `MoveVertex` — undoes to another `MoveVertex` restoring the prior point. */
export interface MoveVertexInverseMoveVertex {
  meshId: string;
  primitiveId: string;
  vertexIndex: number;
  newPoint: import("../../../📸️snapshot/🟦️component.ts").SemioPoint3;
}
