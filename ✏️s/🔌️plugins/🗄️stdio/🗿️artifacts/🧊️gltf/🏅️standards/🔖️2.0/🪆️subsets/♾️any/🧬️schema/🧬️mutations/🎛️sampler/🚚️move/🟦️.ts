/** 🚚️ `move-sampler` wire twin: the flat `Apply` payload `GltfMoveSamplerPayload` and the phase wire `MoveSamplerMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveSamplerPayload {
  index: bigint;
  position: bigint;
}

export type MoveSamplerMutation = GltfApplyPhase<GltfMoveSamplerPayload>;

export const parseGltfMoveSamplerPayload = gltfWireObject<GltfMoveSamplerPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveSamplerMutation = gltfWireApplyPhase(parseGltfMoveSamplerPayload);
