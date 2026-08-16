/** ↩️ reorder-used-extensions direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReorderUsedExtensions, type GltfReorderUsedExtensionsPayload } from '../../reorder-used-extensions/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfReorderUsedExtensionsInverse = (base: GltfSnapshot, payload: GltfReorderUsedExtensionsPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfReorderUsedExtensions(base, payload); return applied.accepted ? { accepted: true, inverse: { extensionsUsed: [...base.document.extensionsUsed] }, touchedPaths: ["document/extensionsUsed"] } : applied; };
