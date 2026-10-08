/** 🔀️ `reorder-buffers` wire twin: the flat `Apply` payload `GltfReorderBuffersPayload` and the phase wire `ReorderBuffersMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderBuffersPayload {
  order: bigint[];
}

export type ReorderBuffersMutation = GltfApplyPhase<GltfReorderBuffersPayload>;

export const parseGltfReorderBuffersPayload = gltfWireObject<GltfReorderBuffersPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderBuffersMutation = gltfWireApplyPhase(parseGltfReorderBuffersPayload);
