/** 🔀️ `reorder-nodes` wire twin: the flat `Apply` payload `GltfReorderNodesPayload` and the phase wire `ReorderNodesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderNodesPayload {
  order: bigint[];
}

export type ReorderNodesMutation = GltfPhase<GltfReorderNodesPayload, GltfDiff>;

export const parseGltfReorderNodesPayload = gltfWireObject<GltfReorderNodesPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderNodesMutation = gltfWirePhase(parseGltfReorderNodesPayload, parseGltfDiff);
