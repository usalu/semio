import { test } from "bun:test";
import vectors from '../../../🧫️fixtures/💡️inferences/📏️proportion/🖼️aspect-ratios/🔣️.json' with { type: 'json' };
import { inferGltfAspectRatios } from '../../../../🧬️schema/💡️inferences/📏️proportion/🖼️aspect-ratios/🟦️.ts';
import type { GltfTsGeometryContext } from '../../../../🧬️schema/💡️inferences/🔨️geometry-core/🟦️.ts';

test("original GLTF 💡️inferences/📏️proportion/🖼️aspect-ratios vectors", () => {
for (const vector of vectors.vectors) {
  const result = inferGltfAspectRatios(vector.context as GltfTsGeometryContext);
  if (JSON.stringify(result.value ?? null) !== JSON.stringify(vector.value) || result.availability !== vector.availability) {
    throw new Error(vector.name);
  }
}
  console.log("[DEBUG] Original GLTF 💡️inferences/📏️proportion/🖼️aspect-ratios vectors=" + vectors.vectors.length);
});
