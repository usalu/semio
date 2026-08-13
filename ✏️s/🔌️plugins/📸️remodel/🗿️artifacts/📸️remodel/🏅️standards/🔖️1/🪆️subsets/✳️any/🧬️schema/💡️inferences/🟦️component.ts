/** 💡️ Remodel inference schema — reconstructed-mesh bounds (bounding box + vertex/face counts). */

export interface RemodelBoundingBox {
  min: [number, number, number];
  max: [number, number, number];
}

export interface RemodelBounds {
  boundingBox: RemodelBoundingBox;
  vertexCount: number;
  faceCount: number;
}

export interface RemodelInference {
  /** @derived */
  bounds: RemodelBounds;
}
