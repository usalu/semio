/** 🧪️ Focused change-mesh-extra-data mutation-law probe. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️.ts';
import { applyGltfChangeMeshExtraData, type GltfChangeMeshExtraDataPayload } from './🟦️';
import { deriveGltfChangeMeshExtraDataDiff } from './🟦️';
import { deriveGltfChangeMeshExtraDataInverse } from './🟦️';
export const assertGltfChangeMeshExtraDataLaws = (base: GltfSnapshot, payload: GltfChangeMeshExtraDataPayload) => { const applied = applyGltfChangeMeshExtraData(base, payload); if (!applied.accepted) return applied; const replay = applyGltfChangeMeshExtraData(base, payload); const direct = deriveGltfChangeMeshExtraDataDiff(base, payload); const undo = deriveGltfChangeMeshExtraDataInverse(base, payload); if (!replay.accepted || !direct.accepted || !undo.accepted || JSON.stringify(applied.snapshot) !== JSON.stringify(replay.snapshot) || JSON.stringify(applied.diff) !== JSON.stringify(replay.diff) || JSON.stringify(applied.touchedPaths) !== JSON.stringify(undo.touchedPaths)) throw new Error('change-mesh-extra-data violates replay, direct-diff, or undo determinism'); return { applied, direct, undo }; };
