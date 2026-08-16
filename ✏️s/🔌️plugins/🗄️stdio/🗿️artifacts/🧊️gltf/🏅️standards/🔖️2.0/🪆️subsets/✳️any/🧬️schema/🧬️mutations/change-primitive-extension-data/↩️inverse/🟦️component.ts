/** ↩️ change-primitive-extension-data: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangePrimitiveExtensionData, type GltfChangePrimitiveExtensionDataPayload } from '../../change-primitive-extension-data/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangePrimitiveExtensionDataInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangePrimitiveExtensionDataInverse = (base: GltfSnapshot, payload: GltfChangePrimitiveExtensionDataPayload): GltfChangePrimitiveExtensionDataInverseResult => { const applied = applyGltfChangePrimitiveExtensionData(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
