/** 💡️ hull-fill-ratio atomic glTF inference leaf. */
export const gltfHullFillRatioInference = {
  id: 's.stdio.gltf.inference.hull-fill-ratio.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.hull-fill-ratio.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfHullFillRatioInference = typeof gltfHullFillRatioInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️component.ts';
export const inferGltfHullFillRatio = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = context.points.length?1:undefined; return context.valid && value !== undefined ? exact(context, value, 'unitless') : unavailable(context, 'unitless'); };
export const unavailableGltfHullFillRatio = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'unitless');
