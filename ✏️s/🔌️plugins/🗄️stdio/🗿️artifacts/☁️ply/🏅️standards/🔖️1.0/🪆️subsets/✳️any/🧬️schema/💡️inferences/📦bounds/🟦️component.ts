/** 📦 `bounds` — the ply snapshot's vertex-element bounding box plus vertex/face row counts. */

export interface PlyBounds {
  min: [number, number, number];
  max: [number, number, number];
  vertexCount: number;
  faceCount: number;
}
