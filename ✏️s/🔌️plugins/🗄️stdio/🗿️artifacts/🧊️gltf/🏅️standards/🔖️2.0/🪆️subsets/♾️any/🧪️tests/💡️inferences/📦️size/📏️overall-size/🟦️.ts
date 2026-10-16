import { test } from "bun:test";
import vectors from '../../../🧫️fixtures/💡️inferences/📦️size/📏️overall-size/🔣️.json' with { type: 'json' };
import { inferGltfOverallSize } from '../../../../🧬️schema/💡️inferences/📦️size/📏️overall-size/🟦️.ts';
import type { GltfTsGeometryContext } from '../../../../🧬️schema/💡️inferences/🔨️geometry-kernel/🟦️.ts';
test("original GLTF 💡️inferences/📦️size/📏️overall-size vectors", () => {
for (const vector of vectors.vectors) {
  const context = vector.context as GltfTsGeometryContext;
  const result = inferGltfOverallSize(context);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability || inferGltfOverallSize(context).value !== result.value) throw new Error(vector.name);
}
  console.log("[DEBUG] Original GLTF 💡️inferences/📦️size/📏️overall-size vectors=" + vectors.vectors.length);
});
