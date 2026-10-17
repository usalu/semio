import { test } from "bun:test";
import vectors from '../../../🧫️fixtures/💡️inferences/📏️proportion/📃️flatness/🔣️.json' with { type: 'json' };
import { inferGltfFlatness } from '../../../../🧬️schema/💡️inferences/📏️proportion/📃️flatness/🟦️.ts';
import type { GltfTsGeometryContext } from '../../../../🧬️schema/💡️inferences/🔨️geometry-kernel/🟦️.ts';

test("original GLTF 💡️inferences/📏️proportion/📃️flatness vectors", () => {
for (const vector of vectors.vectors) {
  const result = inferGltfFlatness(vector.context as GltfTsGeometryContext);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability) throw new Error(vector.name);
}
});
