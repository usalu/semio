/** 💡️ slenderness atomic glTF inference leaf. */
export const gltfSlendernessInference = {
  id: 's.stdio.gltf.inference.slenderness.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.slenderness.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfSlendernessInference = typeof gltfSlendernessInference;

import {
  exact,
  sortedExtents,
  unavailable,
  type GltfTsGeometryContext,
  type GltfTsMeasure,
} from '../../🔨️geometry-core/🟦️component.ts';

export const inferGltfSlenderness = (context: GltfTsGeometryContext): GltfTsMeasure<number> => {
  const extent = sortedExtents(context);
  return context.valid && extent ? exact(context, extent[1] > 0 ? extent[0] / extent[1] : 0, 'unitless') : unavailable(context, 'unitless');
};

export const unavailableGltfSlenderness = (context: GltfTsGeometryContext): GltfTsMeasure<number> =>
  unavailable(context, 'unitless');
