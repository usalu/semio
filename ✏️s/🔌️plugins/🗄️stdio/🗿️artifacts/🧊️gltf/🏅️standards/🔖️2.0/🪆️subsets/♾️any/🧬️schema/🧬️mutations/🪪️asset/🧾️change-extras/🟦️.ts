/** 🧾️ `change-asset-extra-data` wire twin: the flat `Apply` payload `GltfChangeAssetExtraDataPayload` and the phase wire `ChangeAssetExtraDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireObject, gltfWireRequired, parseGltfJson } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeAssetExtraDataPayload {
  data: GltfJson;
}

export type ChangeAssetExtraDataMutation = GltfPhase<GltfChangeAssetExtraDataPayload, GltfDiff>;

export const parseGltfChangeAssetExtraDataPayload = gltfWireObject<GltfChangeAssetExtraDataPayload>({ data: gltfWireRequired(parseGltfJson) });
export const parseChangeAssetExtraDataMutation = gltfWirePhase(parseGltfChangeAssetExtraDataPayload, parseGltfDiff);
