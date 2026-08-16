/** ↩️ reparent-node derives the exact undo diff from its accepted base. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReparentNode, type GltfReparentNodePayload } from '../../reparent-node/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfReparentNodeInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfReparentNodeInverse = (base: GltfSnapshot, payload: GltfReparentNodePayload): GltfReparentNodeInverseResult => { const applied = applyGltfReparentNode(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
