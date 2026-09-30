/** 🔀️ `reorder-meshs` wire twin: the flat `Apply` payload `GltfReorderMeshsPayload` and the phase wire `ReorderMeshsMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderMeshsPayload {
  order: number[];
}

export type ReorderMeshsMutation = GltfPhase<GltfReorderMeshsPayload, GltfDiff>;

export const parseGltfReorderMeshsPayload = gltfWireObject<GltfReorderMeshsPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderMeshsMutation = gltfWirePhase(parseGltfReorderMeshsPayload, parseGltfDiff);
