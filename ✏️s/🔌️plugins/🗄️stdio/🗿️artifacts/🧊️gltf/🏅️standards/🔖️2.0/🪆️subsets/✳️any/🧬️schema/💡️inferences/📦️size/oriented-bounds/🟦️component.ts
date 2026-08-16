/** 💡️ oriented-bounds atomic glTF inference leaf. */
export const gltfOrientedBoundsInference = {
  id: 's.stdio.gltf.inference.oriented-bounds.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.oriented-bounds.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfOrientedBoundsInference = typeof gltfOrientedBoundsInference;

