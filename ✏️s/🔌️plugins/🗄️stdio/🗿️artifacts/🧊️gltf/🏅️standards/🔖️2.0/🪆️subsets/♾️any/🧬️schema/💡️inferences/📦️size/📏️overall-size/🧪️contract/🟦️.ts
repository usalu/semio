import vectors from '../🧫️fixtures/🔣️.json' with { type: 'json' };
import { inferGltfOverallSize } from '../🟦️.ts';
import type { GltfTsGeometryContext } from '../../../🔨️geometry-core/🟦️.ts';
for (const vector of vectors.vectors) {
  const context = vector.context as GltfTsGeometryContext;
  const result = inferGltfOverallSize(context);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability || inferGltfOverallSize(context).value !== result.value) throw new Error(vector.name);
}
