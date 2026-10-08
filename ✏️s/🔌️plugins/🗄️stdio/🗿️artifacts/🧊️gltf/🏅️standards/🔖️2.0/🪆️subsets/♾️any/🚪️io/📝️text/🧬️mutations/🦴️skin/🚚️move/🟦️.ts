/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveSkinPayload,MoveSkinMutation} from "../../../../../🧬️schema/🧬️mutations/🦴️skin/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🦴️skin/🚚️move/🟦️.ts";
/** 🚚️ `move-skin` wire twin: the flat `Apply` payload `GltfMoveSkinPayload` and the phase wire `MoveSkinMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveSkinPayload = gltfWireObject<GltfMoveSkinPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveSkinMutation = gltfWireApplyPhase(parseGltfMoveSkinPayload);
