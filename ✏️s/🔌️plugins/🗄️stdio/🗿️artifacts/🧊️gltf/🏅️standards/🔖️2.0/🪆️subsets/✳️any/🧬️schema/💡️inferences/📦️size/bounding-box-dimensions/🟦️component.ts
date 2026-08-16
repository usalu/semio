/** 💡️ bounding-box-dimensions atomic glTF inference leaf. */
export const gltfBoundingBoxDimensionsInference = {
  id: 's.stdio.gltf.inference.bounding-box-dimensions.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.bounding-box-dimensions.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfBoundingBoxDimensionsInference = typeof gltfBoundingBoxDimensionsInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️component.ts';
export const inferGltfBoundingBoxDimensions = (context: GltfTsGeometryContext): GltfTsMeasure<readonly [number, number, number]> => { const value = bounds(context.points)?.dimensions; return context.valid && value !== undefined ? exact(context, value, 'metre') : unavailable(context, 'metre'); };
export const unavailableGltfBoundingBoxDimensions = (context: GltfTsGeometryContext): GltfTsMeasure<readonly [number, number, number]> => unavailable(context, 'metre');
