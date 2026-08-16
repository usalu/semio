/** ↩️ reorder-morph-target-attributes: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReorderMorphTargetAttributes, type GltfReorderMorphTargetAttributesPayload } from '../../reorder-morph-target-attributes/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfReorderMorphTargetAttributesInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfReorderMorphTargetAttributesInverse = (base: GltfSnapshot, payload: GltfReorderMorphTargetAttributesPayload): GltfReorderMorphTargetAttributesInverseResult => { const applied = applyGltfReorderMorphTargetAttributes(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
