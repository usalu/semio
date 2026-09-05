/** 💡️ number-of-contacts atomic glTF inference leaf. */
export const gltfNumberOfContactsInference = {
  id: 's.stdio.gltf.inference.number-of-contacts.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.number-of-contacts.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfNumberOfContactsInference = typeof gltfNumberOfContactsInference;

