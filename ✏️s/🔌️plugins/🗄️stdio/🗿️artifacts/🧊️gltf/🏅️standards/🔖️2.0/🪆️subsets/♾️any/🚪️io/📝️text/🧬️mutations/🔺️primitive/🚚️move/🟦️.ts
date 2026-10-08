/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMovePrimitivePayload,MovePrimitiveMutation} from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔺️primitive/🚚️move/🟦️.ts";
/** 🚚️ `move-primitive` wire twin: the flat `Apply` payload `GltfMovePrimitivePayload` and the phase wire `MovePrimitiveMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMovePrimitivePayload = gltfWireObject<GltfMovePrimitivePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMovePrimitiveMutation = gltfWireApplyPhase(parseGltfMovePrimitivePayload);
