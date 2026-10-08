/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfBindPrimitiveIndicesPayload,BindPrimitiveIndicesMutation} from "../../../../../🧬️schema/🧬️mutations/🔢️primitive/🔗️bind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔢️primitive/🔗️bind/🟦️.ts";
/** 🔗️ `bind-primitive-indices` wire twin: the flat `Apply` payload `GltfBindPrimitiveIndicesPayload` and the phase wire `BindPrimitiveIndicesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfBindPrimitiveIndicesPayload = gltfWireObject<GltfBindPrimitiveIndicesPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), accessor: gltfWireRequired(gltfWireIndex) });
export const parseBindPrimitiveIndicesMutation = gltfWireApplyPhase(parseGltfBindPrimitiveIndicesPayload);
