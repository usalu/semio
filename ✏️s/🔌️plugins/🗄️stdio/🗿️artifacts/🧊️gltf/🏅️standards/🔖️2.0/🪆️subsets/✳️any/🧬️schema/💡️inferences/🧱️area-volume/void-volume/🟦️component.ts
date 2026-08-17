/** 💡️ void-volume atomic glTF inference leaf. */
export const gltfVoidVolumeInference = {
  id: 's.stdio.gltf.inference.void-volume.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.void-volume.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfVoidVolumeInference = typeof gltfVoidVolumeInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️component.ts';
export const inferGltfVoidVolume = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = 0; return context.valid && value !== undefined ? exact(context, value, 'cubicMetre') : unavailable(context, 'cubicMetre'); };
export const unavailableGltfVoidVolume = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'cubicMetre');
