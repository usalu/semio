/** mutation payload — mirrors `ReplacePrimitiveGeometry`. */
export interface ReplacePrimitiveGeometry {
  meshId: string;
  primitiveId: string;
  positions: import("../../../📸️snapshot/🟦️component.ts").SemioPoint3[];
  normals: import("../../../📸️snapshot/🟦️component.ts").SemioPoint3[];
  uvs: import("../../../📸️snapshot/🟦️component.ts").SemioUv[];
  colors: import("../../../📸️snapshot/🟦️component.ts").SemioRgba[];
  indices: number[];
}
