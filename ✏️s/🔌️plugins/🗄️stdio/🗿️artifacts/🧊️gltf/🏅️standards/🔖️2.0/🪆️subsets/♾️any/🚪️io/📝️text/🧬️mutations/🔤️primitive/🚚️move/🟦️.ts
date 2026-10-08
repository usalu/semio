/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMovePrimitiveAttributePayload,MovePrimitiveAttributeMutation} from "../../../../../🧬️schema/🧬️mutations/🔤️primitive/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔤️primitive/🚚️move/🟦️.ts";
/** 🚚️ `move-primitive-attribute` wire twin: the flat `Apply` payload `GltfMovePrimitiveAttributePayload` and the phase wire `MovePrimitiveAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMovePrimitiveAttributePayload = gltfWireObject<GltfMovePrimitiveAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseMovePrimitiveAttributeMutation = gltfWireApplyPhase(parseGltfMovePrimitiveAttributePayload);
