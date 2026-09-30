/** 🔀️ `reorder-primitives` wire twin: the flat `Apply` payload `GltfReorderPrimitivesPayload` and the phase wire `ReorderPrimitivesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderPrimitivesPayload {
  mesh: number;
  order: number[];
}

export type ReorderPrimitivesMutation = GltfPhase<GltfReorderPrimitivesPayload, GltfDiff>;

export const parseGltfReorderPrimitivesPayload = gltfWireObject<GltfReorderPrimitivesPayload>({ mesh: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderPrimitivesMutation = gltfWirePhase(parseGltfReorderPrimitivesPayload, parseGltfDiff);
