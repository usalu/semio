import vectors from '../🧫️fixtures/🔣️.json' with { type: 'json' };
import { inferGltfAxisAlignedBounds } from '../🟦️.ts';
import type { GltfTsGeometryContext } from '../../../🔨️geometry-core/🟦️.ts';
for (const vector of vectors.vectors) { const context = vector.context as GltfTsGeometryContext; const result=inferGltfAxisAlignedBounds(context); if (JSON.stringify(result.value??null)!==JSON.stringify(vector.value)||JSON.stringify(inferGltfAxisAlignedBounds(context).value)!==JSON.stringify(result.value)) throw new Error('axis-aligned-bounds'); }
