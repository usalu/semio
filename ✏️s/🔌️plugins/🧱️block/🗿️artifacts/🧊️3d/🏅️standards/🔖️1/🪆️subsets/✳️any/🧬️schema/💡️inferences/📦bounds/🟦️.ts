/** 📦 `bounds` — geometric bounding box + vertex count over an object kind's rim vortex templates. */

export interface BoundingBox3d {
  min: [number, number, number];
  max: [number, number, number];
}

export interface Block3dBounds {
  boundingBox: BoundingBox3d | null;
  vertexCount: number;
}
