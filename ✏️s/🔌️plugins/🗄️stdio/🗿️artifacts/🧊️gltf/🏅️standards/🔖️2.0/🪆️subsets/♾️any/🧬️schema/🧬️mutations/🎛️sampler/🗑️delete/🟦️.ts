/** 🗑️ `delete-sampler` wire twin: the flat `Apply` payload `GltfDeleteSamplerPayload` and the phase wire `DeleteSamplerMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteSamplerPayload {
  index: bigint;
}

export type DeleteSamplerMutation = GltfApplyPhase<GltfDeleteSamplerPayload>;

export const parseGltfDeleteSamplerPayload = gltfWireObject<GltfDeleteSamplerPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteSamplerMutation = gltfWireApplyPhase(parseGltfDeleteSamplerPayload);
