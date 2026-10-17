import { test } from "bun:test";
import vectors from '../../../🧫️fixtures/💡️inferences/📏️proportion/↔️elongation/🔣️.json' with { type: 'json' };
import { inferGltfElongation } from '../../../../🧬️schema/💡️inferences/📏️proportion/↔️elongation/🟦️.ts';
import type { GltfTsGeometryContext } from '../../../../🧬️schema/💡️inferences/🔨️geometry-kernel/🟦️.ts';

test("original GLTF 💡️inferences/📏️proportion/↔️elongation vectors", () => {
for (const vector of vectors.vectors) {
  const result = inferGltfElongation(vector.context as GltfTsGeometryContext);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability) throw new Error(vector.name);
}
});
