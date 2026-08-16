/** 💡️ hull-fill-ratio atomic glTF inference leaf. */
export const gltfHullFillRatioInference = {
  id: 's.stdio.gltf.inference.hull-fill-ratio.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.hull-fill-ratio.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfHullFillRatioInference = typeof gltfHullFillRatioInference;

