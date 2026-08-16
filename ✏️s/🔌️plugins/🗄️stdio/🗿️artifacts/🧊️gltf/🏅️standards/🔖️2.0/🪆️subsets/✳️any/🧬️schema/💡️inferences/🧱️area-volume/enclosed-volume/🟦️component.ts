/** 💡️ enclosed-volume atomic glTF inference leaf. */
export const gltfEnclosedVolumeInference = {
  id: 's.stdio.gltf.inference.enclosed-volume.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.enclosed-volume.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfEnclosedVolumeInference = typeof gltfEnclosedVolumeInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️component.ts';
export const inferGltfEnclosedVolume = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = Math.abs(signedVolume(context)); return context.valid && value !== undefined ? exact(context, value, 'cubic-metre') : unavailable(context, 'cubic-metre'); };
export const unavailableGltfEnclosedVolume = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'cubic-metre');
