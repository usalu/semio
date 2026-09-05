/** 💡️ reentrant-volume atomic glTF inference leaf. */
export const gltfReentrantVolumeInference = {
  id: 's.stdio.gltf.inference.reentrant-volume.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.reentrant-volume.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfReentrantVolumeInference = typeof gltfReentrantVolumeInference;

