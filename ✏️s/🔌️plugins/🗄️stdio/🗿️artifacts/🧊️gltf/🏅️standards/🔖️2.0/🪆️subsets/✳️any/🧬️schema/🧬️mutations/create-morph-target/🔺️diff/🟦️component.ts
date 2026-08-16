/** 🔺️ create-morph-target: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfCreateMorphTarget, type GltfCreateMorphTargetPayload } from '../../create-morph-target/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfCreateMorphTargetDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfCreateMorphTargetDiff = (base: GltfSnapshot, payload: GltfCreateMorphTargetPayload): GltfCreateMorphTargetDiffResult => { const applied = applyGltfCreateMorphTarget(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
