/** 💡️ surface-to-volume-ratio atomic glTF inference leaf. */
export const gltfSurfaceToVolumeRatioInference = {
  id: 's.stdio.gltf.inference.surface-to-volume-ratio.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.surface-to-volume-ratio.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfSurfaceToVolumeRatioInference = typeof gltfSurfaceToVolumeRatioInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️.ts';
export const inferGltfSurfaceToVolumeRatio = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = (()=>{const v=Math.abs(signedVolume(context));return v>0?surfaceArea(context)/v:undefined})(); return context.valid && value !== undefined ? exact(context, value, 'inverseMetre') : unavailable(context, 'inverseMetre'); };
export const unavailableGltfSurfaceToVolumeRatio = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'inverseMetre');
