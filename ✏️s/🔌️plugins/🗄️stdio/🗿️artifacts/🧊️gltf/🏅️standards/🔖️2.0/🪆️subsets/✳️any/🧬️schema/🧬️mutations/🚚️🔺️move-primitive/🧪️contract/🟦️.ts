/** 🧪️ Focused move-primitive mutation-law probe. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️.ts';
import { applyGltfMovePrimitive, type GltfMovePrimitivePayload } from './🟦️';
import { deriveGltfMovePrimitiveDiff } from './🟦️';
import { deriveGltfMovePrimitiveInverse } from './🟦️';
export const assertGltfMovePrimitiveLaws = (base: GltfSnapshot, payload: GltfMovePrimitivePayload) => { const applied = applyGltfMovePrimitive(base, payload); if (!applied.accepted) return applied; const replay = applyGltfMovePrimitive(base, payload); const direct = deriveGltfMovePrimitiveDiff(base, payload); const undo = deriveGltfMovePrimitiveInverse(base, payload); if (!replay.accepted || !direct.accepted || !undo.accepted || JSON.stringify(applied.snapshot) !== JSON.stringify(replay.snapshot) || JSON.stringify(applied.diff) !== JSON.stringify(replay.diff) || JSON.stringify(applied.touchedPaths) !== JSON.stringify(undo.touchedPaths)) throw new Error('move-primitive violates replay, direct-diff, or undo determinism'); return { applied, direct, undo }; };
