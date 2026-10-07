/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveImagePayload,MoveImageMutation} from "../../../../../🧬️schema/🧬️mutations/🖼️image/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🖼️image/🚚️move/🟦️.ts";
/** 🚚️ `move-image` wire twin: the flat `Apply` payload `GltfMoveImagePayload` and the phase wire `MoveImageMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveImagePayload = gltfWireObject<GltfMoveImagePayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveImageMutation = gltfWirePhase(parseGltfMoveImagePayload, parseGltfDiff);
