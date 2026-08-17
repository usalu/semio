/** 💡️ material-volume atomic glTF inference leaf. */
export const gltfMaterialVolumeInference = {
  id: 's.stdio.gltf.inference.material-volume.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.material-volume.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfMaterialVolumeInference = typeof gltfMaterialVolumeInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️component.ts';
export const inferGltfMaterialVolume = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = Math.abs(signedVolume(context)); return context.valid && value !== undefined ? exact(context, value, 'cubicMetre') : unavailable(context, 'cubicMetre'); };
export const unavailableGltfMaterialVolume = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'cubicMetre');
