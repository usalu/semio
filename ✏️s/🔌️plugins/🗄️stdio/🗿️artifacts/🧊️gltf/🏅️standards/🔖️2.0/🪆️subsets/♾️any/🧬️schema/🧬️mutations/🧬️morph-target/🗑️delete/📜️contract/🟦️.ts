/** 🧪️ Focused delete-morph-target mutation-law probe. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️.ts';
import { applyGltfDeleteMorphTarget, type GltfDeleteMorphTargetPayload } from './🟦️';
import { deriveGltfDeleteMorphTargetDiff } from './🟦️';
import { deriveGltfDeleteMorphTargetInverse } from './🟦️';
export const assertGltfDeleteMorphTargetLaws = (base: GltfSnapshot, payload: GltfDeleteMorphTargetPayload) => { const applied = applyGltfDeleteMorphTarget(base, payload); if (!applied.accepted) return applied; const replay = applyGltfDeleteMorphTarget(base, payload); const direct = deriveGltfDeleteMorphTargetDiff(base, payload); const undo = deriveGltfDeleteMorphTargetInverse(base, payload); if (!replay.accepted || !direct.accepted || !undo.accepted || JSON.stringify(applied.snapshot) !== JSON.stringify(replay.snapshot) || JSON.stringify(applied.diff) !== JSON.stringify(replay.diff) || JSON.stringify(applied.touchedPaths) !== JSON.stringify(undo.touchedPaths)) throw new Error('delete-morph-target violates replay, direct-diff, or undo determinism'); return { applied, direct, undo }; };
