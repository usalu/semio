/** ↩️ change-document-extension-data direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeDocumentExtensionData, type GltfChangeDocumentExtensionDataPayload } from '../../change-document-extension-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfChangeDocumentExtensionDataInverse = (base: GltfSnapshot, payload: GltfChangeDocumentExtensionDataPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfChangeDocumentExtensionData(base, payload); return applied.accepted ? { accepted: true, inverse: { extensions: base.document.extensions ?? null }, touchedPaths: ["document/extensions"] } : applied; };
