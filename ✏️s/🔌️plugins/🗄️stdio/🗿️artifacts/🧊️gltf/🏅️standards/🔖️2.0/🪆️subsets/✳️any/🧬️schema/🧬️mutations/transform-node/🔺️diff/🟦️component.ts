/** 🔺️ transform-node emits its direct sparse document diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfTransformNode, type GltfTransformNodePayload } from '../../transform-node/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfTransformNodeDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfTransformNodeDiff = (base: GltfSnapshot, payload: GltfTransformNodePayload): GltfTransformNodeDiffResult => { const applied = applyGltfTransformNode(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
