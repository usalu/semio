/** 🔖️ `change-asset-version` wire twin: the flat `Apply` payload `GltfChangeAssetVersionPayload` and the phase wire `ChangeAssetVersionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeAssetVersionPayload {
  version: string;
}

export type ChangeAssetVersionMutation = GltfApplyPhase<GltfChangeAssetVersionPayload>;

export const parseGltfChangeAssetVersionPayload = gltfWireObject<GltfChangeAssetVersionPayload>({ version: gltfWireRequired(gltfWireString) });
export const parseChangeAssetVersionMutation = gltfWireApplyPhase(parseGltfChangeAssetVersionPayload);
