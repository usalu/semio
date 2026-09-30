/** 🔀️ `reorder-morph-targets` wire twin: the flat `Apply` payload `GltfReorderMorphTargetsPayload` and the phase wire `ReorderMorphTargetsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderMorphTargetsPayload {
  mesh: number;
  primitive: number;
  order: number[];
}

export type ReorderMorphTargetsMutation = GltfPhase<GltfReorderMorphTargetsPayload, GltfDiff>;

export const parseGltfReorderMorphTargetsPayload = gltfWireObject<GltfReorderMorphTargetsPayload>({ mesh: gltfWireRequired(gltfWireIndex), primitive: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderMorphTargetsMutation = gltfWirePhase(parseGltfReorderMorphTargetsPayload, parseGltfDiff);
