/** 🚚️ `move-sampler` wire twin: the flat `Apply` payload `GltfMoveSamplerPayload` and the phase wire `MoveSamplerMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveSamplerPayload {
  index: bigint;
  position: bigint;
}

export type MoveSamplerMutation = GltfPhase<GltfMoveSamplerPayload, GltfDiff>;

export const parseGltfMoveSamplerPayload = gltfWireObject<GltfMoveSamplerPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveSamplerMutation = gltfWirePhase(parseGltfMoveSamplerPayload, parseGltfDiff);
