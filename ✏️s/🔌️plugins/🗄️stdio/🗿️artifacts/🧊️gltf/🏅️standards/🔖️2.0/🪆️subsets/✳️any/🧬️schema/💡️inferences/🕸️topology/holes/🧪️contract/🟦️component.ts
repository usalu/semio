import vectors from './🔣️component.json' with { type: 'json' };
import { inferGltfHoles } from '../🟦️component.ts';
import type { GltfTsGeometryContext } from '../../../🔨️geometry-core/🟦️component.ts';

for (const vector of vectors.vectors) {
  const result = inferGltfHoles(vector.context as GltfTsGeometryContext);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability) throw new Error(vector.name);
}
