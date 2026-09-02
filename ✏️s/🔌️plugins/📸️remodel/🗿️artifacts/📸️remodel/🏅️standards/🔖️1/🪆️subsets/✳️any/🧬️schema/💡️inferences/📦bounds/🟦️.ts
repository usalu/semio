/** 📦 `bounds` — one named inference: the reconstructed mesh's axis-aligned bounding box plus vertex/face counts. */

export interface RemodelBoundingBox {
  min: [number, number, number];
  max: [number, number, number];
}

export interface RemodelBounds {
  boundingBox: RemodelBoundingBox;
  vertexCount: number;
  faceCount: number;
}
