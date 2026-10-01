/** 🚚️ `move-animation` wire twin: the flat `Apply` payload `GltfMoveAnimationPayload` and the phase wire `MoveAnimationMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveAnimationPayload {
  index: bigint;
  position: bigint;
}

export type MoveAnimationMutation = GltfPhase<GltfMoveAnimationPayload, GltfDiff>;

export const parseGltfMoveAnimationPayload = gltfWireObject<GltfMoveAnimationPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveAnimationMutation = gltfWirePhase(parseGltfMoveAnimationPayload, parseGltfDiff);
