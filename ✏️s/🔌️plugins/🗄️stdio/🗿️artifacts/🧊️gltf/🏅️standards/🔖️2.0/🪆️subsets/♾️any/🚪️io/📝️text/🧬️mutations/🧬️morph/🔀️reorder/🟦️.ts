/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderMorphTargetsPayload,ReorderMorphTargetsMutation} from "../../../../../🧬️schema/🧬️mutations/🧬️morph/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🧬️morph/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-morph-targets` wire twin: the flat `Apply` payload `GltfReorderMorphTargetsPayload` and the phase wire `ReorderMorphTargetsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderMorphTargetsPayload = gltfWireObject<GltfReorderMorphTargetsPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderMorphTargetsMutation = gltfWireApplyPhase(parseGltfReorderMorphTargetsPayload);
