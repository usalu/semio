/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreatePrimitivePayload,CreatePrimitiveMutation} from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/🌱️create/🟦️.ts";
/** 🌱️ `create-primitive` wire twin: the flat `Apply` payload `GltfCreatePrimitivePayload` and the phase wire `CreatePrimitiveMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreatePrimitivePayload = gltfWireObject<GltfCreatePrimitivePayload>({ mesh: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseCreatePrimitiveMutation = gltfWirePhase(parseGltfCreatePrimitivePayload, parseGltfDiff);
