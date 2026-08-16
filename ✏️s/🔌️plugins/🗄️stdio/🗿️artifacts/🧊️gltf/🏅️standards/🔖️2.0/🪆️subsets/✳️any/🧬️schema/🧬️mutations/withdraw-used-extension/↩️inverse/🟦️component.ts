/** ↩️ withdraw-used-extension direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfWithdrawUsedExtension, type GltfWithdrawUsedExtensionPayload } from '../../withdraw-used-extension/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfWithdrawUsedExtensionInverse = (base: GltfSnapshot, payload: GltfWithdrawUsedExtensionPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfWithdrawUsedExtension(base, payload); return applied.accepted ? { accepted: true, inverse: { extensionsUsed: [...base.document.extensionsUsed] }, touchedPaths: ["document/extensionsUsed"] } : applied; };
