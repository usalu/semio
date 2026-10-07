/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderAnimationsPayload,ReorderAnimationsMutation} from "../../../../../🧬️schema/🧬️mutations/🎞️animation/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎞️animation/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-animations` wire twin: the flat `Apply` payload `GltfReorderAnimationsPayload` and the phase wire `ReorderAnimationsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderAnimationsPayload = gltfWireObject<GltfReorderAnimationsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderAnimationsMutation = gltfWirePhase(parseGltfReorderAnimationsPayload, parseGltfDiff);
