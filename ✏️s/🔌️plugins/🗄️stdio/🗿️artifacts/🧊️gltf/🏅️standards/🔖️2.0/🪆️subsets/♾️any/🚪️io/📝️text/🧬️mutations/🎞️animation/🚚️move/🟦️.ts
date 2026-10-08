/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveAnimationPayload,MoveAnimationMutation} from "../../../../../🧬️schema/🧬️mutations/🎞️animation/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎞️animation/🚚️move/🟦️.ts";
/** 🚚️ `move-animation` wire twin: the flat `Apply` payload `GltfMoveAnimationPayload` and the phase wire `MoveAnimationMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveAnimationPayload = gltfWireObject<GltfMoveAnimationPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveAnimationMutation = gltfWireApplyPhase(parseGltfMoveAnimationPayload);
