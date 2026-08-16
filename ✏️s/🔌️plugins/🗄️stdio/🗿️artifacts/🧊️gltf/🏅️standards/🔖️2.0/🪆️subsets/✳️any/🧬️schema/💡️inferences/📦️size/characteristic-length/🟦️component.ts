/** 💡️ characteristic-length atomic glTF inference leaf. */
export const gltfCharacteristicLengthInference = {
  id: 's.stdio.gltf.inference.characteristic-length.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.characteristic-length.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfCharacteristicLengthInference = typeof gltfCharacteristicLengthInference;

