import { test } from "bun:test";
import vectors from '../../../🧫️fixtures/💡️inferences/📦️size/↔️axis-aligned-bounds/🔣️.json' with { type: 'json' };
import { inferGltfAxisAlignedBounds } from '../../../../🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🟦️.ts';
import type { GltfTsGeometryContext } from '../../../../🧬️schema/💡️inferences/🔨️geometry-core/🟦️.ts';
test("original GLTF 💡️inferences/📦️size/↔️axis-aligned-bounds vectors", () => {
for (const vector of vectors.vectors) { const context = vector.context as GltfTsGeometryContext; const result=inferGltfAxisAlignedBounds(context); if (JSON.stringify(result.value??null)!==JSON.stringify(vector.value)||JSON.stringify(inferGltfAxisAlignedBounds(context).value)!==JSON.stringify(result.value)) throw new Error('axis-aligned-bounds'); }
  console.log("[DEBUG] Original GLTF 💡️inferences/📦️size/↔️axis-aligned-bounds vectors=" + vectors.vectors.length);
});
