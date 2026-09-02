/** 🧬 Transparent TypeScript aggregate for the asset slice of the glTF 2.0 mutation vocabulary. */
import type { GltfChangeAssetDescriptiveMetadataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️📦️change-asset-descriptive-metadata/🟦️.ts';
import type { GltfChangeAssetExtensionDataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️📦️change-asset-extension-data/🟦️.ts';
import type { GltfChangeAssetExtraDataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️📦️change-asset-extra-data/🟦️.ts';
import type { GltfChangeAssetVersionPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️📦️change-asset-version/🟦️.ts';
import type { GltfChangeDocumentExtensionDataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️📄️change-document-extension-data/🟦️.ts';
import type { GltfChangeDocumentExtraDataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️📄️change-document-extra-data/🟦️.ts';
import type { GltfDeclareUsedExtensionPayload } from '../../../✳️any/🧬️schema/🧬️mutations/📣️🧩️add-used-extension/🟦️.ts';
import type { GltfMoveRequiredExtensionPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🧩️move-required-extension/🟦️.ts';
import type { GltfMoveUsedExtensionPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🧩️move-used-extension/🟦️.ts';
import type { GltfReorderRequiredExtensionsPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🧩️reorder-required-extensions/🟦️.ts';
import type { GltfReorderUsedExtensionsPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🧩️reorder-used-extensions/🟦️.ts';
import type { GltfRequireExtensionPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✅️🧩️add-required-extension/🟦️.ts';
import type { GltfUnrequireExtensionPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚫️🧩️remove-required-extension/🟦️.ts';
import type { GltfWithdrawUsedExtensionPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔙️🧩️remove-used-extension/🟦️.ts';

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
