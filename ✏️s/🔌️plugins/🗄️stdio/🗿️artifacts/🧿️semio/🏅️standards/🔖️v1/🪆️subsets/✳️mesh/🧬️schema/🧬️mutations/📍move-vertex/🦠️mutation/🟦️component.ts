/** mutation payload — mirrors `MoveVertex`. */
export interface MoveVertex {
  meshId: string;
  primitiveId: string;
  vertexIndex: number;
  newPoint: import("../../../📸️snapshot/🟦️component.ts").SemioPoint3;
}
