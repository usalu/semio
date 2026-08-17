/** 💡️ aspect-ratios atomic glTF inference leaf. */
export const gltfAspectRatiosInference = {
  id: 's.stdio.gltf.inference.aspect-ratios.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.aspect-ratios.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfAspectRatiosInference = typeof gltfAspectRatiosInference;

import {
  exact,
  sortedExtents,
  unavailable,
  type GltfTsGeometryContext,
  type GltfTsMeasure,
} from '../../🔨️geometry-core/🟦️component.ts';
import type { GltfVec3 } from '../../../../🔨️modules/🧾️measurement-contracts/🟦️component.ts';

export const inferGltfAspectRatios = (context: GltfTsGeometryContext): GltfTsMeasure<GltfVec3> => {
  const extent = sortedExtents(context);
  if (!context.valid || !extent) return unavailable(context, 'unitless');
  return exact(
    context,
    {
      x: extent[1] > 0 ? extent[0] / extent[1] : 0,
      y: extent[2] > 0 ? extent[1] / extent[2] : 0,
      z: extent[2] > 0 ? extent[0] / extent[2] : 0,
    },
    'unitless',
  );
};

export const unavailableGltfAspectRatios = (context: GltfTsGeometryContext): GltfTsMeasure<GltfVec3> =>
  unavailable(context, 'unitless');
