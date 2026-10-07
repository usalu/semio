/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteAccessorPayload,DeleteAccessorMutation} from "../../../../../🧬️schema/🧬️mutations/📐️accessor/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📐️accessor/🗑️delete/🟦️.ts";
/** 🗑️ `delete-accessor` wire twin: the flat `Apply` payload `GltfDeleteAccessorPayload` and the phase wire `DeleteAccessorMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteAccessorPayload = gltfWireObject<GltfDeleteAccessorPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteAccessorMutation = gltfWirePhase(parseGltfDeleteAccessorPayload, parseGltfDiff);
