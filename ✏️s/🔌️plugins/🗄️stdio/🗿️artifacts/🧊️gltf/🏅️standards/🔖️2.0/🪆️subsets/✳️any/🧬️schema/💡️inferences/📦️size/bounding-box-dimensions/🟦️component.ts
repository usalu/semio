/** 💡️ bounding-box-dimensions atomic glTF inference leaf. */
export const gltfBoundingBoxDimensionsInference = {
  id: 's.stdio.gltf.inference.bounding-box-dimensions.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.bounding-box-dimensions.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfBoundingBoxDimensionsInference = typeof gltfBoundingBoxDimensionsInference;

