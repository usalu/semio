/** 📝 Canonical sparse diff text and its derived inverse/touched regions. */
import type { GltfDiff, GltfDiffDerivation } from '../🟦️.ts';
export interface GltfDiffTextDocument { text: string; value: GltfDiff; derivation: GltfDiffDerivation }
export type GltfDiffText = string;
