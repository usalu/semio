/** 🔺️ change-document-extra-data direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeDocumentExtraData, type GltfChangeDocumentExtraDataPayload } from '../../change-document-extra-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfChangeDocumentExtraDataDiff = (base: GltfSnapshot, payload: GltfChangeDocumentExtraDataPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfChangeDocumentExtraData(base, payload); return applied.accepted ? { accepted: true, diff: { extras: payload.data }, touchedPaths: GltfChangeDocumentExtraDataDescriptor.touchedPaths } : applied; };
export const GltfChangeDocumentExtraDataDescriptor = { id: 's.stdio.gltf.mutation.change-document-extra-data.v1', touchedPaths: ["document/extras"] } as const;
