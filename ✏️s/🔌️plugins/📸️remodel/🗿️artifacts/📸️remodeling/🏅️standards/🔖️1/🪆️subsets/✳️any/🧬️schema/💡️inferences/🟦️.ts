/** 💡️ Remodeling inference schema — reconstructed-mesh bounds (bounding box + vertex/face counts). */

export interface RemodelingBoundingBox {
  min: [number, number, number];
  max: [number, number, number];
}

export interface RemodelingBounds {
  boundingBox: RemodelingBoundingBox;
  vertexCount: number;
  faceCount: number;
}

export interface RemodelingInference {
  /** @derived */
  bounds: RemodelingBounds;
}
