/** 💡️ compactness atomic glTF inference leaf. */
export const gltfCompactnessInference = {
  id: 's.stdio.gltf.inference.compactness.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.compactness.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfCompactnessInference = typeof gltfCompactnessInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️.ts';
export const inferGltfCompactness = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = (()=>{const v=Math.abs(signedVolume(context)),s=surfaceArea(context);return v>0&&s>0?Math.cbrt(Math.PI)*Math.pow(6*v,2/3)/s:undefined})(); return context.valid && value !== undefined ? exact(context, value, 'unitless') : unavailable(context, 'unitless'); };
export const unavailableGltfCompactness = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'unitless');
