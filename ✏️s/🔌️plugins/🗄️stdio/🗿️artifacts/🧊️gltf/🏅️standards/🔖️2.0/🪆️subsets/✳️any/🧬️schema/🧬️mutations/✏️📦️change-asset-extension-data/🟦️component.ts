/** 🦠️ change-asset-extension-data executable glTF command. */
import type { GltfJson, GltfSnapshot } from '../../📸️snapshot/🟦️component.ts';
import { clone, reject, run, same, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️component.ts';
export const GltfChangeAssetExtensionDataDescriptor = { id: 's.stdio.gltf.mutation.change-asset-extension-data.v1', version: 1, touchedPaths: ["document/asset/extensions"], referencePolicy: 'none' } as const;
export interface GltfChangeAssetExtensionDataPayload { data: GltfJson | null }
export const validateGltfChangeAssetExtensionData = (payload: GltfChangeAssetExtensionDataPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { if (same(payload.data, base.document.asset.extensions ?? null)) return reject('gltf.mutation.no-observable-change', 'document/asset/extensions', 'value already has this value'); return undefined; };
export const applyGltfChangeAssetExtensionData = (base: GltfSnapshot, payload: GltfChangeAssetExtensionDataPayload): GltfLeafResult => run(base, payload, validateGltfChangeAssetExtensionData, (next, payload) => { next.document.asset.extensions = payload.data ?? undefined; });
