/** ↩️ move-required-extension direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMoveRequiredExtension, type GltfMoveRequiredExtensionPayload } from '../../move-required-extension/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfMoveRequiredExtensionInverse = (base: GltfSnapshot, payload: GltfMoveRequiredExtensionPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfMoveRequiredExtension(base, payload); return applied.accepted ? { accepted: true, inverse: { extensionsRequired: [...base.document.extensionsRequired] }, touchedPaths: ["document/extensionsRequired"] } : applied; };
