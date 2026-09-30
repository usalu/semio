/** 🗑️ `delete-image` wire twin: the flat `Apply` payload `GltfDeleteImagePayload` and the phase wire `DeleteImageMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteImagePayload {
  index: number;
}

export type DeleteImageMutation = GltfPhase<GltfDeleteImagePayload, GltfDiff>;

export const parseGltfDeleteImagePayload = gltfWireObject<GltfDeleteImagePayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteImageMutation = gltfWirePhase(parseGltfDeleteImagePayload, parseGltfDiff);
