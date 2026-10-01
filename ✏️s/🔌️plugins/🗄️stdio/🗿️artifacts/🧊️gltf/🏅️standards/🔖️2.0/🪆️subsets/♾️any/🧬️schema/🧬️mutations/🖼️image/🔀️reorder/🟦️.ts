/** 🔀️ `reorder-images` wire twin: the flat `Apply` payload `GltfReorderImagesPayload` and the phase wire `ReorderImagesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderImagesPayload {
  order: bigint[];
}

export type ReorderImagesMutation = GltfPhase<GltfReorderImagesPayload, GltfDiff>;

export const parseGltfReorderImagesPayload = gltfWireObject<GltfReorderImagesPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderImagesMutation = gltfWirePhase(parseGltfReorderImagesPayload, parseGltfDiff);
