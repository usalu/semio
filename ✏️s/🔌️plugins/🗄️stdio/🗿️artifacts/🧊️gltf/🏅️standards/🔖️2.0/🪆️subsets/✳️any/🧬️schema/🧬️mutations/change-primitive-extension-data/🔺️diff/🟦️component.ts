/** 🔺️ change-primitive-extension-data: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangePrimitiveExtensionData, type GltfChangePrimitiveExtensionDataPayload } from '../../change-primitive-extension-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangePrimitiveExtensionDataDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangePrimitiveExtensionDataDiff = (base: GltfSnapshot, payload: GltfChangePrimitiveExtensionDataPayload): GltfChangePrimitiveExtensionDataDiffResult => { const applied = applyGltfChangePrimitiveExtensionData(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
