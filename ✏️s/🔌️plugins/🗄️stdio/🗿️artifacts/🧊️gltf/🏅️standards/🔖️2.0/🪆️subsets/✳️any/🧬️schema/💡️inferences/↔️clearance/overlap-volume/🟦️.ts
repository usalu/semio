/** 💡️ overlap-volume atomic glTF inference leaf. */
export const gltfOverlapVolumeInference = {
  id: 's.stdio.gltf.inference.overlap-volume.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.overlap-volume.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfOverlapVolumeInference = typeof gltfOverlapVolumeInference;

