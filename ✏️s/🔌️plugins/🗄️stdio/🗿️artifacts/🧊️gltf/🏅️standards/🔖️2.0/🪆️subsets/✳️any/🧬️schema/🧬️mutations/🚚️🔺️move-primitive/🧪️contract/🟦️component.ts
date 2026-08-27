/** 🧪️ Focused move-primitive mutation-law probe. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMovePrimitive, type GltfMovePrimitivePayload } from '../../move-primitive/🟦️component.ts';
import { deriveGltfMovePrimitiveDiff } from '../../move-primitive/🔺️diff/🟦️component.ts';
import { deriveGltfMovePrimitiveInverse } from '../../move-primitive/↩️inverse/🟦️component.ts';
export const assertGltfMovePrimitiveLaws = (base: GltfSnapshot, payload: GltfMovePrimitivePayload) => { const applied = applyGltfMovePrimitive(base, payload); if (!applied.accepted) return applied; const replay = applyGltfMovePrimitive(base, payload); const direct = deriveGltfMovePrimitiveDiff(base, payload); const undo = deriveGltfMovePrimitiveInverse(base, payload); if (!replay.accepted || !direct.accepted || !undo.accepted || JSON.stringify(applied.snapshot) !== JSON.stringify(replay.snapshot) || JSON.stringify(applied.diff) !== JSON.stringify(replay.diff) || JSON.stringify(applied.touchedPaths) !== JSON.stringify(undo.touchedPaths)) throw new Error('move-primitive violates replay, direct-diff, or undo determinism'); return { applied, direct, undo }; };
