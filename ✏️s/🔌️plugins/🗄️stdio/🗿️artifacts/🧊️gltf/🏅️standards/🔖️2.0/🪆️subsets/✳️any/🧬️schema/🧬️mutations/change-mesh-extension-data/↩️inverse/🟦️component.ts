/** ↩️ change-mesh-extension-data: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeMeshExtensionData, type GltfChangeMeshExtensionDataPayload } from '../../change-mesh-extension-data/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangeMeshExtensionDataInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangeMeshExtensionDataInverse = (base: GltfSnapshot, payload: GltfChangeMeshExtensionDataPayload): GltfChangeMeshExtensionDataInverseResult => { const applied = applyGltfChangeMeshExtensionData(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
