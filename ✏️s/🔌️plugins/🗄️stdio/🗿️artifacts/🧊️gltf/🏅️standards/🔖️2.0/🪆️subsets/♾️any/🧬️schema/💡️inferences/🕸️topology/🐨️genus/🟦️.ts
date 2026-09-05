/** 💡️ genus atomic glTF inference leaf. */
export const gltfGenusInference = {
  id: 's.stdio.gltf.inference.genus.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.genus.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfGenusInference = typeof gltfGenusInference;

import { exact, unavailable, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️.ts';

export const inferGltfGenus = (context: GltfTsGeometryContext): GltfTsMeasure<number> => {
  if (!context.valid) return unavailable(context, 'unitless');
  const value = context.topology?.genus;
  return value !== undefined ? exact(context, value, 'unitless') : unavailable(context, 'unitless', 'nonManifold');
};

export const unavailableGltfGenus = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'unitless');
