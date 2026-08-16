/** ↩️ reorder-primitives: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReorderPrimitives, type GltfReorderPrimitivesPayload } from '../../reorder-primitives/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfReorderPrimitivesInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfReorderPrimitivesInverse = (base: GltfSnapshot, payload: GltfReorderPrimitivesPayload): GltfReorderPrimitivesInverseResult => { const applied = applyGltfReorderPrimitives(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
