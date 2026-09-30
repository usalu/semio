/** 🗑️ `delete-buffer` wire twin: the flat `Apply` payload `GltfDeleteBufferPayload` and the phase wire `DeleteBufferMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteBufferPayload {
  index: number;
}

export type DeleteBufferMutation = GltfPhase<GltfDeleteBufferPayload, GltfDiff>;

export const parseGltfDeleteBufferPayload = gltfWireObject<GltfDeleteBufferPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteBufferMutation = gltfWirePhase(parseGltfDeleteBufferPayload, parseGltfDiff);
