/** 🧪️ Focused change-mesh-name mutation-law probe. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️.ts';
import { applyGltfChangeMeshName, type GltfChangeMeshNamePayload } from './🟦️';
import { deriveGltfChangeMeshNameDiff } from './🟦️';
import { deriveGltfChangeMeshNameInverse } from './🟦️';
export const assertGltfChangeMeshNameLaws = (base: GltfSnapshot, payload: GltfChangeMeshNamePayload) => { const applied = applyGltfChangeMeshName(base, payload); if (!applied.accepted) return applied; const replay = applyGltfChangeMeshName(base, payload); const direct = deriveGltfChangeMeshNameDiff(base, payload); const undo = deriveGltfChangeMeshNameInverse(base, payload); if (!replay.accepted || !direct.accepted || !undo.accepted || JSON.stringify(applied.snapshot) !== JSON.stringify(replay.snapshot) || JSON.stringify(applied.diff) !== JSON.stringify(replay.diff) || JSON.stringify(applied.touchedPaths) !== JSON.stringify(undo.touchedPaths)) throw new Error('change-mesh-name violates replay, direct-diff, or undo determinism'); return { applied, direct, undo }; };
