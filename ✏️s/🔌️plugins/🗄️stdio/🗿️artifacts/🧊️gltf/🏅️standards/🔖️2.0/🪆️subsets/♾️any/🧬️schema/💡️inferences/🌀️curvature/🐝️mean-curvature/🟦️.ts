/** 💡️ mean-curvature atomic glTF inference leaf. */
export const gltfMeanCurvatureInference = {
  id: 's.stdio.gltf.inference.mean-curvature.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.mean-curvature.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfMeanCurvatureInference = typeof gltfMeanCurvatureInference;

