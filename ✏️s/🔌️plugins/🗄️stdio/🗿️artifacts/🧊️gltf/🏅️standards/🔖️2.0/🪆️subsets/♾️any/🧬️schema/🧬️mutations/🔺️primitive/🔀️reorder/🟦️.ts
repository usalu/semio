/** 🔀️ `reorder-primitives` wire twin: the flat `Apply` payload `GltfReorderPrimitivesPayload` and the phase wire `ReorderPrimitivesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderPrimitivesPayload {
  mesh: bigint;
  order: bigint[];
}

export type ReorderPrimitivesMutation = GltfApplyPhase<GltfReorderPrimitivesPayload>;

export const parseGltfReorderPrimitivesPayload = gltfWireObject<GltfReorderPrimitivesPayload>({ mesh: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderPrimitivesMutation = gltfWireApplyPhase(parseGltfReorderPrimitivesPayload);
