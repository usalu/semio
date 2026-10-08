/** 🗑️ `delete-animation` wire twin: the flat `Apply` payload `GltfDeleteAnimationPayload` and the phase wire `DeleteAnimationMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteAnimationPayload {
  index: bigint;
}

export type DeleteAnimationMutation = GltfApplyPhase<GltfDeleteAnimationPayload>;

export const parseGltfDeleteAnimationPayload = gltfWireObject<GltfDeleteAnimationPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteAnimationMutation = gltfWireApplyPhase(parseGltfDeleteAnimationPayload);
