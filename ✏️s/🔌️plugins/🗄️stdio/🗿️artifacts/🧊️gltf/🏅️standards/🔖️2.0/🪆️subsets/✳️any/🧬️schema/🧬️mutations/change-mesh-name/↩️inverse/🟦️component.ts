/** ↩️ change-mesh-name: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeMeshName, type GltfChangeMeshNamePayload } from '../../change-mesh-name/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangeMeshNameInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangeMeshNameInverse = (base: GltfSnapshot, payload: GltfChangeMeshNamePayload): GltfChangeMeshNameInverseResult => { const applied = applyGltfChangeMeshName(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
