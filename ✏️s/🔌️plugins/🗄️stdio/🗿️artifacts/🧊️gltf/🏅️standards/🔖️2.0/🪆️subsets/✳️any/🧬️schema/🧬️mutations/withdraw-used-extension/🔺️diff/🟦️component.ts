/** 🔺️ withdraw-used-extension direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfWithdrawUsedExtension, type GltfWithdrawUsedExtensionPayload } from '../../withdraw-used-extension/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfWithdrawUsedExtensionDiff = (base: GltfSnapshot, payload: GltfWithdrawUsedExtensionPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfWithdrawUsedExtension(base, payload); return applied.accepted ? { accepted: true, diff: { extensionsUsed: base.document.extensionsUsed.filter(value => value !== payload.extension) }, touchedPaths: GltfWithdrawUsedExtensionDescriptor.touchedPaths } : applied; };
export const GltfWithdrawUsedExtensionDescriptor = { id: 's.stdio.gltf.mutation.withdraw-used-extension.v1', touchedPaths: ["document/extensionsUsed"] } as const;
