/** ↩️ delete-primitive: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfDeletePrimitive, type GltfDeletePrimitivePayload } from '../../delete-primitive/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfDeletePrimitiveInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfDeletePrimitiveInverse = (base: GltfSnapshot, payload: GltfDeletePrimitivePayload): GltfDeletePrimitiveInverseResult => { const applied = applyGltfDeletePrimitive(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
