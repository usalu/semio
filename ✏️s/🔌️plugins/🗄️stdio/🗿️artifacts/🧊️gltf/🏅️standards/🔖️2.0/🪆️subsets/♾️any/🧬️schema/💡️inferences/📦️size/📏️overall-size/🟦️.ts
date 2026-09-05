/** 💡️ overall-size atomic glTF inference leaf. */
export const gltfOverallSizeInference = {
  id: 's.stdio.gltf.inference.overall-size.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.overall-size.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfOverallSizeInference = typeof gltfOverallSizeInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️.ts';
export const inferGltfOverallSize = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = Math.hypot(...(bounds(context.points)?.dimensions ?? [0,0,0])); return context.valid && value !== undefined ? exact(context, value, 'metre') : unavailable(context, 'metre'); };
export const unavailableGltfOverallSize = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'metre');
