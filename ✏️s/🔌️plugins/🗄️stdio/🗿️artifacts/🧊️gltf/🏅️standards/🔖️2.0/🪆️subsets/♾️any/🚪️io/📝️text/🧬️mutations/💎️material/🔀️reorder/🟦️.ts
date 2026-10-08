/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderMaterialsPayload,ReorderMaterialsMutation} from "../../../../../🧬️schema/🧬️mutations/💎️material/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/💎️material/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-materials` wire twin: the flat `Apply` payload `GltfReorderMaterialsPayload` and the phase wire `ReorderMaterialsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderMaterialsPayload = gltfWireObject<GltfReorderMaterialsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderMaterialsMutation = gltfWireApplyPhase(parseGltfReorderMaterialsPayload);
