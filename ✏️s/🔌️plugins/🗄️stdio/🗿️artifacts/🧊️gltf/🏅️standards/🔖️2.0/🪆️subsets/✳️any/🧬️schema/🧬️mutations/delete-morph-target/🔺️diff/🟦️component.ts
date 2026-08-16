/** 🔺️ delete-morph-target: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfDeleteMorphTarget, type GltfDeleteMorphTargetPayload } from '../../delete-morph-target/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfDeleteMorphTargetDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfDeleteMorphTargetDiff = (base: GltfSnapshot, payload: GltfDeleteMorphTargetPayload): GltfDeleteMorphTargetDiffResult => { const applied = applyGltfDeleteMorphTarget(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
