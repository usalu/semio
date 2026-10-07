/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderMorphTargetAttributesPayload,ReorderMorphTargetAttributesMutation} from "../../../../../🧬️schema/🧬️mutations/🎚️morph/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎚️morph/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-morph-target-attributes` wire twin: the flat `Apply` payload `GltfReorderMorphTargetAttributesPayload` and the phase wire `ReorderMorphTargetAttributesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderMorphTargetAttributesPayload = gltfWireObject<GltfReorderMorphTargetAttributesPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), target: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireString)) });
export const parseReorderMorphTargetAttributesMutation = gltfWirePhase(parseGltfReorderMorphTargetAttributesPayload, parseGltfDiff);
