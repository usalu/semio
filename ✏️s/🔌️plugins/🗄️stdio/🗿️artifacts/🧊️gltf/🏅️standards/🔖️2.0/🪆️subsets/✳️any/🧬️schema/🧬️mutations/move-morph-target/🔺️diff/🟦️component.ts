/** 🔺️ move-morph-target: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMoveMorphTarget, type GltfMoveMorphTargetPayload } from '../../move-morph-target/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfMoveMorphTargetDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfMoveMorphTargetDiff = (base: GltfSnapshot, payload: GltfMoveMorphTargetPayload): GltfMoveMorphTargetDiffResult => { const applied = applyGltfMoveMorphTarget(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
