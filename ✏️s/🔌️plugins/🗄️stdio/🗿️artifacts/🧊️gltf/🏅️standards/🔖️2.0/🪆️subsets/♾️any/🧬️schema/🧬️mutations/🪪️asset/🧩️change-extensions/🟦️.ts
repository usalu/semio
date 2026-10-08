/** 🧩️ `change-asset-extension-data` wire twin: the flat `Apply` payload `GltfChangeAssetExtensionDataPayload` and the phase wire `ChangeAssetExtensionDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireObject, gltfWireRequired, parseGltfJson } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeAssetExtensionDataPayload {
  data: GltfJson;
}

export type ChangeAssetExtensionDataMutation = GltfApplyPhase<GltfChangeAssetExtensionDataPayload>;

export const parseGltfChangeAssetExtensionDataPayload = gltfWireObject<GltfChangeAssetExtensionDataPayload>({ data: gltfWireRequired(parseGltfJson) });
export const parseChangeAssetExtensionDataMutation = gltfWireApplyPhase(parseGltfChangeAssetExtensionDataPayload);
