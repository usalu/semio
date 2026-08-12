/** ↩️ inverse for `ReplacePrimitiveGeometry` — undoes to another `ReplacePrimitiveGeometry` restoring the prior buffers. */
export interface ReplacePrimitiveGeometryInverseReplacePrimitiveGeometry {
  meshId: string;
  primitiveId: string;
  positions: import("../../../📸️snapshot/🟦️component.ts").SemioPoint3[];
  normals: import("../../../📸️snapshot/🟦️component.ts").SemioPoint3[];
  uvs: import("../../../📸️snapshot/🟦️component.ts").SemioUv[];
  colors: import("../../../📸️snapshot/🟦️component.ts").SemioRgba[];
  indices: number[];
}
