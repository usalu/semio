/** 🚚️ `move-node` wire twin: the flat `Apply` payload `GltfMoveNodePayload` and the phase wire `MoveNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveNodePayload {
  index: bigint;
  position: bigint;
}

export type MoveNodeMutation = GltfApplyPhase<GltfMoveNodePayload>;

export const parseGltfMoveNodePayload = gltfWireObject<GltfMoveNodePayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveNodeMutation = gltfWireApplyPhase(parseGltfMoveNodePayload);
