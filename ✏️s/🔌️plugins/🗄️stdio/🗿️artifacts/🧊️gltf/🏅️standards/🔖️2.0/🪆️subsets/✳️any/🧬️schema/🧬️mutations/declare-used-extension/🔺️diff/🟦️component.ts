/** 🔺️ declare-used-extension direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfDeclareUsedExtension, type GltfDeclareUsedExtensionPayload } from '../../declare-used-extension/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfDeclareUsedExtensionDiff = (base: GltfSnapshot, payload: GltfDeclareUsedExtensionPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfDeclareUsedExtension(base, payload); return applied.accepted ? { accepted: true, diff: { extensionsUsed: [...base.document.extensionsUsed.slice(0, payload.position), payload.extension, ...base.document.extensionsUsed.slice(payload.position)] }, touchedPaths: GltfDeclareUsedExtensionDescriptor.touchedPaths } : applied; };
export const GltfDeclareUsedExtensionDescriptor = { id: 's.stdio.gltf.mutation.declare-used-extension.v1', touchedPaths: ["document/extensionsUsed"] } as const;
