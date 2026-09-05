/** 🧪️ Mutation-law probe for move-node-child. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️.ts';
import { applyGltfMoveNodeChild, type GltfMoveNodeChildPayload } from './🟦️';
import { deriveGltfMoveNodeChildDiff } from './🟦️';
import { deriveGltfMoveNodeChildInverse } from './🟦️';
export const assertGltfMoveNodeChildLaws = (base: GltfSnapshot, payload: GltfMoveNodeChildPayload) => { const first = applyGltfMoveNodeChild(base, payload); if (!first.accepted) return first; const replay = applyGltfMoveNodeChild(base, payload); if (!replay.accepted || JSON.stringify(first.snapshot) !== JSON.stringify(replay.snapshot) || JSON.stringify(first.diff) !== JSON.stringify(replay.diff)) throw new Error('move-node-child replay is non-deterministic'); const direct = deriveGltfMoveNodeChildDiff(base, payload); const inverse = deriveGltfMoveNodeChildInverse(base, payload); if (!direct.accepted || !inverse.accepted || JSON.stringify(direct.touchedPaths) !== JSON.stringify(first.touchedPaths) || JSON.stringify(inverse.touchedPaths) !== JSON.stringify(first.touchedPaths)) throw new Error('move-node-child diff or inverse law failed'); return { first, direct, inverse }; };
