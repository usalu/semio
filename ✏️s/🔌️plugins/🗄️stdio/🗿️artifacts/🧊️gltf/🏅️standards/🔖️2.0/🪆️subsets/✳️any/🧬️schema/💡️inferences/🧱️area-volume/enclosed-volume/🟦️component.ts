/** 💡️ enclosed-volume atomic glTF inference leaf. */
export const gltfEnclosedVolumeInference = {
  id: 's.stdio.gltf.inference.enclosed-volume.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.enclosed-volume.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfEnclosedVolumeInference = typeof gltfEnclosedVolumeInference;

