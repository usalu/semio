/** 💡️ overall-size atomic glTF inference leaf. */
export const gltfOverallSizeInference = {
  id: 's.stdio.gltf.inference.overall-size.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.overall-size.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfOverallSizeInference = typeof gltfOverallSizeInference;

