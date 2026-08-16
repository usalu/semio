/** 🔺️ bind-primitive-attribute: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfBindPrimitiveAttribute, type GltfBindPrimitiveAttributePayload } from '../../bind-primitive-attribute/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfBindPrimitiveAttributeDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfBindPrimitiveAttributeDiff = (base: GltfSnapshot, payload: GltfBindPrimitiveAttributePayload): GltfBindPrimitiveAttributeDiffResult => { const applied = applyGltfBindPrimitiveAttribute(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
