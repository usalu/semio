/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateMaterialPayload,CreateMaterialMutation} from "../../../../../🧬️schema/🧬️mutations/💎️material/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/💎️material/🌱️create/🟦️.ts";
/** 🌱️ `create-material` wire twin: the flat `Apply` payload `GltfCreateMaterialPayload` and the phase wire `CreateMaterialMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, parseGltfMaterial } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateMaterialPayload = gltfWireObject<GltfCreateMaterialPayload>({ position: gltfWireRequired(gltfWireIndex), material: gltfWireOptional(parseGltfMaterial) });
export const parseCreateMaterialMutation = gltfWireApplyPhase(parseGltfCreateMaterialPayload);
