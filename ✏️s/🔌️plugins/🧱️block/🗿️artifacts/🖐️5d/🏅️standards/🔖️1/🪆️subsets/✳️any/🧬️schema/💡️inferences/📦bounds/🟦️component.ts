/** 📦 `bounds` — geometric bounding box + vertex count over a part kind's rim grip templates' 3d placements. */

export interface BoundingBox3d {
  min: [number, number, number];
  max: [number, number, number];
}

export interface Block5dBounds {
  boundingBox: BoundingBox3d | null;
  vertexCount: number;
}
