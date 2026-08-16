/** 🔺️ reorder-required-extensions direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReorderRequiredExtensions, type GltfReorderRequiredExtensionsPayload } from '../../reorder-required-extensions/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfReorderRequiredExtensionsDiff = (base: GltfSnapshot, payload: GltfReorderRequiredExtensionsPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfReorderRequiredExtensions(base, payload); return applied.accepted ? { accepted: true, diff: { extensionsRequired: [...payload.order] }, touchedPaths: GltfReorderRequiredExtensionsDescriptor.touchedPaths } : applied; };
export const GltfReorderRequiredExtensionsDescriptor = { id: 's.stdio.gltf.mutation.reorder-required-extensions.v1', touchedPaths: ["document/extensionsRequired"] } as const;
