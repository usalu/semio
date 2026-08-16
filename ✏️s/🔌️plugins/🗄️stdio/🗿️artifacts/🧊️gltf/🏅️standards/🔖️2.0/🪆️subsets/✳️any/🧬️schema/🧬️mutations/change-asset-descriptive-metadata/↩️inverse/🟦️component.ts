/** ↩️ change-asset-descriptive-metadata direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeAssetDescriptiveMetadata, type GltfChangeAssetDescriptiveMetadataPayload } from '../../change-asset-descriptive-metadata/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfChangeAssetDescriptiveMetadataInverse = (base: GltfSnapshot, payload: GltfChangeAssetDescriptiveMetadataPayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfChangeAssetDescriptiveMetadata(base, payload); return applied.accepted ? { accepted: true, inverse: { asset: { generator: base.document.asset.generator ?? null, copyright: base.document.asset.copyright ?? null, minVersion: base.document.asset.minVersion ?? null } }, touchedPaths: ["document/asset/generator","document/asset/copyright","document/asset/minVersion"] } : applied; };
