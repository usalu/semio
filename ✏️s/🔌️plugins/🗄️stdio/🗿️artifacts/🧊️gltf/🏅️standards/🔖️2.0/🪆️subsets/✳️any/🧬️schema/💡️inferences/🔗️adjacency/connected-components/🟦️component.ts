/** 💡️ connected-components atomic glTF inference leaf. */
export const gltfConnectedComponentsInference = {
  id: 's.stdio.gltf.inference.connected-components.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.connected-components.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfConnectedComponentsInference = typeof gltfConnectedComponentsInference;

