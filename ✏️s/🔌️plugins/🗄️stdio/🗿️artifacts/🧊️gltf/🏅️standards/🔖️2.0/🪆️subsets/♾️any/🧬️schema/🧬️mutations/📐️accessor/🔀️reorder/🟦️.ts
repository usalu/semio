/** 🔀️ `reorder-accessors` wire twin: the flat `Apply` payload `GltfReorderAccessorsPayload` and the phase wire `ReorderAccessorsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderAccessorsPayload {
  order: number[];
}

export type ReorderAccessorsMutation = GltfPhase<GltfReorderAccessorsPayload, GltfDiff>;

export const parseGltfReorderAccessorsPayload = gltfWireObject<GltfReorderAccessorsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderAccessorsMutation = gltfWirePhase(parseGltfReorderAccessorsPayload, parseGltfDiff);
