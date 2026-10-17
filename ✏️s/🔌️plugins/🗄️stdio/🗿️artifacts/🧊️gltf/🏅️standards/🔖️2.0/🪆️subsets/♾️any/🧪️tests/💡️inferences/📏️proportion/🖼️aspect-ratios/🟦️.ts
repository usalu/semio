import { test } from "bun:test";
import vectors from '../../../🧫️fixtures/💡️inferences/📏️proportion/🖼️aspect-ratios/🔣️.json' with { type: 'json' };
import { inferGltfAspectRatios } from '../../../../🧬️schema/💡️inferences/📏️proportion/🖼️aspect-ratios/🟦️.ts';
import type { GltfTsGeometryContext } from '../../../../🧬️schema/💡️inferences/🔨️geometry-kernel/🟦️.ts';

test("original GLTF 💡️inferences/📏️proportion/🖼️aspect-ratios vectors", () => {
for (const vector of vectors.vectors) {
  const result = inferGltfAspectRatios(vector.context as GltfTsGeometryContext);
  if (JSON.stringify(result.value ?? null) !== JSON.stringify(vector.value) || result.availability !== vector.availability) {
    throw new Error(vector.name);
  }
}
});
