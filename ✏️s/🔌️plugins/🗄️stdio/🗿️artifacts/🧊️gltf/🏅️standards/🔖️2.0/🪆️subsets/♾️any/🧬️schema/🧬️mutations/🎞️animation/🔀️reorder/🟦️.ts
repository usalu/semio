/** 🔀️ `reorder-animations` wire twin: the flat `Apply` payload `GltfReorderAnimationsPayload` and the phase wire `ReorderAnimationsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderAnimationsPayload {
  order: number[];
}

export type ReorderAnimationsMutation = GltfPhase<GltfReorderAnimationsPayload, GltfDiff>;

export const parseGltfReorderAnimationsPayload = gltfWireObject<GltfReorderAnimationsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderAnimationsMutation = gltfWirePhase(parseGltfReorderAnimationsPayload, parseGltfDiff);
