/** ↩️ require-extension direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfRequireExtension, type GltfRequireExtensionPayload } from '../../require-extension/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfRequireExtensionInverse = (base: GltfSnapshot, payload: GltfRequireExtensionPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfRequireExtension(base, payload); return applied.accepted ? { accepted: true, inverse: { extensionsRequired: [...base.document.extensionsRequired] }, touchedPaths: ["document/extensionsRequired"] } : applied; };
