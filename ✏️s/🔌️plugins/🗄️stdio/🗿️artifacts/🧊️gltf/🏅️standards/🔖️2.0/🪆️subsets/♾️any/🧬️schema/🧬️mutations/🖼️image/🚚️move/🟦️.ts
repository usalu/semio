/** 🚚️ `move-image` wire twin: the flat `Apply` payload `GltfMoveImagePayload` and the phase wire `MoveImageMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveImagePayload {
  index: bigint;
  position: bigint;
}

export type MoveImageMutation = GltfPhase<GltfMoveImagePayload, GltfDiff>;

export const parseGltfMoveImagePayload = gltfWireObject<GltfMoveImagePayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveImageMutation = gltfWirePhase(parseGltfMoveImagePayload, parseGltfDiff);
