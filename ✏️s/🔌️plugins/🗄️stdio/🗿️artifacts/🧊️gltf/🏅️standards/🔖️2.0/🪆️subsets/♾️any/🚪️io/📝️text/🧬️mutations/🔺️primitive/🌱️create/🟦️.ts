/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreatePrimitivePayload,CreatePrimitiveMutation} from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/🌱️create/🟦️.ts";
/** 🌱️ `create-primitive` wire twin: the flat `Apply` payload `GltfCreatePrimitivePayload` and the phase wire `CreatePrimitiveMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, parseGltfPrimitive } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreatePrimitivePayload = gltfWireObject<GltfCreatePrimitivePayload>({ mesh: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex), primitive: gltfWireOptional(parseGltfPrimitive) });
export const parseCreatePrimitiveMutation = gltfWireApplyPhase(parseGltfCreatePrimitivePayload);
