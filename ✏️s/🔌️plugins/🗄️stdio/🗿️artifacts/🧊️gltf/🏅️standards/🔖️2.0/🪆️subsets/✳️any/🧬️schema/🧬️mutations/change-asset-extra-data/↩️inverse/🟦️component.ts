/** ↩️ change-asset-extra-data direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeAssetExtraData, type GltfChangeAssetExtraDataPayload } from '../../change-asset-extra-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfChangeAssetExtraDataInverse = (base: GltfSnapshot, payload: GltfChangeAssetExtraDataPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfChangeAssetExtraData(base, payload); return applied.accepted ? { accepted: true, inverse: { asset: { extras: base.document.asset.extras ?? null } }, touchedPaths: ["document/asset/extras"] } : applied; };
