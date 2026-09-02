/** 🧪️ Mutation-law probe for bind-node-mesh. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️.ts';
import { applyGltfBindNodeMesh, type GltfBindNodeMeshPayload } from './🟦️';
import { deriveGltfBindNodeMeshDiff } from './🟦️';
import { deriveGltfBindNodeMeshInverse } from './🟦️';
export const assertGltfBindNodeMeshLaws = (base: GltfSnapshot, payload: GltfBindNodeMeshPayload) => { const first = applyGltfBindNodeMesh(base, payload); if (!first.accepted) return first; const replay = applyGltfBindNodeMesh(base, payload); if (!replay.accepted || JSON.stringify(first.snapshot) !== JSON.stringify(replay.snapshot) || JSON.stringify(first.diff) !== JSON.stringify(replay.diff)) throw new Error('bind-node-mesh replay is non-deterministic'); const direct = deriveGltfBindNodeMeshDiff(base, payload); const inverse = deriveGltfBindNodeMeshInverse(base, payload); if (!direct.accepted || !inverse.accepted || JSON.stringify(direct.touchedPaths) !== JSON.stringify(first.touchedPaths) || JSON.stringify(inverse.touchedPaths) !== JSON.stringify(first.touchedPaths)) throw new Error('bind-node-mesh diff or inverse law failed'); return { first, direct, inverse }; };
