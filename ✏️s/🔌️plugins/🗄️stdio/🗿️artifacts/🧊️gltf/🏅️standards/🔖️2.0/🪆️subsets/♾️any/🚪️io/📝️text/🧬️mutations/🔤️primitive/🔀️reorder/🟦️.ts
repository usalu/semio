/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderPrimitiveAttributesPayload,ReorderPrimitiveAttributesMutation} from "../../../../../🧬️schema/🧬️mutations/🔤️primitive/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔤️primitive/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-primitive-attributes` wire twin: the flat `Apply` payload `GltfReorderPrimitiveAttributesPayload` and the phase wire `ReorderPrimitiveAttributesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderPrimitiveAttributesPayload = gltfWireObject<GltfReorderPrimitiveAttributesPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireString)) });
export const parseReorderPrimitiveAttributesMutation = gltfWireApplyPhase(parseGltfReorderPrimitiveAttributesPayload);
