/** 🗑️ `delete-buffer` wire twin: the flat `Apply` payload `GltfDeleteBufferPayload` and the phase wire `DeleteBufferMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteBufferPayload {
  index: bigint;
}

export type DeleteBufferMutation = GltfApplyPhase<GltfDeleteBufferPayload>;

export const parseGltfDeleteBufferPayload = gltfWireObject<GltfDeleteBufferPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteBufferMutation = gltfWireApplyPhase(parseGltfDeleteBufferPayload);
