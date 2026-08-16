/** 🔺️ require-extension direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfRequireExtension, type GltfRequireExtensionPayload } from '../../require-extension/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfRequireExtensionDiff = (base: GltfSnapshot, payload: GltfRequireExtensionPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfRequireExtension(base, payload); return applied.accepted ? { accepted: true, diff: { extensionsRequired: [...base.document.extensionsRequired.slice(0, payload.position), payload.extension, ...base.document.extensionsRequired.slice(payload.position)] }, touchedPaths: GltfRequireExtensionDescriptor.touchedPaths } : applied; };
export const GltfRequireExtensionDescriptor = { id: 's.stdio.gltf.mutation.require-extension.v1', touchedPaths: ["document/extensionsRequired"] } as const;
