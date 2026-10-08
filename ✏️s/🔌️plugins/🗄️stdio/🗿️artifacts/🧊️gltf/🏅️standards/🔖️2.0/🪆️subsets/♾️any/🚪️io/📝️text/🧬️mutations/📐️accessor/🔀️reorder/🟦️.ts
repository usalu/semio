/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderAccessorsPayload,ReorderAccessorsMutation} from "../../../../../🧬️schema/🧬️mutations/📐️accessor/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📐️accessor/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-accessors` wire twin: the flat `Apply` payload `GltfReorderAccessorsPayload` and the phase wire `ReorderAccessorsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderAccessorsPayload = gltfWireObject<GltfReorderAccessorsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderAccessorsMutation = gltfWireApplyPhase(parseGltfReorderAccessorsPayload);
