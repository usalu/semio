/** ↩️ reorder-primitive-attributes: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReorderPrimitiveAttributes, type GltfReorderPrimitiveAttributesPayload } from '../../reorder-primitive-attributes/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfReorderPrimitiveAttributesInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfReorderPrimitiveAttributesInverse = (base: GltfSnapshot, payload: GltfReorderPrimitiveAttributesPayload): GltfReorderPrimitiveAttributesInverseResult => { const applied = applyGltfReorderPrimitiveAttributes(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
