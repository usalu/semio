/** 💡️ axis-aligned-bounds atomic glTF inference leaf. */
export const gltfAxisAlignedBoundsInference = {
  id: 's.stdio.gltf.inference.axis-aligned-bounds.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.axis-aligned-bounds.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfAxisAlignedBoundsInference = typeof gltfAxisAlignedBoundsInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️component.ts';
export const inferGltfAxisAlignedBounds = (context: GltfTsGeometryContext): GltfTsMeasure<GltfTsBounds3> => { const value = bounds(context.points); return context.valid && value !== undefined ? exact(context, value, 'metre') : unavailable(context, 'metre'); };
export const unavailableGltfAxisAlignedBounds = (context: GltfTsGeometryContext): GltfTsMeasure<GltfTsBounds3> => unavailable(context, 'metre');
