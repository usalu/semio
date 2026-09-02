/** 💡️ flatness atomic glTF inference leaf. */
export const gltfFlatnessInference = {
  id: 's.stdio.gltf.inference.flatness.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.flatness.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfFlatnessInference = typeof gltfFlatnessInference;

import {
  exact,
  sortedExtents,
  unavailable,
  type GltfTsGeometryContext,
  type GltfTsMeasure,
} from '../../🔨️geometry-core/🟦️.ts';

export const inferGltfFlatness = (context: GltfTsGeometryContext): GltfTsMeasure<number> => {
  const extent = sortedExtents(context);
  return context.valid && extent ? exact(context, extent[1] > 0 ? extent[2] / extent[1] : 0, 'unitless') : unavailable(context, 'unitless');
};

export const unavailableGltfFlatness = (context: GltfTsGeometryContext): GltfTsMeasure<number> =>
  unavailable(context, 'unitless');
