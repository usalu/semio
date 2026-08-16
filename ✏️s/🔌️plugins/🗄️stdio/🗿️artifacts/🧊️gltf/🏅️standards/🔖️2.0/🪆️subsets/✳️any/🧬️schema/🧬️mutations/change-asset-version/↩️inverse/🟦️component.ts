/** ↩️ change-asset-version direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeAssetVersion, type GltfChangeAssetVersionPayload } from '../../change-asset-version/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfChangeAssetVersionInverse = (base: GltfSnapshot, payload: GltfChangeAssetVersionPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfChangeAssetVersion(base, payload); return applied.accepted ? { accepted: true, inverse: { asset: { version: base.document.asset.version } }, touchedPaths: ["document/asset/version"] } : applied; };
