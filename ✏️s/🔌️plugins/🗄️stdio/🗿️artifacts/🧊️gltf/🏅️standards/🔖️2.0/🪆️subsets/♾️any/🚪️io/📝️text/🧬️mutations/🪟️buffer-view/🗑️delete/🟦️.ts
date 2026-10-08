/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteBufferViewPayload,DeleteBufferViewMutation} from "../../../../../🧬️schema/🧬️mutations/🪟️buffer-view/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🪟️buffer-view/🗑️delete/🟦️.ts";
/** 🗑️ `delete-buffer-view` wire twin: the flat `Apply` payload `GltfDeleteBufferViewPayload` and the phase wire `DeleteBufferViewMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteBufferViewPayload = gltfWireObject<GltfDeleteBufferViewPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteBufferViewMutation = gltfWireApplyPhase(parseGltfDeleteBufferViewPayload);
