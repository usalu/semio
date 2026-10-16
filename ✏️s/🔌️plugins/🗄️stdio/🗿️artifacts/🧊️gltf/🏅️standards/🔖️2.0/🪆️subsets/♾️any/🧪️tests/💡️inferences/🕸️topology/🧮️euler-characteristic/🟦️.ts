import { test } from "bun:test";
import vectors from '../../../🧫️fixtures/💡️inferences/🕸️topology/🧮️euler-characteristic/🔣️.json' with { type: 'json' };
import { inferGltfEulerCharacteristic } from '../../../../🧬️schema/💡️inferences/🕸️topology/🧮️euler-characteristic/🟦️.ts';
import type { GltfTsGeometryContext } from '../../../../🧬️schema/💡️inferences/🔨️geometry-kernel/🟦️.ts';

test("original GLTF 💡️inferences/🕸️topology/🧮️euler-characteristic vectors", () => {
for (const vector of vectors.vectors) {
  const result = inferGltfEulerCharacteristic(vector.context as GltfTsGeometryContext);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability) throw new Error(vector.name);
}
  console.log("[DEBUG] Original GLTF 💡️inferences/🕸️topology/🧮️euler-characteristic vectors=" + vectors.vectors.length);
});
