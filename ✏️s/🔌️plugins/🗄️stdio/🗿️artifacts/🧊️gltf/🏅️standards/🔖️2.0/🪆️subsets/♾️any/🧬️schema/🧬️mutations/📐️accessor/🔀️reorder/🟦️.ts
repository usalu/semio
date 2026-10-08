/** 🔀️ `reorder-accessors` wire twin: the flat `Apply` payload `GltfReorderAccessorsPayload` and the phase wire `ReorderAccessorsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderAccessorsPayload {
  order: bigint[];
}

export type ReorderAccessorsMutation = GltfApplyPhase<GltfReorderAccessorsPayload>;

export const parseGltfReorderAccessorsPayload = gltfWireObject<GltfReorderAccessorsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderAccessorsMutation = gltfWireApplyPhase(parseGltfReorderAccessorsPayload);
