/** ↩️ move-used-extension direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMoveUsedExtension, type GltfMoveUsedExtensionPayload } from '../../move-used-extension/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfMoveUsedExtensionInverse = (base: GltfSnapshot, payload: GltfMoveUsedExtensionPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfMoveUsedExtension(base, payload); return applied.accepted ? { accepted: true, inverse: { extensionsUsed: [...base.document.extensionsUsed] }, touchedPaths: ["document/extensionsUsed"] } : applied; };
