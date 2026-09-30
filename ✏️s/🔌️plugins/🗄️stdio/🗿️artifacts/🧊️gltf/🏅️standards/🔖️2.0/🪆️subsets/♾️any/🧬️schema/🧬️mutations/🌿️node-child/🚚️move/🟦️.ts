/** 🚚️ `move-node-child` wire twin: the flat `Apply` payload `GltfMoveNodeChildPayload` and the phase wire `MoveNodeChildMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveNodeChildPayload {
  parent: number;
  child: number;
  position: number;
}

export type MoveNodeChildMutation = GltfPhase<GltfMoveNodeChildPayload, GltfDiff>;

export const parseGltfMoveNodeChildPayload = gltfWireObject<GltfMoveNodeChildPayload>({ parent: gltfWireRequired(gltfWireIndex), child: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveNodeChildMutation = gltfWirePhase(parseGltfMoveNodeChildPayload, parseGltfDiff);
