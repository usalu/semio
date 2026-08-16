/** 🔺️ reorder-morph-target-attributes: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReorderMorphTargetAttributes, type GltfReorderMorphTargetAttributesPayload } from '../../reorder-morph-target-attributes/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfReorderMorphTargetAttributesDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfReorderMorphTargetAttributesDiff = (base: GltfSnapshot, payload: GltfReorderMorphTargetAttributesPayload): GltfReorderMorphTargetAttributesDiffResult => { const applied = applyGltfReorderMorphTargetAttributes(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
