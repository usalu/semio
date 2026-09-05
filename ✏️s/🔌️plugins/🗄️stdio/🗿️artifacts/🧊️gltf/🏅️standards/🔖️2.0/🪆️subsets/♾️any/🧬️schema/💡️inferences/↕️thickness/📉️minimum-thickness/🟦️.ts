/** 💡️ minimum-thickness atomic glTF inference leaf. */
export const gltfMinimumThicknessInference = {
  id: 's.stdio.gltf.inference.minimum-thickness.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.minimum-thickness.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfMinimumThicknessInference = typeof gltfMinimumThicknessInference;

