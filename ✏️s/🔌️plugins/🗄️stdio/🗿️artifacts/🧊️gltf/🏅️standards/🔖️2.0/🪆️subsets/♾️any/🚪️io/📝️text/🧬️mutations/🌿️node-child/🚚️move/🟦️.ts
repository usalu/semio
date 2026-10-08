/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveNodeChildPayload,MoveNodeChildMutation} from "../../../../../🧬️schema/🧬️mutations/🌿️node-child/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌿️node-child/🚚️move/🟦️.ts";
/** 🚚️ `move-node-child` wire twin: the flat `Apply` payload `GltfMoveNodeChildPayload` and the phase wire `MoveNodeChildMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveNodeChildPayload = gltfWireObject<GltfMoveNodeChildPayload>({ parent: gltfWireRequired(gltfWireIndex), child: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveNodeChildMutation = gltfWireApplyPhase(parseGltfMoveNodeChildPayload);
