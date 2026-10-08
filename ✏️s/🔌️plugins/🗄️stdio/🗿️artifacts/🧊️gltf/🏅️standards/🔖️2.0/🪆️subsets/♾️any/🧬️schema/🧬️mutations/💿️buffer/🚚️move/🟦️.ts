/** 🚚️ `move-buffer` wire twin: the flat `Apply` payload `GltfMoveBufferPayload` and the phase wire `MoveBufferMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveBufferPayload {
  index: bigint;
  position: bigint;
}

export type MoveBufferMutation = GltfApplyPhase<GltfMoveBufferPayload>;

export const parseGltfMoveBufferPayload = gltfWireObject<GltfMoveBufferPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveBufferMutation = gltfWireApplyPhase(parseGltfMoveBufferPayload);
