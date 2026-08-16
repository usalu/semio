/** 🔺️ reparent-node emits its direct sparse document diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReparentNode, type GltfReparentNodePayload } from '../../reparent-node/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfReparentNodeDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfReparentNodeDiff = (base: GltfSnapshot, payload: GltfReparentNodePayload): GltfReparentNodeDiffResult => { const applied = applyGltfReparentNode(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
