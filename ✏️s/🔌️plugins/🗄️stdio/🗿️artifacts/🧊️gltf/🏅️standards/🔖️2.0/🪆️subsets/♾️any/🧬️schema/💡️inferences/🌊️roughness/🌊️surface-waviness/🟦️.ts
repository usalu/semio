/** 💡️ surface-waviness atomic glTF inference leaf. */
export const gltfSurfaceWavinessInference = {
  id: 's.stdio.gltf.inference.surface-waviness.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.surface-waviness.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfSurfaceWavinessInference = typeof gltfSurfaceWavinessInference;

