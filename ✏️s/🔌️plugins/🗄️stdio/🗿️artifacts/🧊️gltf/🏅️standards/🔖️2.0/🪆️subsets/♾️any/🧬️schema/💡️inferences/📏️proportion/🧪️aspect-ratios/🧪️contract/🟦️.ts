import vectors from './🔣️.json' with { type: 'json' };
import { inferGltfAspectRatios } from '../🟦️.ts';
import type { GltfTsGeometryContext } from '../../../🔨️geometry-core/🟦️.ts';

for (const vector of vectors.vectors) {
  const result = inferGltfAspectRatios(vector.context as GltfTsGeometryContext);
  if (JSON.stringify(result.value ?? null) !== JSON.stringify(vector.value) || result.availability !== vector.availability) {
    throw new Error(vector.name);
  }
}
