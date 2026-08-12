/** 📦 `bounds` — the gltf snapshot's mesh-primitive `POSITION`-accessor-derived spatial bounding box. */

export interface GltfBounds {
  min: [number, number, number];
  max: [number, number, number];
  vertexCount: number;
  meshCount: number;
  primitiveCount: number;
}
