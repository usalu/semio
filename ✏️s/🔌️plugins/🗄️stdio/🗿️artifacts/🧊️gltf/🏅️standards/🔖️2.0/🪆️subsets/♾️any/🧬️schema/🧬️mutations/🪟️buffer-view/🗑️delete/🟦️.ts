/** 🗑️ `delete-buffer-view` wire twin: the flat `Apply` payload `GltfDeleteBufferViewPayload` and the phase wire `DeleteBufferViewMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteBufferViewPayload {
  index: bigint;
}

export type DeleteBufferViewMutation = GltfApplyPhase<GltfDeleteBufferViewPayload>;

export const parseGltfDeleteBufferViewPayload = gltfWireObject<GltfDeleteBufferViewPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteBufferViewMutation = gltfWireApplyPhase(parseGltfDeleteBufferViewPayload);
