/** 🔺️ change-asset-extension-data direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeAssetExtensionData, type GltfChangeAssetExtensionDataPayload } from '../../change-asset-extension-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfChangeAssetExtensionDataDiff = (base: GltfSnapshot, payload: GltfChangeAssetExtensionDataPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfChangeAssetExtensionData(base, payload); return applied.accepted ? { accepted: true, diff: { asset: { extensions: payload.data } }, touchedPaths: GltfChangeAssetExtensionDataDescriptor.touchedPaths } : applied; };
export const GltfChangeAssetExtensionDataDescriptor = { id: 's.stdio.gltf.mutation.change-asset-extension-data.v1', touchedPaths: ["document/asset/extensions"] } as const;
