/** 💡️ inertia-tensor atomic glTF inference leaf. */
export const gltfInertiaTensorInference = {
  id: 's.stdio.gltf.inference.inertia-tensor.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.inertia-tensor.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfInertiaTensorInference = typeof gltfInertiaTensorInference;

