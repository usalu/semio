/** 💡️ surface-area atomic glTF inference leaf. */
export const gltfSurfaceAreaInference = {
  id: 's.stdio.gltf.inference.surface-area.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.surface-area.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfSurfaceAreaInference = typeof gltfSurfaceAreaInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️component.ts';
export const inferGltfSurfaceArea = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = surfaceArea(context); return context.valid && value !== undefined ? exact(context, value, 'square-metre') : unavailable(context, 'square-metre'); };
export const unavailableGltfSurfaceArea = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'square-metre');
