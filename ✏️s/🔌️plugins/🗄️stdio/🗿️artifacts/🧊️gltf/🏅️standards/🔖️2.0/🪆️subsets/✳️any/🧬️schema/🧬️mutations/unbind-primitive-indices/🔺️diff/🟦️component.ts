/** 🔺️ unbind-primitive-indices: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfUnbindPrimitiveIndices, type GltfUnbindPrimitiveIndicesPayload } from '../../unbind-primitive-indices/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfUnbindPrimitiveIndicesDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfUnbindPrimitiveIndicesDiff = (base: GltfSnapshot, payload: GltfUnbindPrimitiveIndicesPayload): GltfUnbindPrimitiveIndicesDiffResult => { const applied = applyGltfUnbindPrimitiveIndices(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
