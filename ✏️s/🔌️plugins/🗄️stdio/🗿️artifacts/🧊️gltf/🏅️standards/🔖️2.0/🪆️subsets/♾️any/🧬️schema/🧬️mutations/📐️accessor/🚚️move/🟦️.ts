/** 🚚️ `move-accessor` wire twin: the flat `Apply` payload `GltfMoveAccessorPayload` and the phase wire `MoveAccessorMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveAccessorPayload {
  index: number;
  position: number;
}

export type MoveAccessorMutation = GltfPhase<GltfMoveAccessorPayload, GltfDiff>;

export const parseGltfMoveAccessorPayload = gltfWireObject<GltfMoveAccessorPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveAccessorMutation = gltfWirePhase(parseGltfMoveAccessorPayload, parseGltfDiff);
