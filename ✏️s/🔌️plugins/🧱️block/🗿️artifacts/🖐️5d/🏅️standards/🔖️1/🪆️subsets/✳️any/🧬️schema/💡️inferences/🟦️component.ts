/** 💡️ Block5d inference schema — bounds (bounding box + vertex count) over the part kind's rim grip templates' 3d placements. */

export interface BoundingBox3d {
  min: [number, number, number];
  max: [number, number, number];
}

export interface Block5dBounds {
  boundingBox: BoundingBox3d | null;
  vertexCount: number;
}

export interface Block5dInference {
  /** @derived */
  bounds: Block5dBounds;
}
