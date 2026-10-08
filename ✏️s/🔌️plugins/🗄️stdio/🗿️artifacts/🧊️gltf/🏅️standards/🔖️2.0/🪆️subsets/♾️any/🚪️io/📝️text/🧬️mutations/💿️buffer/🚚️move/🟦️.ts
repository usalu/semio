/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveBufferPayload,MoveBufferMutation} from "../../../../../🧬️schema/🧬️mutations/💿️buffer/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/💿️buffer/🚚️move/🟦️.ts";
/** 🚚️ `move-buffer` wire twin: the flat `Apply` payload `GltfMoveBufferPayload` and the phase wire `MoveBufferMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveBufferPayload = gltfWireObject<GltfMoveBufferPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveBufferMutation = gltfWireApplyPhase(parseGltfMoveBufferPayload);
