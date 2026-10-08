/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfUnbindPrimitiveMaterialPayload,UnbindPrimitiveMaterialMutation} from "../../../../../🧬️schema/🧬️mutations/🧱️primitive/✂️unbind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🧱️primitive/✂️unbind/🟦️.ts";
/** ✂️ `unbind-primitive-material` wire twin: the flat `Apply` payload `GltfUnbindPrimitiveMaterialPayload` and the phase wire `UnbindPrimitiveMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfUnbindPrimitiveMaterialPayload = gltfWireObject<GltfUnbindPrimitiveMaterialPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex) });
export const parseUnbindPrimitiveMaterialMutation = gltfWireApplyPhase(parseGltfUnbindPrimitiveMaterialPayload);
