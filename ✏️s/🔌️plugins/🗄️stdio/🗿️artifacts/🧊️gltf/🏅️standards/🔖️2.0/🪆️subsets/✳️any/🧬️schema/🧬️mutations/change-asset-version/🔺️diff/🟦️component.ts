/** 🔺️ change-asset-version direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeAssetVersion, type GltfChangeAssetVersionPayload } from '../../change-asset-version/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfChangeAssetVersionDiff = (base: GltfSnapshot, payload: GltfChangeAssetVersionPayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfChangeAssetVersion(base, payload); return applied.accepted ? { accepted: true, diff: { asset: { version: payload.version } }, touchedPaths: GltfChangeAssetVersionDescriptor.touchedPaths } : applied; };
export const GltfChangeAssetVersionDescriptor = { id: 's.stdio.gltf.mutation.change-asset-version.v1', touchedPaths: ["document/asset/version"] } as const;
