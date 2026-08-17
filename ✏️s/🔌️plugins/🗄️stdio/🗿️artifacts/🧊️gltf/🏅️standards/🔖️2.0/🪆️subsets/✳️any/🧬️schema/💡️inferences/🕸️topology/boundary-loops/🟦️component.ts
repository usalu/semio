/** 💡️ boundary-loops atomic glTF inference leaf. */
export const gltfBoundaryLoopsInference = {
  id: 's.stdio.gltf.inference.boundary-loops.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.boundary-loops.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfBoundaryLoopsInference = typeof gltfBoundaryLoopsInference;

import { exact, unavailable, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️component.ts';

export const inferGltfBoundaryLoops = (context: GltfTsGeometryContext): GltfTsMeasure<number> => {
  const value = context.topology?.boundaryLoops;
  return context.valid && value !== undefined ? exact(context, value, 'unitless') : unavailable(context, 'unitless');
};

export const unavailableGltfBoundaryLoops = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'unitless');
