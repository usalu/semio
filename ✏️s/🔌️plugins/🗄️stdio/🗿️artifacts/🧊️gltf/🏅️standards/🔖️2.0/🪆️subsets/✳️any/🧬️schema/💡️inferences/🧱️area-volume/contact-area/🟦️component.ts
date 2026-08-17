/** 💡️ contact-area atomic glTF inference leaf. */
export const gltfContactAreaInference = {
  id: 's.stdio.gltf.inference.contact-area.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.contact-area.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfContactAreaInference = typeof gltfContactAreaInference;
import { bounds, exact, unavailable, signedVolume, surfaceArea, type GltfTsBounds3, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️component.ts';
export const inferGltfContactArea = (context: GltfTsGeometryContext): GltfTsMeasure<number> => { const value = 0; return context.valid && value !== undefined ? exact(context, value, 'squareMetre') : unavailable(context, 'squareMetre'); };
export const unavailableGltfContactArea = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'squareMetre');
