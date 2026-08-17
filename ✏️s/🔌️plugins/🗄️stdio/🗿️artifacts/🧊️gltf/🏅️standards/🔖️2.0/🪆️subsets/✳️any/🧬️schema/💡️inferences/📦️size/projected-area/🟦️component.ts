/** 💡️ projected-area atomic glTF inference leaf. */
export const gltfProjectedAreaInference = {
  id: 's.stdio.gltf.inference.projected-area.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.projected-area.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfProjectedAreaInference = typeof gltfProjectedAreaInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️component.ts';
export const inferGltfProjectedArea = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = (()=>{const d=bounds(context.points)?.dimensions;return d?d[0]*d[1]:undefined})(); return context.valid && value !== undefined ? exact(context, value, 'squareMetre') : unavailable(context, 'squareMetre'); };
export const unavailableGltfProjectedArea = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'squareMetre');
