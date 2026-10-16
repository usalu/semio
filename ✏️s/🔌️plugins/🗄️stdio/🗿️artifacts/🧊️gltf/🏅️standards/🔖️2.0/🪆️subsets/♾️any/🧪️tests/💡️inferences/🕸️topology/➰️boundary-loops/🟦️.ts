import { test } from "bun:test";
import vectors from '../../../🧫️fixtures/💡️inferences/🕸️topology/➰️boundary-loops/🔣️.json' with { type: 'json' };
import { inferGltfBoundaryLoops } from '../../../../🧬️schema/💡️inferences/🕸️topology/➰️boundary-loops/🟦️.ts';
import type { GltfTsGeometryContext } from '../../../../🧬️schema/💡️inferences/🔨️geometry-kernel/🟦️.ts';

test("original GLTF 💡️inferences/🕸️topology/➰️boundary-loops vectors", () => {
for (const vector of vectors.vectors) {
  const result = inferGltfBoundaryLoops(vector.context as GltfTsGeometryContext);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability) throw new Error(vector.name);
}
  console.log("[DEBUG] Original GLTF 💡️inferences/🕸️topology/➰️boundary-loops vectors=" + vectors.vectors.length);
});
