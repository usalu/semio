/** ↩️ create-primitive: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfCreatePrimitive, type GltfCreatePrimitivePayload } from '../../create-primitive/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfCreatePrimitiveInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfCreatePrimitiveInverse = (base: GltfSnapshot, payload: GltfCreatePrimitivePayload): GltfCreatePrimitiveInverseResult => { const applied = applyGltfCreatePrimitive(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
