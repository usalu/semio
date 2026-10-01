/** 🔀️ `reorder-used-extensions` wire twin: the flat `Apply` payload `GltfReorderUsedExtensionsPayload` and the phase wire `ReorderUsedExtensionsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderUsedExtensionsPayload {
  order: string[];
}

export type ReorderUsedExtensionsMutation = GltfPhase<GltfReorderUsedExtensionsPayload, GltfDiff>;

export const parseGltfReorderUsedExtensionsPayload = gltfWireObject<GltfReorderUsedExtensionsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireString)) });
export const parseReorderUsedExtensionsMutation = gltfWirePhase(parseGltfReorderUsedExtensionsPayload, parseGltfDiff);
