/** 📦 `bounds` — geometric bounding box + vertex count over a node kind's rim handle templates. */

export interface BoundingBox2d {
  min: [number, number];
  max: [number, number];
}

export interface Block2dBounds {
  boundingBox: BoundingBox2d | null;
  vertexCount: number;
}
