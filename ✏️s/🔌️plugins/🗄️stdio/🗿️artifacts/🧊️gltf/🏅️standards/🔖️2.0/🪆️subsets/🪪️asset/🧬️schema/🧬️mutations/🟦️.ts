/** 🪪️ `GltfAssetMutation` twin: the asset slice of the glTF 2.0 mutation vocabulary, a view over the any subset's `GltfMutation`
 * that selects branches and never restates a payload.
 * @see ./🔣️.json */
import { gltfWireRefuse, type GltfWireReader } from "../../../♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import { parseGltfMutation, type GltfMutation } from "../../../♾️any/🧬️schema/🧬️mutations/🟦️.ts";

export type GltfAssetMutation = Extract<GltfMutation, { readonly mutation: "changeAssetDescriptiveMetadata" | "changeAssetExtensionData" | "changeAssetExtraData" | "changeAssetVersion" | "changeDocumentExtensionData" | "changeDocumentExtraData" | "addUsedExtension" | "moveRequiredExtension" | "moveUsedExtension" | "reorderRequiredExtensions" | "reorderUsedExtensions" | "addRequiredExtension" | "removeRequiredExtension" | "removeUsedExtension" }>;

const members: readonly GltfMutation["mutation"][] = ["changeAssetDescriptiveMetadata", "changeAssetExtensionData", "changeAssetExtraData", "changeAssetVersion", "changeDocumentExtensionData", "changeDocumentExtraData", "addUsedExtension", "moveRequiredExtension", "moveUsedExtension", "reorderRequiredExtensions", "reorderUsedExtensions", "addRequiredExtension", "removeRequiredExtension", "removeUsedExtension"];

export const parseGltfAssetMutation: GltfWireReader<GltfAssetMutation> = (value, at = "$") => {
  const mutation = parseGltfMutation(value, at);
  return members.includes(mutation.mutation) ? (mutation as GltfAssetMutation) : gltfWireRefuse(`${at}.mutation`, `value is not one of ${members.join(", ")}`);
};
