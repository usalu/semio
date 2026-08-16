/** 💡️ axis-aligned-bounds atomic glTF inference leaf. */
export const gltfAxisAlignedBoundsInference = {
  id: 's.stdio.gltf.inference.axis-aligned-bounds.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.axis-aligned-bounds.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfAxisAlignedBoundsInference = typeof gltfAxisAlignedBoundsInference;

