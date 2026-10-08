/** 🗑️ `delete-image` wire twin: the flat `Apply` payload `GltfDeleteImagePayload` and the phase wire `DeleteImageMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteImagePayload {
  index: bigint;
}

export type DeleteImageMutation = GltfApplyPhase<GltfDeleteImagePayload>;

export const parseGltfDeleteImagePayload = gltfWireObject<GltfDeleteImagePayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteImageMutation = gltfWireApplyPhase(parseGltfDeleteImagePayload);
