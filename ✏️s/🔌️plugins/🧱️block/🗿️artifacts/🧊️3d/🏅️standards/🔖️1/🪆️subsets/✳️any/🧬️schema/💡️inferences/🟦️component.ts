/** 💡️ Block3d inference schema — bounds (bounding box + vertex count) over the object kind's rim vortex templates. */

export interface BoundingBox3d {
  min: [number, number, number];
  max: [number, number, number];
}

export interface Block3dBounds {
  boundingBox: BoundingBox3d | null;
  vertexCount: number;
}

export interface Block3dInference {
  /** @state inferred */
  bounds: Block3dBounds;
}
