/** 🧩️ `change-asset-extension-data` wire twin: the flat `Apply` payload `GltfChangeAssetExtensionDataPayload` and the phase wire `ChangeAssetExtensionDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireObject, gltfWireRequired, parseGltfJson } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeAssetExtensionDataPayload {
  data: GltfJson;
}

export type ChangeAssetExtensionDataMutation = GltfPhase<GltfChangeAssetExtensionDataPayload, GltfDiff>;

export const parseGltfChangeAssetExtensionDataPayload = gltfWireObject<GltfChangeAssetExtensionDataPayload>({ data: gltfWireRequired(parseGltfJson) });
export const parseChangeAssetExtensionDataMutation = gltfWirePhase(parseGltfChangeAssetExtensionDataPayload, parseGltfDiff);
