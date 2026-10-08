/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfBindPrimitiveMaterialPayload,BindPrimitiveMaterialMutation} from "../../../../../🧬️schema/🧬️mutations/🧱️primitive/🔗️bind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🧱️primitive/🔗️bind/🟦️.ts";
/** 🔗️ `bind-primitive-material` wire twin: the flat `Apply` payload `GltfBindPrimitiveMaterialPayload` and the phase wire `BindPrimitiveMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfBindPrimitiveMaterialPayload = gltfWireObject<GltfBindPrimitiveMaterialPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), material: gltfWireRequired(gltfWireIndex) });
export const parseBindPrimitiveMaterialMutation = gltfWireApplyPhase(parseGltfBindPrimitiveMaterialPayload);
