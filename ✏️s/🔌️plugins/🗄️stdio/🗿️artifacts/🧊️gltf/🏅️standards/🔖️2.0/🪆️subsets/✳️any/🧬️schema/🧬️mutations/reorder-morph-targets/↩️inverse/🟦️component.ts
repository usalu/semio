/** ↩️ reorder-morph-targets: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReorderMorphTargets, type GltfReorderMorphTargetsPayload } from '../../reorder-morph-targets/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfReorderMorphTargetsInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfReorderMorphTargetsInverse = (base: GltfSnapshot, payload: GltfReorderMorphTargetsPayload): GltfReorderMorphTargetsInverseResult => { const applied = applyGltfReorderMorphTargets(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
