/** 🧪️ Focused move-morph-target mutation-law probe. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️.ts';
import { applyGltfMoveMorphTarget, type GltfMoveMorphTargetPayload } from '../../move-morph-target/🟦️.ts';
import { deriveGltfMoveMorphTargetDiff } from '../../move-morph-target/🔺️diff/🟦️.ts';
import { deriveGltfMoveMorphTargetInverse } from '../../move-morph-target/↩️inverse/🟦️.ts';
export const assertGltfMoveMorphTargetLaws = (base: GltfSnapshot, payload: GltfMoveMorphTargetPayload) => { const applied = applyGltfMoveMorphTarget(base, payload); if (!applied.accepted) return applied; const replay = applyGltfMoveMorphTarget(base, payload); const direct = deriveGltfMoveMorphTargetDiff(base, payload); const undo = deriveGltfMoveMorphTargetInverse(base, payload); if (!replay.accepted || !direct.accepted || !undo.accepted || JSON.stringify(applied.snapshot) !== JSON.stringify(replay.snapshot) || JSON.stringify(applied.diff) !== JSON.stringify(replay.diff) || JSON.stringify(applied.touchedPaths) !== JSON.stringify(undo.touchedPaths)) throw new Error('move-morph-target violates replay, direct-diff, or undo determinism'); return { applied, direct, undo }; };
