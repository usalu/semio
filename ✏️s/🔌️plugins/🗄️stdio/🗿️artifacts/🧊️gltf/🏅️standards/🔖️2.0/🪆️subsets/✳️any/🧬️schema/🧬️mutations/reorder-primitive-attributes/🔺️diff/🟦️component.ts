/** 🔺️ reorder-primitive-attributes: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReorderPrimitiveAttributes, type GltfReorderPrimitiveAttributesPayload } from '../../reorder-primitive-attributes/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfReorderPrimitiveAttributesDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfReorderPrimitiveAttributesDiff = (base: GltfSnapshot, payload: GltfReorderPrimitiveAttributesPayload): GltfReorderPrimitiveAttributesDiffResult => { const applied = applyGltfReorderPrimitiveAttributes(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
