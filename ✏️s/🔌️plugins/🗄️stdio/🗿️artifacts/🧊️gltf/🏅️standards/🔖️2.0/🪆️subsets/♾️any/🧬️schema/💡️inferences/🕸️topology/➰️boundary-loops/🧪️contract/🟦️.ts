import vectors from './🔣️.json' with { type: 'json' };
import { inferGltfBoundaryLoops } from '../🟦️.ts';
import type { GltfTsGeometryContext } from '../../../🔨️geometry-core/🟦️.ts';

for (const vector of vectors.vectors) {
  const result = inferGltfBoundaryLoops(vector.context as GltfTsGeometryContext);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability) throw new Error(vector.name);
}
