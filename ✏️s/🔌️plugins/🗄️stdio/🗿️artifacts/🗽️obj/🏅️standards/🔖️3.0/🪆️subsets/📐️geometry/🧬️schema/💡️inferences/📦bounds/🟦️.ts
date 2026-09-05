/** 📦 `bounds` — the obj snapshot's vertex-derived spatial bounding box. */

export interface ObjBounds {
  min: [number, number, number];
  max: [number, number, number];
  vertexCount: number;
  faceCount: number;
  groupCount: number;
}
