/** 🔺️ change-mesh-morph-weights: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeMeshMorphWeights, type GltfChangeMeshMorphWeightsPayload } from '../../change-mesh-morph-weights/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangeMeshMorphWeightsDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangeMeshMorphWeightsDiff = (base: GltfSnapshot, payload: GltfChangeMeshMorphWeightsPayload): GltfChangeMeshMorphWeightsDiffResult => { const applied = applyGltfChangeMeshMorphWeights(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
