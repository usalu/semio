/** 🗑️ `delete-accessor` wire twin: the flat `Apply` payload `GltfDeleteAccessorPayload` and the phase wire `DeleteAccessorMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteAccessorPayload {
  index: number;
}

export type DeleteAccessorMutation = GltfPhase<GltfDeleteAccessorPayload, GltfDiff>;

export const parseGltfDeleteAccessorPayload = gltfWireObject<GltfDeleteAccessorPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteAccessorMutation = gltfWirePhase(parseGltfDeleteAccessorPayload, parseGltfDiff);
