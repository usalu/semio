/** 🔺️ reorder-used-extensions direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReorderUsedExtensions, type GltfReorderUsedExtensionsPayload } from '../../reorder-used-extensions/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfReorderUsedExtensionsDiff = (base: GltfSnapshot, payload: GltfReorderUsedExtensionsPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfReorderUsedExtensions(base, payload); return applied.accepted ? { accepted: true, diff: { extensionsUsed: [...payload.order] }, touchedPaths: GltfReorderUsedExtensionsDescriptor.touchedPaths } : applied; };
export const GltfReorderUsedExtensionsDescriptor = { id: 's.stdio.gltf.mutation.reorder-used-extensions.v1', touchedPaths: ["document/extensionsUsed"] } as const;
