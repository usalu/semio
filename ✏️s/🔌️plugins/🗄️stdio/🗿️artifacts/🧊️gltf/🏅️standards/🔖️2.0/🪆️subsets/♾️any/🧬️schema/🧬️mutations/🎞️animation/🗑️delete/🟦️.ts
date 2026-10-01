/** 🗑️ `delete-animation` wire twin: the flat `Apply` payload `GltfDeleteAnimationPayload` and the phase wire `DeleteAnimationMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteAnimationPayload {
  index: bigint;
}

export type DeleteAnimationMutation = GltfPhase<GltfDeleteAnimationPayload, GltfDiff>;

export const parseGltfDeleteAnimationPayload = gltfWireObject<GltfDeleteAnimationPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteAnimationMutation = gltfWirePhase(parseGltfDeleteAnimationPayload, parseGltfDiff);
