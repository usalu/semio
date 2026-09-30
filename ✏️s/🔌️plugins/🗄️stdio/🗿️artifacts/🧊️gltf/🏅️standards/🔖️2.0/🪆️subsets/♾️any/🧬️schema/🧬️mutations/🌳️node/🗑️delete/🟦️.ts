/** 🗑️ `delete-node` wire twin: the flat `Apply` payload `GltfDeleteNodePayload` and the phase wire `DeleteNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteNodePayload {
  index: number;
}

export type DeleteNodeMutation = GltfPhase<GltfDeleteNodePayload, GltfDiff>;

export const parseGltfDeleteNodePayload = gltfWireObject<GltfDeleteNodePayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteNodeMutation = gltfWirePhase(parseGltfDeleteNodePayload, parseGltfDiff);
