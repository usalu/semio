import vectors from './🔣️component.json' with { type: 'json' };
import { inferGltfSlenderness } from '../🟦️component.ts';
import type { GltfTsGeometryContext } from '../../../🔨️geometry-core/🟦️component.ts';

for (const vector of vectors.vectors) {
  const result = inferGltfSlenderness(vector.context as GltfTsGeometryContext);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability) throw new Error(vector.name);
}
