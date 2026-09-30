/** 🔀️ `reorder-buffers` wire twin: the flat `Apply` payload `GltfReorderBuffersPayload` and the phase wire `ReorderBuffersMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderBuffersPayload {
  order: number[];
}

export type ReorderBuffersMutation = GltfPhase<GltfReorderBuffersPayload, GltfDiff>;

export const parseGltfReorderBuffersPayload = gltfWireObject<GltfReorderBuffersPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderBuffersMutation = gltfWirePhase(parseGltfReorderBuffersPayload, parseGltfDiff);
