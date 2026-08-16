/** 🔺️ change-asset-extra-data direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeAssetExtraData, type GltfChangeAssetExtraDataPayload } from '../../change-asset-extra-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfChangeAssetExtraDataDiff = (base: GltfSnapshot, payload: GltfChangeAssetExtraDataPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfChangeAssetExtraData(base, payload); return applied.accepted ? { accepted: true, diff: { asset: { extras: payload.data } }, touchedPaths: GltfChangeAssetExtraDataDescriptor.touchedPaths } : applied; };
export const GltfChangeAssetExtraDataDescriptor = { id: 's.stdio.gltf.mutation.change-asset-extra-data.v1', touchedPaths: ["document/asset/extras"] } as const;
