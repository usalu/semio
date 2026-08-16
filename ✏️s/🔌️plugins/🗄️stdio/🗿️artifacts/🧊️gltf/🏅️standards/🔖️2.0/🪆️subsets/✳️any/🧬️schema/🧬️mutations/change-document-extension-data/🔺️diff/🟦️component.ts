/** 🔺️ change-document-extension-data direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeDocumentExtensionData, type GltfChangeDocumentExtensionDataPayload } from '../../change-document-extension-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfChangeDocumentExtensionDataDiff = (base: GltfSnapshot, payload: GltfChangeDocumentExtensionDataPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfChangeDocumentExtensionData(base, payload); return applied.accepted ? { accepted: true, diff: { extensions: payload.data }, touchedPaths: GltfChangeDocumentExtensionDataDescriptor.touchedPaths } : applied; };
export const GltfChangeDocumentExtensionDataDescriptor = { id: 's.stdio.gltf.mutation.change-document-extension-data.v1', touchedPaths: ["document/extensions"] } as const;
