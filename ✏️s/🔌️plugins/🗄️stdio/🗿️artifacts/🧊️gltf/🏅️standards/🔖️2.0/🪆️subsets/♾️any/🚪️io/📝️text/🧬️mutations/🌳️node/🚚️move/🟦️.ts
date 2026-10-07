/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveNodePayload,MoveNodeMutation} from "../../../../../🧬️schema/🧬️mutations/🌳️node/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌳️node/🚚️move/🟦️.ts";
/** 🚚️ `move-node` wire twin: the flat `Apply` payload `GltfMoveNodePayload` and the phase wire `MoveNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveNodePayload = gltfWireObject<GltfMoveNodePayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveNodeMutation = gltfWirePhase(parseGltfMoveNodePayload, parseGltfDiff);
