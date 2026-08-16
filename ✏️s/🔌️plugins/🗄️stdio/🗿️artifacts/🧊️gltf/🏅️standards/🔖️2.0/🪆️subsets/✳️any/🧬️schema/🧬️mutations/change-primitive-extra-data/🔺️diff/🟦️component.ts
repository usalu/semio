/** 🔺️ change-primitive-extra-data: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangePrimitiveExtraData, type GltfChangePrimitiveExtraDataPayload } from '../../change-primitive-extra-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangePrimitiveExtraDataDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangePrimitiveExtraDataDiff = (base: GltfSnapshot, payload: GltfChangePrimitiveExtraDataPayload): GltfChangePrimitiveExtraDataDiffResult => { const applied = applyGltfChangePrimitiveExtraData(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
