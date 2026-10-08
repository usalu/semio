/** 🗑️ `delete-accessor` wire twin: the flat `Apply` payload `GltfDeleteAccessorPayload` and the phase wire `DeleteAccessorMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteAccessorPayload {
  index: bigint;
}

export type DeleteAccessorMutation = GltfApplyPhase<GltfDeleteAccessorPayload>;

export const parseGltfDeleteAccessorPayload = gltfWireObject<GltfDeleteAccessorPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteAccessorMutation = gltfWireApplyPhase(parseGltfDeleteAccessorPayload);
