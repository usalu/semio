/** ↩️ unrequire-extension direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfUnrequireExtension, type GltfUnrequireExtensionPayload } from '../../unrequire-extension/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfUnrequireExtensionInverse = (base: GltfSnapshot, payload: GltfUnrequireExtensionPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfUnrequireExtension(base, payload); return applied.accepted ? { accepted: true, inverse: { extensionsRequired: [...base.document.extensionsRequired] }, touchedPaths: ["document/extensionsRequired"] } : applied; };
