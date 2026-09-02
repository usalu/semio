/** 🦠️ change-asset-extra-data executable glTF command. */
import type { GltfJson, GltfSnapshot } from '../../📸️snapshot/🟦️.ts';
import { clone, reject, run, same, type GltfLeafResult, type GltfMutationRejection } from './🟦️';
export const GltfChangeAssetExtraDataDescriptor = { id: 's.stdio.gltf.mutation.change-asset-extra-data.v1', version: 1, touchedPaths: ["document/asset/extras"], referencePolicy: 'none' } as const;
export interface GltfChangeAssetExtraDataPayload { data: GltfJson | null }
export const validateGltfChangeAssetExtraData = (payload: GltfChangeAssetExtraDataPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { if (same(payload.data, base.document.asset.extras ?? null)) return reject('gltf.mutation.no-observable-change', 'document/asset/extras', 'value already has this value'); return undefined; };
export const applyGltfChangeAssetExtraData = (base: GltfSnapshot, payload: GltfChangeAssetExtraDataPayload): GltfLeafResult => run(base, payload, validateGltfChangeAssetExtraData, (next, payload) => { next.document.asset.extras = payload.data ?? undefined; });
