import vectors from './🔣️component.json' with { type: 'json' };
import { inferGltfOverallSize } from '../🟦️component.ts';
for (const vector of vectors.vectors) {
  const result = inferGltfOverallSize(vector.context);
  if ((result.value ?? null) !== vector.value || result.validity !== vector.availability || inferGltfOverallSize(vector.context).value !== result.value) throw new Error(vector.name);
}
