/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeletePrimitivePayload,DeletePrimitiveMutation} from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/🗑️delete/🟦️.ts";
/** 🗑️ `delete-primitive` wire twin: the flat `Apply` payload `GltfDeletePrimitivePayload` and the phase wire `DeletePrimitiveMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeletePrimitivePayload = gltfWireObject<GltfDeletePrimitivePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex) });
export const parseDeletePrimitiveMutation = gltfWirePhase(parseGltfDeletePrimitivePayload, parseGltfDiff);
