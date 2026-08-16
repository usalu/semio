/** ↩️ bind-primitive-attribute: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfBindPrimitiveAttribute, type GltfBindPrimitiveAttributePayload } from '../../bind-primitive-attribute/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfBindPrimitiveAttributeInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfBindPrimitiveAttributeInverse = (base: GltfSnapshot, payload: GltfBindPrimitiveAttributePayload): GltfBindPrimitiveAttributeInverseResult => { const applied = applyGltfBindPrimitiveAttribute(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
