/** 🧪️ Focused change-mesh-extension-data mutation-law probe. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️.ts';
import { applyGltfChangeMeshExtensionData, type GltfChangeMeshExtensionDataPayload } from './🟦️';
import { deriveGltfChangeMeshExtensionDataDiff } from './🟦️';
import { deriveGltfChangeMeshExtensionDataInverse } from './🟦️';
export const assertGltfChangeMeshExtensionDataLaws = (base: GltfSnapshot, payload: GltfChangeMeshExtensionDataPayload) => { const applied = applyGltfChangeMeshExtensionData(base, payload); if (!applied.accepted) return applied; const replay = applyGltfChangeMeshExtensionData(base, payload); const direct = deriveGltfChangeMeshExtensionDataDiff(base, payload); const undo = deriveGltfChangeMeshExtensionDataInverse(base, payload); if (!replay.accepted || !direct.accepted || !undo.accepted || JSON.stringify(applied.snapshot) !== JSON.stringify(replay.snapshot) || JSON.stringify(applied.diff) !== JSON.stringify(replay.diff) || JSON.stringify(applied.touchedPaths) !== JSON.stringify(undo.touchedPaths)) throw new Error('change-mesh-extension-data violates replay, direct-diff, or undo determinism'); return { applied, direct, undo }; };
