/** 🚚️ `move-accessor` wire twin: the flat `Apply` payload `GltfMoveAccessorPayload` and the phase wire `MoveAccessorMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveAccessorPayload {
  index: bigint;
  position: bigint;
}

export type MoveAccessorMutation = GltfApplyPhase<GltfMoveAccessorPayload>;

export const parseGltfMoveAccessorPayload = gltfWireObject<GltfMoveAccessorPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveAccessorMutation = gltfWireApplyPhase(parseGltfMoveAccessorPayload);
