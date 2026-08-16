/** ↩️ unbind-primitive-attribute: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfUnbindPrimitiveAttribute, type GltfUnbindPrimitiveAttributePayload } from '../../unbind-primitive-attribute/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfUnbindPrimitiveAttributeInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfUnbindPrimitiveAttributeInverse = (base: GltfSnapshot, payload: GltfUnbindPrimitiveAttributePayload): GltfUnbindPrimitiveAttributeInverseResult => { const applied = applyGltfUnbindPrimitiveAttribute(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
