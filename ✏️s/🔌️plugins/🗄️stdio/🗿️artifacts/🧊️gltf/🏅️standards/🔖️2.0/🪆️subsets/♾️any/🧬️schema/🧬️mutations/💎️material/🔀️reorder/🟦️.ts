/** 🔀️ `reorder-materials` wire twin: the flat `Apply` payload `GltfReorderMaterialsPayload` and the phase wire `ReorderMaterialsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderMaterialsPayload {
  order: bigint[];
}

export type ReorderMaterialsMutation = GltfApplyPhase<GltfReorderMaterialsPayload>;

export const parseGltfReorderMaterialsPayload = gltfWireObject<GltfReorderMaterialsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderMaterialsMutation = gltfWireApplyPhase(parseGltfReorderMaterialsPayload);
