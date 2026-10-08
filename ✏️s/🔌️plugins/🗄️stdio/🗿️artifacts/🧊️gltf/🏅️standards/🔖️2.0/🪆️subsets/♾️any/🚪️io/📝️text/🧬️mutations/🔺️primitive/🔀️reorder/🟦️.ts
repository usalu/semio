/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderPrimitivesPayload,ReorderPrimitivesMutation} from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-primitives` wire twin: the flat `Apply` payload `GltfReorderPrimitivesPayload` and the phase wire `ReorderPrimitivesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderPrimitivesPayload = gltfWireObject<GltfReorderPrimitivesPayload>({ mesh: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderPrimitivesMutation = gltfWireApplyPhase(parseGltfReorderPrimitivesPayload);
