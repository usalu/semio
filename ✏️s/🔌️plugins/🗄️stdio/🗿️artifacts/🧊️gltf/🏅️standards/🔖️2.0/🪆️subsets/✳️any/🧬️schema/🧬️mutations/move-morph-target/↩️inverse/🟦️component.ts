/** ↩️ move-morph-target: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMoveMorphTarget, type GltfMoveMorphTargetPayload } from '../../move-morph-target/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfMoveMorphTargetInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfMoveMorphTargetInverse = (base: GltfSnapshot, payload: GltfMoveMorphTargetPayload): GltfMoveMorphTargetInverseResult => { const applied = applyGltfMoveMorphTarget(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
