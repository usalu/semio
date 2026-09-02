/** 🧬 Transparent TypeScript aggregate for the asset slice of the glTF 2.0 mutation vocabulary. */
import type { GltfChangeAssetDescriptiveMetadataPayload } from './✏️📦️change-asset-descriptive-metadata/🟦️.ts';
import type { GltfChangeAssetExtensionDataPayload } from './✏️📦️change-asset-extension-data/🟦️.ts';
import type { GltfChangeAssetExtraDataPayload } from './✏️📦️change-asset-extra-data/🟦️.ts';
import type { GltfChangeAssetVersionPayload } from './✏️📦️change-asset-version/🟦️.ts';
import type { GltfChangeDocumentExtensionDataPayload } from './✏️📄️change-document-extension-data/🟦️.ts';
import type { GltfChangeDocumentExtraDataPayload } from './✏️📄️change-document-extra-data/🟦️.ts';
import type { GltfDeclareUsedExtensionPayload } from './📣️🧩️add-used-extension/🟦️.ts';
import type { GltfMoveRequiredExtensionPayload } from './🚚️🧩️move-required-extension/🟦️.ts';
import type { GltfMoveUsedExtensionPayload } from './🚚️🧩️move-used-extension/🟦️.ts';
import type { GltfReorderRequiredExtensionsPayload } from './🔀️🧩️reorder-required-extensions/🟦️.ts';
import type { GltfReorderUsedExtensionsPayload } from './🔀️🧩️reorder-used-extensions/🟦️.ts';
import type { GltfRequireExtensionPayload } from './✅️🧩️add-required-extension/🟦️.ts';
import type { GltfUnrequireExtensionPayload } from './🚫️🧩️remove-required-extension/🟦️.ts';
import type { GltfWithdrawUsedExtensionPayload } from './🔙️🧩️remove-used-extension/🟦️.ts';

export type GltfAssetMutation =
  | { readonly mutation: 'addRequiredExtension'; readonly payload: GltfRequireExtensionPayload }
  | { readonly mutation: 'changeDocumentExtensionData'; readonly payload: GltfChangeDocumentExtensionDataPayload }
  | { readonly mutation: 'changeDocumentExtraData'; readonly payload: GltfChangeDocumentExtraDataPayload }
  | { readonly mutation: 'changeAssetDescriptiveMetadata'; readonly payload: GltfChangeAssetDescriptiveMetadataPayload }
  | { readonly mutation: 'changeAssetExtensionData'; readonly payload: GltfChangeAssetExtensionDataPayload }
  | { readonly mutation: 'changeAssetExtraData'; readonly payload: GltfChangeAssetExtraDataPayload }
  | { readonly mutation: 'changeAssetVersion'; readonly payload: GltfChangeAssetVersionPayload }
  | { readonly mutation: 'addUsedExtension'; readonly payload: GltfDeclareUsedExtensionPayload }
  | { readonly mutation: 'reorderRequiredExtensions'; readonly payload: GltfReorderRequiredExtensionsPayload }
  | { readonly mutation: 'reorderUsedExtensions'; readonly payload: GltfReorderUsedExtensionsPayload }
  | { readonly mutation: 'removeUsedExtension'; readonly payload: GltfWithdrawUsedExtensionPayload }
  | { readonly mutation: 'moveRequiredExtension'; readonly payload: GltfMoveRequiredExtensionPayload }
  | { readonly mutation: 'moveUsedExtension'; readonly payload: GltfMoveUsedExtensionPayload }
  | { readonly mutation: 'removeRequiredExtension'; readonly payload: GltfUnrequireExtensionPayload };
