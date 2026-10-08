/** 🗑️ `delete-node` wire twin: the flat `Apply` payload `GltfDeleteNodePayload` and the phase wire `DeleteNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteNodePayload {
  index: bigint;
}

export type DeleteNodeMutation = GltfApplyPhase<GltfDeleteNodePayload>;

export const parseGltfDeleteNodePayload = gltfWireObject<GltfDeleteNodePayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteNodeMutation = gltfWireApplyPhase(parseGltfDeleteNodePayload);
