import vectors from './🔣️.json' with { type: 'json' };
import { inferGltfOverallSize } from '../🟦️.ts';
for (const vector of vectors.vectors) {
  const result = inferGltfOverallSize(vector.context);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability || inferGltfOverallSize(vector.context).value !== result.value) throw new Error(vector.name);
}
