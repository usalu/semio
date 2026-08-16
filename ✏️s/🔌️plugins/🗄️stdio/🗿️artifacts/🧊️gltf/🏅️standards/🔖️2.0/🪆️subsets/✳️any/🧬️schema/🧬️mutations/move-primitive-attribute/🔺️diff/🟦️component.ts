/** 🔺️ move-primitive-attribute: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMovePrimitiveAttribute, type GltfMovePrimitiveAttributePayload } from '../../move-primitive-attribute/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfMovePrimitiveAttributeDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfMovePrimitiveAttributeDiff = (base: GltfSnapshot, payload: GltfMovePrimitiveAttributePayload): GltfMovePrimitiveAttributeDiffResult => { const applied = applyGltfMovePrimitiveAttribute(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
