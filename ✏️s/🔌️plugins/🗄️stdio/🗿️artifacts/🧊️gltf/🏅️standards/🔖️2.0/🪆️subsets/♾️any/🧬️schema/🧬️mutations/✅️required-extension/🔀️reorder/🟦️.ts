/** 🔀️ `reorder-required-extensions` wire twin: the flat `Apply` payload `GltfReorderRequiredExtensionsPayload` and the phase wire `ReorderRequiredExtensionsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderRequiredExtensionsPayload {
  order: string[];
}

export type ReorderRequiredExtensionsMutation = GltfPhase<GltfReorderRequiredExtensionsPayload, GltfDiff>;

export const parseGltfReorderRequiredExtensionsPayload = gltfWireObject<GltfReorderRequiredExtensionsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireString)) });
export const parseReorderRequiredExtensionsMutation = gltfWirePhase(parseGltfReorderRequiredExtensionsPayload, parseGltfDiff);
