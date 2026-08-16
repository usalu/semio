/** ↩️ create-morph-target: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfCreateMorphTarget, type GltfCreateMorphTargetPayload } from '../../create-morph-target/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfCreateMorphTargetInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfCreateMorphTargetInverse = (base: GltfSnapshot, payload: GltfCreateMorphTargetPayload): GltfCreateMorphTargetInverseResult => { const applied = applyGltfCreateMorphTarget(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
