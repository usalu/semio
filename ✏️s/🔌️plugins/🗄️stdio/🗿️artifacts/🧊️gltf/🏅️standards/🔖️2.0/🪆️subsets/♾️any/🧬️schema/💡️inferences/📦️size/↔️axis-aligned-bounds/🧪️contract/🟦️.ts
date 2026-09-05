import vectors from './🔣️.json' with { type: 'json' };
import { inferGltfAxisAlignedBounds } from '../🟦️.ts';
for (const vector of vectors.vectors) { const result=inferGltfAxisAlignedBounds(vector.context); if (JSON.stringify(result.value??null)!==JSON.stringify(vector.value)||JSON.stringify(inferGltfAxisAlignedBounds(vector.context).value)!==JSON.stringify(result.value)) throw new Error('axis-aligned-bounds'); }
