/** 🦠️ change-document-extra-data executable glTF command. */
import type { GltfJson, GltfSnapshot } from '../../📸️snapshot/🟦️.ts';
import { clone, reject, run, same, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️.ts';
export const GltfChangeDocumentExtraDataDescriptor = { id: 's.stdio.gltf.mutation.change-document-extra-data.v1', version: 1, touchedPaths: ["document/extras"], referencePolicy: 'none' } as const;
export interface GltfChangeDocumentExtraDataPayload { data: GltfJson | null }
export const validateGltfChangeDocumentExtraData = (payload: GltfChangeDocumentExtraDataPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { if (same(payload.data, base.document.extras ?? null)) return reject('gltf.mutation.no-observable-change', 'document/extras', 'value already has this value'); return undefined; };
export const applyGltfChangeDocumentExtraData = (base: GltfSnapshot, payload: GltfChangeDocumentExtraDataPayload): GltfLeafResult => run(base, payload, validateGltfChangeDocumentExtraData, (next, payload) => { next.document.extras = payload.data ?? undefined; });
