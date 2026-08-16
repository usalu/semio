/** ↩️ transform-node derives the exact undo diff from its accepted base. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfTransformNode, type GltfTransformNodePayload } from '../../transform-node/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfTransformNodeInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfTransformNodeInverse = (base: GltfSnapshot, payload: GltfTransformNodePayload): GltfTransformNodeInverseResult => { const applied = applyGltfTransformNode(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
