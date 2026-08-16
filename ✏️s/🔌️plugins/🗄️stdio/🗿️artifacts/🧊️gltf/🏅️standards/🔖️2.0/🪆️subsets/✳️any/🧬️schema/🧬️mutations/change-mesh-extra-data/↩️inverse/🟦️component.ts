/** ↩️ change-mesh-extra-data: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeMeshExtraData, type GltfChangeMeshExtraDataPayload } from '../../change-mesh-extra-data/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangeMeshExtraDataInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangeMeshExtraDataInverse = (base: GltfSnapshot, payload: GltfChangeMeshExtraDataPayload): GltfChangeMeshExtraDataInverseResult => { const applied = applyGltfChangeMeshExtraData(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
