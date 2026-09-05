/** 💡️ exposed-area atomic glTF inference leaf. */
export const gltfExposedAreaInference = {
  id: 's.stdio.gltf.inference.exposed-area.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.exposed-area.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfExposedAreaInference = typeof gltfExposedAreaInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️.ts';
export const inferGltfExposedArea = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = surfaceArea(context); return context.valid && value !== undefined ? exact(context, value, 'squareMetre') : unavailable(context, 'squareMetre'); };
export const unavailableGltfExposedArea = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'squareMetre');
