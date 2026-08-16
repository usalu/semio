/** 💡️ characteristic-length atomic glTF inference leaf. */
export const gltfCharacteristicLengthInference = {
  id: 's.stdio.gltf.inference.characteristic-length.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.characteristic-length.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfCharacteristicLengthInference = typeof gltfCharacteristicLengthInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️component.ts';
export const inferGltfCharacteristicLength = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = Math.hypot(...(bounds(context.points)?.dimensions ?? [0,0,0])); return context.valid && value !== undefined ? exact(context, value, 'metre') : unavailable(context, 'metre'); };
export const unavailableGltfCharacteristicLength = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'metre');
