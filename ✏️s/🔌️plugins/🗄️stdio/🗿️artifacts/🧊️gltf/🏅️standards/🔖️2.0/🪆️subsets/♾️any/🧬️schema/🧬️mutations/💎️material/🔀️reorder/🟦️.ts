/** 🔀️ `reorder-materials` wire twin: the flat `Apply` payload `GltfReorderMaterialsPayload` and the phase wire `ReorderMaterialsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderMaterialsPayload {
  order: number[];
}

export type ReorderMaterialsMutation = GltfPhase<GltfReorderMaterialsPayload, GltfDiff>;

export const parseGltfReorderMaterialsPayload = gltfWireObject<GltfReorderMaterialsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderMaterialsMutation = gltfWirePhase(parseGltfReorderMaterialsPayload, parseGltfDiff);
