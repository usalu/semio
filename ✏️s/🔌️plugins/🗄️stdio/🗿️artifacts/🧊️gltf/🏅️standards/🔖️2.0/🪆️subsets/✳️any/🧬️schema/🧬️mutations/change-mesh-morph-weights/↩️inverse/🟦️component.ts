/** ↩️ change-mesh-morph-weights: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeMeshMorphWeights, type GltfChangeMeshMorphWeightsPayload } from '../../change-mesh-morph-weights/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangeMeshMorphWeightsInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangeMeshMorphWeightsInverse = (base: GltfSnapshot, payload: GltfChangeMeshMorphWeightsPayload): GltfChangeMeshMorphWeightsInverseResult => { const applied = applyGltfChangeMeshMorphWeights(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
