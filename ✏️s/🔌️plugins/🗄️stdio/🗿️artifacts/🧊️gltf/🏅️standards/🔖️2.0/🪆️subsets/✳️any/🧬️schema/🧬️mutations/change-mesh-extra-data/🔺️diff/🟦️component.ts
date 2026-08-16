/** 🔺️ change-mesh-extra-data: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeMeshExtraData, type GltfChangeMeshExtraDataPayload } from '../../change-mesh-extra-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangeMeshExtraDataDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangeMeshExtraDataDiff = (base: GltfSnapshot, payload: GltfChangeMeshExtraDataPayload): GltfChangeMeshExtraDataDiffResult => { const applied = applyGltfChangeMeshExtraData(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
