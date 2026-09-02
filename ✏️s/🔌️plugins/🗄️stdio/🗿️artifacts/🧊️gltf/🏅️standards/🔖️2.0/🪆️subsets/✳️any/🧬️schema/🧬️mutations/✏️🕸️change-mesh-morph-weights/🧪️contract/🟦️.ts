/** 🧪️ Focused change-mesh-morph-weights mutation-law probe. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️.ts';
import { applyGltfChangeMeshMorphWeights, type GltfChangeMeshMorphWeightsPayload } from './🟦️';
import { deriveGltfChangeMeshMorphWeightsDiff } from './🟦️';
import { deriveGltfChangeMeshMorphWeightsInverse } from './🟦️';
export const assertGltfChangeMeshMorphWeightsLaws = (base: GltfSnapshot, payload: GltfChangeMeshMorphWeightsPayload) => { const applied = applyGltfChangeMeshMorphWeights(base, payload); if (!applied.accepted) return applied; const replay = applyGltfChangeMeshMorphWeights(base, payload); const direct = deriveGltfChangeMeshMorphWeightsDiff(base, payload); const undo = deriveGltfChangeMeshMorphWeightsInverse(base, payload); if (!replay.accepted || !direct.accepted || !undo.accepted || JSON.stringify(applied.snapshot) !== JSON.stringify(replay.snapshot) || JSON.stringify(applied.diff) !== JSON.stringify(replay.diff) || JSON.stringify(applied.touchedPaths) !== JSON.stringify(undo.touchedPaths)) throw new Error('change-mesh-morph-weights violates replay, direct-diff, or undo determinism'); return { applied, direct, undo }; };
