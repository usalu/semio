/** ↩️ reorder-required-extensions direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReorderRequiredExtensions, type GltfReorderRequiredExtensionsPayload } from '../../reorder-required-extensions/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfReorderRequiredExtensionsInverse = (base: GltfSnapshot, payload: GltfReorderRequiredExtensionsPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfReorderRequiredExtensions(base, payload); return applied.accepted ? { accepted: true, inverse: { extensionsRequired: [...base.document.extensionsRequired] }, touchedPaths: ["document/extensionsRequired"] } : applied; };
