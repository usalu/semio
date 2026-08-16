/** 🔺️ change-asset-descriptive-metadata direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeAssetDescriptiveMetadata, type GltfChangeAssetDescriptiveMetadataPayload } from '../../change-asset-descriptive-metadata/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfChangeAssetDescriptiveMetadataDiff = (base: GltfSnapshot, payload: GltfChangeAssetDescriptiveMetadataPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfChangeAssetDescriptiveMetadata(base, payload); return applied.accepted ? { accepted: true, diff: { asset: { generator: payload.generator, copyright: payload.copyright, minVersion: payload.minVersion } }, touchedPaths: GltfChangeAssetDescriptiveMetadataDescriptor.touchedPaths } : applied; };
export const GltfChangeAssetDescriptiveMetadataDescriptor = { id: 's.stdio.gltf.mutation.change-asset-descriptive-metadata.v1', touchedPaths: ["document/asset/generator","document/asset/copyright","document/asset/minVersion"] } as const;
