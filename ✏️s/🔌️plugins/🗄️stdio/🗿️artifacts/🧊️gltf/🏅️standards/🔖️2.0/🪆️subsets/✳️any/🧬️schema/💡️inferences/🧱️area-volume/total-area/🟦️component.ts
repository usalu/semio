/** 💡️ total-area atomic glTF inference leaf. */
export const gltfTotalAreaInference = {
  id: 's.stdio.gltf.inference.total-area.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.total-area.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfTotalAreaInference = typeof gltfTotalAreaInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️component.ts';
export const inferGltfTotalArea = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = surfaceArea(context); return context.valid && value !== undefined ? exact(context, value, 'square-metre') : unavailable(context, 'square-metre'); };
export const unavailableGltfTotalArea = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'square-metre');
