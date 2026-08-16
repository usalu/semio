/** 🔺️ move-used-extension direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMoveUsedExtension, type GltfMoveUsedExtensionPayload } from '../../move-used-extension/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfMoveUsedExtensionDiff = (base: GltfSnapshot, payload: GltfMoveUsedExtensionPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfMoveUsedExtension(base, payload); return applied.accepted ? { accepted: true, diff: { extensionsUsed: (() => { const values = [...base.document.extensionsUsed]; const value = values.splice(values.indexOf(payload.extension), 1)[0]!; values.splice(payload.position, 0, value); return values; })() }, touchedPaths: GltfMoveUsedExtensionDescriptor.touchedPaths } : applied; };
export const GltfMoveUsedExtensionDescriptor = { id: 's.stdio.gltf.mutation.move-used-extension.v1', touchedPaths: ["document/extensionsUsed"] } as const;
