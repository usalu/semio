/** ↩️ change-document-extra-data direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeDocumentExtraData, type GltfChangeDocumentExtraDataPayload } from '../../change-document-extra-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfChangeDocumentExtraDataInverse = (base: GltfSnapshot, payload: GltfChangeDocumentExtraDataPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfChangeDocumentExtraData(base, payload); return applied.accepted ? { accepted: true, inverse: { extras: base.document.extras ?? null }, touchedPaths: ["document/extras"] } : applied; };
