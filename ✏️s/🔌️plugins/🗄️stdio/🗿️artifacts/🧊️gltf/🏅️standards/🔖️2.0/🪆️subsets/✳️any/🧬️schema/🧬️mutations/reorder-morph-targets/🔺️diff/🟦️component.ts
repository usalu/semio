/** 🔺️ reorder-morph-targets: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReorderMorphTargets, type GltfReorderMorphTargetsPayload } from '../../reorder-morph-targets/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfReorderMorphTargetsDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfReorderMorphTargetsDiff = (base: GltfSnapshot, payload: GltfReorderMorphTargetsPayload): GltfReorderMorphTargetsDiffResult => { const applied = applyGltfReorderMorphTargets(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
