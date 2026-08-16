/** ↩️ move-primitive: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMovePrimitive, type GltfMovePrimitivePayload } from '../../move-primitive/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfMovePrimitiveInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfMovePrimitiveInverse = (base: GltfSnapshot, payload: GltfMovePrimitivePayload): GltfMovePrimitiveInverseResult => { const applied = applyGltfMovePrimitive(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
