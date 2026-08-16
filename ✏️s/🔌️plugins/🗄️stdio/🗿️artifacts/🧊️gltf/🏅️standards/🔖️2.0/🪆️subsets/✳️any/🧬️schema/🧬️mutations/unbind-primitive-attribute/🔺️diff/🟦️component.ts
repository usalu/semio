/** 🔺️ unbind-primitive-attribute: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfUnbindPrimitiveAttribute, type GltfUnbindPrimitiveAttributePayload } from '../../unbind-primitive-attribute/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfUnbindPrimitiveAttributeDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfUnbindPrimitiveAttributeDiff = (base: GltfSnapshot, payload: GltfUnbindPrimitiveAttributePayload): GltfUnbindPrimitiveAttributeDiffResult => { const applied = applyGltfUnbindPrimitiveAttribute(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
