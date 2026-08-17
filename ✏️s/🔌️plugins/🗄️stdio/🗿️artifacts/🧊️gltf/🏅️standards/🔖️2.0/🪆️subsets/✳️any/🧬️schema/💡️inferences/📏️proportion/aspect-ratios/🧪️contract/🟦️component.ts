import vectors from './🔣️component.json' with { type: 'json' };
import { inferGltfAspectRatios } from '../🟦️component.ts';
import type { GltfTsGeometryContext } from '../../../🔨️geometry-core/🟦️component.ts';

for (const vector of vectors.vectors) {
  const result = inferGltfAspectRatios(vector.context as GltfTsGeometryContext);
  if (JSON.stringify(result.value ?? null) !== JSON.stringify(vector.value) || result.availability !== vector.availability) {
    throw new Error(vector.name);
  }
}
