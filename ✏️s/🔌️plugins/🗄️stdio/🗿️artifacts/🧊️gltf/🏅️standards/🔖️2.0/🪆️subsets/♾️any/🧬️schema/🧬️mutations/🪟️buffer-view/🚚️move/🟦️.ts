/** 🚚️ `move-buffer-view` wire twin: the flat `Apply` payload `GltfMoveBufferViewPayload` and the phase wire `MoveBufferViewMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveBufferViewPayload {
  index: bigint;
  position: bigint;
}

export type MoveBufferViewMutation = GltfApplyPhase<GltfMoveBufferViewPayload>;

export const parseGltfMoveBufferViewPayload = gltfWireObject<GltfMoveBufferViewPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveBufferViewMutation = gltfWireApplyPhase(parseGltfMoveBufferViewPayload);
