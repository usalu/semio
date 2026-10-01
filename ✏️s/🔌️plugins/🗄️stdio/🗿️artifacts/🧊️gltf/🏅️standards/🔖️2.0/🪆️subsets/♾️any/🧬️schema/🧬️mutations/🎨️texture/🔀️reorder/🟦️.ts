/** 🔀️ `reorder-textures` wire twin: the flat `Apply` payload `GltfReorderTexturesPayload` and the phase wire `ReorderTexturesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderTexturesPayload {
  order: bigint[];
}

export type ReorderTexturesMutation = GltfPhase<GltfReorderTexturesPayload, GltfDiff>;

export const parseGltfReorderTexturesPayload = gltfWireObject<GltfReorderTexturesPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderTexturesMutation = gltfWirePhase(parseGltfReorderTexturesPayload, parseGltfDiff);
