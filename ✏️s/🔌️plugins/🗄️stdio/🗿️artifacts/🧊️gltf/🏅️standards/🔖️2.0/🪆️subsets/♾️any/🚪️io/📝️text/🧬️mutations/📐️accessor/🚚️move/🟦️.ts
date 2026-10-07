/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveAccessorPayload,MoveAccessorMutation} from "../../../../../🧬️schema/🧬️mutations/📐️accessor/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📐️accessor/🚚️move/🟦️.ts";
/** 🚚️ `move-accessor` wire twin: the flat `Apply` payload `GltfMoveAccessorPayload` and the phase wire `MoveAccessorMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveAccessorPayload = gltfWireObject<GltfMoveAccessorPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveAccessorMutation = gltfWirePhase(parseGltfMoveAccessorPayload, parseGltfDiff);
