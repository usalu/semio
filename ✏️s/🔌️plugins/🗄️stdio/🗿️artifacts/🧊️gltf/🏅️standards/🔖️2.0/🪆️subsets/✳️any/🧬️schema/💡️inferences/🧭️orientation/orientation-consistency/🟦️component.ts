/** 💡️ orientation-consistency atomic glTF inference leaf. */
export const gltfOrientationConsistencyInference = {
  id: 's.stdio.gltf.inference.orientation-consistency.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.orientation-consistency.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfOrientationConsistencyInference = typeof gltfOrientationConsistencyInference;

