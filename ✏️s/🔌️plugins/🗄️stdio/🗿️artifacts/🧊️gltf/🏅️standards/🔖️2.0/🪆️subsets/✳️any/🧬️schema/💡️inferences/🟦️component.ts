/** 💡️ Gltf inference schema — mesh-primitive POSITION-accessor-derived bounding box. */

export interface GltfBounds {
  min: [number, number, number];
  max: [number, number, number];
  vertexCount: number;
  meshCount: number;
  primitiveCount: number;
}

export interface GltfInference {
  /** @derived */
  bounds: GltfBounds;
}
