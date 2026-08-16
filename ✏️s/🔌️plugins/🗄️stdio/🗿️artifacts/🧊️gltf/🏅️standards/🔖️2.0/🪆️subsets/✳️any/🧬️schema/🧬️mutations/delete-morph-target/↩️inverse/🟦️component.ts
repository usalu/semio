/** ↩️ delete-morph-target: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfDeleteMorphTarget, type GltfDeleteMorphTargetPayload } from '../../delete-morph-target/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfDeleteMorphTargetInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfDeleteMorphTargetInverse = (base: GltfSnapshot, payload: GltfDeleteMorphTargetPayload): GltfDeleteMorphTargetInverseResult => { const applied = applyGltfDeleteMorphTarget(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
