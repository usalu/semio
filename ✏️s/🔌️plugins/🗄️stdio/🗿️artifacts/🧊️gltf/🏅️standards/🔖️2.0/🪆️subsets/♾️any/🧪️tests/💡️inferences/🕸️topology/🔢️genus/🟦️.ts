import { test } from "bun:test";
import vectors from '../../../🧫️fixtures/💡️inferences/🕸️topology/🔢️genus/🔣️.json' with { type: 'json' };
import { inferGltfGenus } from '../../../../🧬️schema/💡️inferences/🕸️topology/🔢️genus/🟦️.ts';
import type { GltfTsGeometryContext } from '../../../../🧬️schema/💡️inferences/🔨️geometry-kernel/🟦️.ts';

test("original GLTF 💡️inferences/🕸️topology/🔢️genus vectors", () => {
for (const vector of vectors.vectors) {
  const result = inferGltfGenus(vector.context as GltfTsGeometryContext);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability) throw new Error(vector.name);
}
});
