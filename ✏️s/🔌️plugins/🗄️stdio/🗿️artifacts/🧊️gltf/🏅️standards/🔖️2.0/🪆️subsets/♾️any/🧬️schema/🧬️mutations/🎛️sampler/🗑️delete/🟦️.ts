/** 🗑️ `delete-sampler` wire twin: the flat `Apply` payload `GltfDeleteSamplerPayload` and the phase wire `DeleteSamplerMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteSamplerPayload {
  index: number;
}

export type DeleteSamplerMutation = GltfPhase<GltfDeleteSamplerPayload, GltfDiff>;

export const parseGltfDeleteSamplerPayload = gltfWireObject<GltfDeleteSamplerPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteSamplerMutation = gltfWirePhase(parseGltfDeleteSamplerPayload, parseGltfDiff);
