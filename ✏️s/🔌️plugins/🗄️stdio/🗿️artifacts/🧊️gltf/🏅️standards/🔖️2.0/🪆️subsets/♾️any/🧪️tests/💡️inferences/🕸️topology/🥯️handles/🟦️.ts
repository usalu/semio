import { test } from "bun:test";
import vectors from '../../../🧫️fixtures/💡️inferences/🕸️topology/🥯️handles/🔣️.json' with { type: 'json' };
import { inferGltfHandles } from '../../../../🧬️schema/💡️inferences/🕸️topology/🥯️handles/🟦️.ts';
import type { GltfTsGeometryContext } from '../../../../🧬️schema/💡️inferences/🔨️geometry-kernel/🟦️.ts';

test("original GLTF 💡️inferences/🕸️topology/🥯️handles vectors", () => {
for (const vector of vectors.vectors) {
  const result = inferGltfHandles(vector.context as GltfTsGeometryContext);
  if ((result.value ?? null) !== vector.value || result.availability !== vector.availability) throw new Error(vector.name);
}
  console.log("[DEBUG] Original GLTF 💡️inferences/🕸️topology/🥯️handles vectors=" + vectors.vectors.length);
});
