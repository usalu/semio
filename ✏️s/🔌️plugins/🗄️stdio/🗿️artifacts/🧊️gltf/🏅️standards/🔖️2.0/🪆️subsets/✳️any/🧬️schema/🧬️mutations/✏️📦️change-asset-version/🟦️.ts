/** 🦠️ change-asset-version executable glTF command. */
import type { GltfJson, GltfSnapshot } from '../../📸️snapshot/🟦️.ts';
import { clone, reject, run, same, type GltfLeafResult, type GltfMutationRejection } from './🟦️';
export const GltfChangeAssetVersionDescriptor = { id: 's.stdio.gltf.mutation.change-asset-version.v1', version: 1, touchedPaths: ["document/asset/version"], referencePolicy: 'none' } as const;
export interface GltfChangeAssetVersionPayload { version: string }
export const validateGltfChangeAssetVersion = (payload: GltfChangeAssetVersionPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { if (!payload.version.trim()) return reject('gltf.mutation.invalid-asset-version', 'document/asset/version', 'version must be non-empty'); if (payload.version === base.document.asset.version) return reject('gltf.mutation.no-observable-change', 'document/asset/version', 'version already has this value'); return undefined; };
export const applyGltfChangeAssetVersion = (base: GltfSnapshot, payload: GltfChangeAssetVersionPayload): GltfLeafResult => run(base, payload, validateGltfChangeAssetVersion, (next, payload) => { next.document.asset.version = payload.version; });
