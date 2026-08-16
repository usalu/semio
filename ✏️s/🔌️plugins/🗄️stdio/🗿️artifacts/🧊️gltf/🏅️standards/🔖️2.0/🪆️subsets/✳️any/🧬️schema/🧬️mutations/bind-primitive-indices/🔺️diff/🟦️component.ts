/** 🔺️ bind-primitive-indices: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfBindPrimitiveIndices, type GltfBindPrimitiveIndicesPayload } from '../../bind-primitive-indices/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfBindPrimitiveIndicesDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfBindPrimitiveIndicesDiff = (base: GltfSnapshot, payload: GltfBindPrimitiveIndicesPayload): GltfBindPrimitiveIndicesDiffResult => { const applied = applyGltfBindPrimitiveIndices(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
