/** ↩️ move-primitive-attribute: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMovePrimitiveAttribute, type GltfMovePrimitiveAttributePayload } from '../../move-primitive-attribute/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfMovePrimitiveAttributeInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfMovePrimitiveAttributeInverse = (base: GltfSnapshot, payload: GltfMovePrimitiveAttributePayload): GltfMovePrimitiveAttributeInverseResult => { const applied = applyGltfMovePrimitiveAttribute(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
