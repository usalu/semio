/** 🧪️ Focused create-morph-target mutation-law probe. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️.ts';
import { applyGltfCreateMorphTarget, type GltfCreateMorphTargetPayload } from './🟦️';
import { deriveGltfCreateMorphTargetDiff } from './🟦️';
import { deriveGltfCreateMorphTargetInverse } from './🟦️';
export const assertGltfCreateMorphTargetLaws = (base: GltfSnapshot, payload: GltfCreateMorphTargetPayload) => { const applied = applyGltfCreateMorphTarget(base, payload); if (!applied.accepted) return applied; const replay = applyGltfCreateMorphTarget(base, payload); const direct = deriveGltfCreateMorphTargetDiff(base, payload); const undo = deriveGltfCreateMorphTargetInverse(base, payload); if (!replay.accepted || !direct.accepted || !undo.accepted || JSON.stringify(applied.snapshot) !== JSON.stringify(replay.snapshot) || JSON.stringify(applied.diff) !== JSON.stringify(replay.diff) || JSON.stringify(applied.touchedPaths) !== JSON.stringify(undo.touchedPaths)) throw new Error('create-morph-target violates replay, direct-diff, or undo determinism'); return { applied, direct, undo }; };
