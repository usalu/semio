/** 💡️ Obj inference schema — vertex-derived bounding box. */

export interface ObjBounds {
  min: [number, number, number];
  max: [number, number, number];
  vertexCount: number;
  faceCount: number;
  groupCount: number;
}

export interface ObjInference {
  /** @derived */
  bounds: ObjBounds;
}
