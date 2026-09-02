/** 📦 `bounds` — one named inference: the reconstructed mesh's axis-aligned bounding box plus vertex/face counts. */

export interface RemodelingBoundingBox {
  min: [number, number, number];
  max: [number, number, number];
}

export interface RemodelingBounds {
  boundingBox: RemodelingBoundingBox;
  vertexCount: number;
  faceCount: number;
}
