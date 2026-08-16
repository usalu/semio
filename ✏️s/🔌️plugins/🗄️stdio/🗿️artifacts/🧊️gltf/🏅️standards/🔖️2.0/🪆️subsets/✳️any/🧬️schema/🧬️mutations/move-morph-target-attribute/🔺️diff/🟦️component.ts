/** 🔺️ move-morph-target-attribute: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMoveMorphTargetAttribute, type GltfMoveMorphTargetAttributePayload } from '../../move-morph-target-attribute/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfMoveMorphTargetAttributeDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfMoveMorphTargetAttributeDiff = (base: GltfSnapshot, payload: GltfMoveMorphTargetAttributePayload): GltfMoveMorphTargetAttributeDiffResult => { const applied = applyGltfMoveMorphTargetAttribute(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
