/** 🔀️ `reorder-samplers` wire twin: the flat `Apply` payload `GltfReorderSamplersPayload` and the phase wire `ReorderSamplersMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderSamplersPayload {
  order: bigint[];
}

export type ReorderSamplersMutation = GltfPhase<GltfReorderSamplersPayload, GltfDiff>;

export const parseGltfReorderSamplersPayload = gltfWireObject<GltfReorderSamplersPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderSamplersMutation = gltfWirePhase(parseGltfReorderSamplersPayload, parseGltfDiff);
