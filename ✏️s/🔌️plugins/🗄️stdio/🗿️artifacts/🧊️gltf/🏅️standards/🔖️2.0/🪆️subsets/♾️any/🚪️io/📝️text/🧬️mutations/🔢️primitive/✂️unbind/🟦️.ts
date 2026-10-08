/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfUnbindPrimitiveIndicesPayload,UnbindPrimitiveIndicesMutation} from "../../../../../🧬️schema/🧬️mutations/🔢️primitive/✂️unbind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🔢️primitive/✂️unbind/🟦️.ts";
/** ✂️ `unbind-primitive-indices` wire twin: the flat `Apply` payload `GltfUnbindPrimitiveIndicesPayload` and the phase wire `UnbindPrimitiveIndicesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfUnbindPrimitiveIndicesPayload = gltfWireObject<GltfUnbindPrimitiveIndicesPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex) });
export const parseUnbindPrimitiveIndicesMutation = gltfWireApplyPhase(parseGltfUnbindPrimitiveIndicesPayload);
