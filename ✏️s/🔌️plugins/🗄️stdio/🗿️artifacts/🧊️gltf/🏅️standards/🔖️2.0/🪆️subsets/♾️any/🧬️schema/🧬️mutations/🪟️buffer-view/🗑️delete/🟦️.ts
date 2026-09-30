/** 🗑️ `delete-buffer-view` wire twin: the flat `Apply` payload `GltfDeleteBufferViewPayload` and the phase wire `DeleteBufferViewMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteBufferViewPayload {
  index: number;
}

export type DeleteBufferViewMutation = GltfPhase<GltfDeleteBufferViewPayload, GltfDiff>;

export const parseGltfDeleteBufferViewPayload = gltfWireObject<GltfDeleteBufferViewPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteBufferViewMutation = gltfWirePhase(parseGltfDeleteBufferViewPayload, parseGltfDiff);
