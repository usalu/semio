/** 💡️ Block2d inference schema — bounds (bounding box + vertex count) over the node kind's rim handle templates. */

export interface BoundingBox2d {
  min: [number, number];
  max: [number, number];
}

export interface Block2dBounds {
  boundingBox: BoundingBox2d | null;
  vertexCount: number;
}

export interface Block2dInference {
  /** @derived */
  bounds: Block2dBounds;
}
