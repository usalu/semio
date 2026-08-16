/** 🔺️ move-required-extension direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMoveRequiredExtension, type GltfMoveRequiredExtensionPayload } from '../../move-required-extension/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfMoveRequiredExtensionDiff = (base: GltfSnapshot, payload: GltfMoveRequiredExtensionPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfMoveRequiredExtension(base, payload); return applied.accepted ? { accepted: true, diff: { extensionsRequired: (() => { const values = [...base.document.extensionsRequired]; const value = values.splice(values.indexOf(payload.extension), 1)[0]!; values.splice(payload.position, 0, value); return values; })() }, touchedPaths: GltfMoveRequiredExtensionDescriptor.touchedPaths } : applied; };
export const GltfMoveRequiredExtensionDescriptor = { id: 's.stdio.gltf.mutation.move-required-extension.v1', touchedPaths: ["document/extensionsRequired"] } as const;
