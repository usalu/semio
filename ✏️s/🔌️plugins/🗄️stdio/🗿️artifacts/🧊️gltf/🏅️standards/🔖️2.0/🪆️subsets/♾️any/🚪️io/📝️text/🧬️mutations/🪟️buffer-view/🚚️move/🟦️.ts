/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveBufferViewPayload,MoveBufferViewMutation} from "../../../../../🧬️schema/🧬️mutations/🪟️buffer-view/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🪟️buffer-view/🚚️move/🟦️.ts";
/** 🚚️ `move-buffer-view` wire twin: the flat `Apply` payload `GltfMoveBufferViewPayload` and the phase wire `MoveBufferViewMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveBufferViewPayload = gltfWireObject<GltfMoveBufferViewPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveBufferViewMutation = gltfWirePhase(parseGltfMoveBufferViewPayload, parseGltfDiff);
