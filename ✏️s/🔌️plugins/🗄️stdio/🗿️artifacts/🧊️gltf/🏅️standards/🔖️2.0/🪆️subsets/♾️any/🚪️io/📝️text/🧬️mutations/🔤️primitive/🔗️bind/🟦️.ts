/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfBindPrimitiveAttributePayload,BindPrimitiveAttributeMutation} from "../../../../../🧬️schema/🧬️mutations/🔤️primitive/🔗️bind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔤️primitive/🔗️bind/🟦️.ts";
/** 🔗️ `bind-primitive-attribute` wire twin: the flat `Apply` payload `GltfBindPrimitiveAttributePayload` and the phase wire `BindPrimitiveAttributeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfBindPrimitiveAttributePayload = gltfWireObject<GltfBindPrimitiveAttributePayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), semantic: gltfWireRequired(gltfWireString), accessor: gltfWireRequired(gltfWireIndex) });
export const parseBindPrimitiveAttributeMutation = gltfWireApplyPhase(parseGltfBindPrimitiveAttributePayload);
