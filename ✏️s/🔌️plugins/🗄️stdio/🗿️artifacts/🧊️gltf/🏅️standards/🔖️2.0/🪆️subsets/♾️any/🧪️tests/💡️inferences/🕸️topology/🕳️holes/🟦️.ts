import { test } from "bun:test";
import vectors from '../../../🧫️fixtures/💡️inferences/🕸️topology/🕳️holes/🔣️.json' with { type: 'json' };
import { inferGltfHoles } from '../../../../🧬️schema/💡️inferences/🕸️topology/🕳️holes/🟦️.ts';
import type { GltfTsGeometryContext } from '../../../../🧬️schema/💡️inferences/🔨️geometry-kernel/🟦️.ts';

test("original GLTF 💡️inferences/🕸️topology/🕳️holes vectors", () => {
for (const vector of vectors.vectors) {
  const result = inferGltfHoles(vector.context as GltfTsGeometryContext);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability) throw new Error(vector.name);
}
  console.log("[DEBUG] Original GLTF 💡️inferences/🕸️topology/🕳️holes vectors=" + vectors.vectors.length);
});
