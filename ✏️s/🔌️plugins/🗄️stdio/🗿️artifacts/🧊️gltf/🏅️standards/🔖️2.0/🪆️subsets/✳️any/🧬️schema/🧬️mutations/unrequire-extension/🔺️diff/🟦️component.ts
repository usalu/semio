/** 🔺️ unrequire-extension direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfUnrequireExtension, type GltfUnrequireExtensionPayload } from '../../unrequire-extension/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfUnrequireExtensionDiff = (base: GltfSnapshot, payload: GltfUnrequireExtensionPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfUnrequireExtension(base, payload); return applied.accepted ? { accepted: true, diff: { extensionsRequired: base.document.extensionsRequired.filter(value => value !== payload.extension) }, touchedPaths: GltfUnrequireExtensionDescriptor.touchedPaths } : applied; };
export const GltfUnrequireExtensionDescriptor = { id: 's.stdio.gltf.mutation.unrequire-extension.v1', touchedPaths: ["document/extensionsRequired"] } as const;
