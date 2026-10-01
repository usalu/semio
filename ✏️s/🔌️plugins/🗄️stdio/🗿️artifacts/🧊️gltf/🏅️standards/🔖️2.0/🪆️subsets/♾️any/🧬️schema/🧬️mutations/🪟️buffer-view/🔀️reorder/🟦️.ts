/** 🔀️ `reorder-buffer-views` wire twin: the flat `Apply` payload `GltfReorderBufferViewsPayload` and the phase wire `ReorderBufferViewsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderBufferViewsPayload {
  order: bigint[];
}

export type ReorderBufferViewsMutation = GltfPhase<GltfReorderBufferViewsPayload, GltfDiff>;

export const parseGltfReorderBufferViewsPayload = gltfWireObject<GltfReorderBufferViewsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderBufferViewsMutation = gltfWirePhase(parseGltfReorderBufferViewsPayload, parseGltfDiff);
