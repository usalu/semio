/** 🚚️ `move-skin` wire twin: the flat `Apply` payload `GltfMoveSkinPayload` and the phase wire `MoveSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveSkinPayload {
  index: bigint;
  position: bigint;
}

export type MoveSkinMutation = GltfApplyPhase<GltfMoveSkinPayload>;

export const parseGltfMoveSkinPayload = gltfWireObject<GltfMoveSkinPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveSkinMutation = gltfWireApplyPhase(parseGltfMoveSkinPayload);
