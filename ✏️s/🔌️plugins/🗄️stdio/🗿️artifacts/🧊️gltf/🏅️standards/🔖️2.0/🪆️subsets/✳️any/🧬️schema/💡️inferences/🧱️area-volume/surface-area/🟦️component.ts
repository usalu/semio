/** 💡️ surface-area atomic glTF inference leaf. */
export const gltfSurfaceAreaInference = {
  id: 's.stdio.gltf.inference.surface-area.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.surface-area.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfSurfaceAreaInference = typeof gltfSurfaceAreaInference;

