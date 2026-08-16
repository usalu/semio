/** 🔺️ change-mesh-extension-data: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeMeshExtensionData, type GltfChangeMeshExtensionDataPayload } from '../../change-mesh-extension-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangeMeshExtensionDataDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangeMeshExtensionDataDiff = (base: GltfSnapshot, payload: GltfChangeMeshExtensionDataPayload): GltfChangeMeshExtensionDataDiffResult => { const applied = applyGltfChangeMeshExtensionData(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
