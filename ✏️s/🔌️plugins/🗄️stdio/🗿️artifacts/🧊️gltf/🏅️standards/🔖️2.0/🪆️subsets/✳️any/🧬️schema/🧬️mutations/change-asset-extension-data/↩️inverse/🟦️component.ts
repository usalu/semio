/** ↩️ change-asset-extension-data direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeAssetExtensionData, type GltfChangeAssetExtensionDataPayload } from '../../change-asset-extension-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfChangeAssetExtensionDataInverse = (base: GltfSnapshot, payload: GltfChangeAssetExtensionDataPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfChangeAssetExtensionData(base, payload); return applied.accepted ? { accepted: true, inverse: { asset: { extensions: base.document.asset.extensions ?? null } }, touchedPaths: ["document/asset/extensions"] } : applied; };
