/** ↩️ unbind-morph-target-attribute: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfUnbindMorphTargetAttribute, type GltfUnbindMorphTargetAttributePayload } from '../../unbind-morph-target-attribute/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfUnbindMorphTargetAttributeInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfUnbindMorphTargetAttributeInverse = (base: GltfSnapshot, payload: GltfUnbindMorphTargetAttributePayload): GltfUnbindMorphTargetAttributeInverseResult => { const applied = applyGltfUnbindMorphTargetAttribute(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
