/** ↩️ change-primitive-extra-data: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangePrimitiveExtraData, type GltfChangePrimitiveExtraDataPayload } from '../../change-primitive-extra-data/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangePrimitiveExtraDataInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangePrimitiveExtraDataInverse = (base: GltfSnapshot, payload: GltfChangePrimitiveExtraDataPayload): GltfChangePrimitiveExtraDataInverseResult => { const applied = applyGltfChangePrimitiveExtraData(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
