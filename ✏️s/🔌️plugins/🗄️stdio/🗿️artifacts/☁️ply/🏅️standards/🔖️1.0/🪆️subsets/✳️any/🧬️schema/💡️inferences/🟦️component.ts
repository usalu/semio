/** 💡️ Ply inference schema — vertex-element bounding box plus vertex/face counts. */

export interface PlyBounds {
  min: [number, number, number];
  max: [number, number, number];
  vertexCount: number;
  faceCount: number;
}

export interface PlyInference {
  /** @derived */
  bounds: PlyBounds;
}
