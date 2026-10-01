/** 💡️ deviation-from-smoothed-geometry atomic glTF inference leaf. */
export const gltfDeviationFromSmoothedGeometryInference = {
  id: 's.stdio.gltf.inference.deviation-from-smoothed-geometry.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.deviation-from-smoothed-geometry.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfDeviationFromSmoothedGeometryInference = typeof gltfDeviationFromSmoothedGeometryInference;

