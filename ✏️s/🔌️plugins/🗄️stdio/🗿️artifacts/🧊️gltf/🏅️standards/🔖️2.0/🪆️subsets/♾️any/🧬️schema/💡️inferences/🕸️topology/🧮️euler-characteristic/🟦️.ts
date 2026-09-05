/** 💡️ euler-characteristic atomic glTF inference leaf. */
export const gltfEulerCharacteristicInference = {
  id: 's.stdio.gltf.inference.euler-characteristic.v1',
  algorithmVersion: 1,
  cacheKey: 's.stdio.gltf.inference.euler-characteristic.v1:geometry-v2',
  reads: ['document/scene', 'document/scenes', 'document/nodes', 'document/meshes', 'document/accessors', 'document/bufferViews', 'document/buffers', 'buffers'],
} as const;
export type GltfEulerCharacteristicInference = typeof gltfEulerCharacteristicInference;

import { exact, unavailable, type GltfTsGeometryContext, type GltfTsMeasure } from '../../🔨️geometry-core/🟦️.ts';

export const inferGltfEulerCharacteristic = (context: GltfTsGeometryContext): GltfTsMeasure<number> => {
  const value = context.topology?.eulerCharacteristic;
  return context.valid && value !== undefined ? exact(context, value, 'unitless') : unavailable(context, 'unitless');
};

export const unavailableGltfEulerCharacteristic = (context: GltfTsGeometryContext): GltfTsMeasure<number> => unavailable(context, 'unitless');
