/** 💡️ mean-thickness atomic glTF inference leaf. */
export const gltfMeanThicknessInference = {
  id: 's.stdio.gltf.inference.mean-thickness.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.mean-thickness.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfMeanThicknessInference = typeof gltfMeanThicknessInference;

