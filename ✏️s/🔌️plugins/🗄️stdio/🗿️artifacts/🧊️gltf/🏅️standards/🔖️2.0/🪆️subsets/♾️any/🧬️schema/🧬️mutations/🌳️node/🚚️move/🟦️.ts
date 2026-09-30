/** 🚚️ `move-node` wire twin: the flat `Apply` payload `GltfMoveNodePayload` and the phase wire `MoveNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveNodePayload {
  index: number;
  position: number;
}

export type MoveNodeMutation = GltfPhase<GltfMoveNodePayload, GltfDiff>;

export const parseGltfMoveNodePayload = gltfWireObject<GltfMoveNodePayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveNodeMutation = gltfWirePhase(parseGltfMoveNodePayload, parseGltfDiff);
