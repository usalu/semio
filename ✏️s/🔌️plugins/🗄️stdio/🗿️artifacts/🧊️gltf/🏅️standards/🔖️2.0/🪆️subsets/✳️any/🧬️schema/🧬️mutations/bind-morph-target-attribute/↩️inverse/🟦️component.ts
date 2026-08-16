/** ↩️ bind-morph-target-attribute: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfBindMorphTargetAttribute, type GltfBindMorphTargetAttributePayload } from '../../bind-morph-target-attribute/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfBindMorphTargetAttributeInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfBindMorphTargetAttributeInverse = (base: GltfSnapshot, payload: GltfBindMorphTargetAttributePayload): GltfBindMorphTargetAttributeInverseResult => { const applied = applyGltfBindMorphTargetAttribute(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
