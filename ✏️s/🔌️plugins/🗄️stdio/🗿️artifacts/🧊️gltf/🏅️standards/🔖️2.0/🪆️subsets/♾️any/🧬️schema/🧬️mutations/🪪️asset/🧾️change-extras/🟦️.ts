/** 🧾️ `change-asset-extra-data` wire twin: the flat `Apply` payload `GltfChangeAssetExtraDataPayload` and the phase wire `ChangeAssetExtraDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireObject, gltfWireRequired, parseGltfJson } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeAssetExtraDataPayload {
  data: GltfJson;
}

export type ChangeAssetExtraDataMutation = GltfApplyPhase<GltfChangeAssetExtraDataPayload>;

export const parseGltfChangeAssetExtraDataPayload = gltfWireObject<GltfChangeAssetExtraDataPayload>({ data: gltfWireRequired(parseGltfJson) });
export const parseChangeAssetExtraDataMutation = gltfWireApplyPhase(parseGltfChangeAssetExtraDataPayload);
