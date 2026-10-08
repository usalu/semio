/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveUsedExtensionPayload,MoveUsedExtensionMutation} from "../../../../../🧬️schema/🧬️mutations/📣️used/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📣️used/🚚️move/🟦️.ts";
/** 🚚️ `move-used-extension` wire twin: the flat `Apply` payload `GltfMoveUsedExtensionPayload` and the phase wire `MoveUsedExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveUsedExtensionPayload = gltfWireObject<GltfMoveUsedExtensionPayload>({ extension: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveUsedExtensionMutation = gltfWireApplyPhase(parseGltfMoveUsedExtensionPayload);
