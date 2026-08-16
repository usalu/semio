/** ↩️ bind-primitive-indices: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfBindPrimitiveIndices, type GltfBindPrimitiveIndicesPayload } from '../../bind-primitive-indices/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfBindPrimitiveIndicesInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfBindPrimitiveIndicesInverse = (base: GltfSnapshot, payload: GltfBindPrimitiveIndicesPayload): GltfBindPrimitiveIndicesInverseResult => { const applied = applyGltfBindPrimitiveIndices(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
