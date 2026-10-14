import { test } from "bun:test";
import vectors from '../../../🧫️fixtures/💡️inferences/📏️proportion/📏️slenderness/🔣️.json' with { type: 'json' };
import { inferGltfSlenderness } from '../../../../🧬️schema/💡️inferences/📏️proportion/📏️slenderness/🟦️.ts';
import type { GltfTsGeometryContext } from '../../../../🧬️schema/💡️inferences/🔨️geometry-core/🟦️.ts';

test("original GLTF 💡️inferences/📏️proportion/📏️slenderness vectors", () => {
for (const vector of vectors.vectors) {
  const result = inferGltfSlenderness(vector.context as GltfTsGeometryContext);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability) throw new Error(vector.name);
}
  console.log("[DEBUG] Original GLTF 💡️inferences/📏️proportion/📏️slenderness vectors=" + vectors.vectors.length);
});
