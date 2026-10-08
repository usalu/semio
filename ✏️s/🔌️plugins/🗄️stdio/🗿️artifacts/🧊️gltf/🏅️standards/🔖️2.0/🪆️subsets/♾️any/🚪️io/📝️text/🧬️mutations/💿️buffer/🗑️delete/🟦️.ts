/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteBufferPayload,DeleteBufferMutation} from "../../../../../🧬️schema/🧬️mutations/💿️buffer/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/💿️buffer/🗑️delete/🟦️.ts";
/** 🗑️ `delete-buffer` wire twin: the flat `Apply` payload `GltfDeleteBufferPayload` and the phase wire `DeleteBufferMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteBufferPayload = gltfWireObject<GltfDeleteBufferPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteBufferMutation = gltfWireApplyPhase(parseGltfDeleteBufferPayload);
