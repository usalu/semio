/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfUnbindPrimitiveAttributePayload,UnbindPrimitiveAttributeMutation} from "../../../../../🧬️schema/🧬️mutations/🔤️primitive/✂️unbind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔤️primitive/✂️unbind/🟦️.ts";
/** ✂️ `unbind-primitive-attribute` wire twin: the flat `Apply` payload `GltfUnbindPrimitiveAttributePayload` and the phase wire `UnbindPrimitiveAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfUnbindPrimitiveAttributePayload = gltfWireObject<GltfUnbindPrimitiveAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString) });
export const parseUnbindPrimitiveAttributeMutation = gltfWireApplyPhase(parseGltfUnbindPrimitiveAttributePayload);
