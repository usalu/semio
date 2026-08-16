/** ↩️ move-morph-target-attribute: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMoveMorphTargetAttribute, type GltfMoveMorphTargetAttributePayload } from '../../move-morph-target-attribute/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfMoveMorphTargetAttributeInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfMoveMorphTargetAttributeInverse = (base: GltfSnapshot, payload: GltfMoveMorphTargetAttributePayload): GltfMoveMorphTargetAttributeInverseResult => { const applied = applyGltfMoveMorphTargetAttribute(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
