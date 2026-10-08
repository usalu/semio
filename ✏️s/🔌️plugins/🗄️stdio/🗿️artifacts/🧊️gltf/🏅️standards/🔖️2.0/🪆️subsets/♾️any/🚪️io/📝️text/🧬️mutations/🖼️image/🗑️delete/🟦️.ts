/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteImagePayload,DeleteImageMutation} from "../../../../../🧬️schema/🧬️mutations/🖼️image/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🖼️image/🗑️delete/🟦️.ts";
/** 🗑️ `delete-image` wire twin: the flat `Apply` payload `GltfDeleteImagePayload` and the phase wire `DeleteImageMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteImagePayload = gltfWireObject<GltfDeleteImagePayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteImageMutation = gltfWireApplyPhase(parseGltfDeleteImagePayload);
