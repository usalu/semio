/** 🦠️ change-document-extension-data executable glTF command. */
import type { GltfJson, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { clone, reject, run, same, type GltfLeafResult, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const GltfChangeDocumentExtensionDataDescriptor = { id: 's.stdio.gltf.mutation.change-document-extension-data.v1', version: 1, touchedPaths: ["document/extensions"], referencePolicy: 'none' } as const;
export interface GltfChangeDocumentExtensionDataPayload { data: GltfJson | null }
export const validateGltfChangeDocumentExtensionData = (payload: GltfChangeDocumentExtensionDataPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { if (same(payload.data, base.document.extensions ?? null)) return reject('gltf.mutation.no-observable-change', 'document/extensions', 'value already has this value'); return undefined; };
export const applyGltfChangeDocumentExtensionData = (base: GltfSnapshot, payload: GltfChangeDocumentExtensionDataPayload): GltfLeafResult => run(base, payload, validateGltfChangeDocumentExtensionData, (next, payload) => { next.document.extensions = payload.data ?? undefined; });
