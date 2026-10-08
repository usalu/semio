/** 🔀️ `reorder-nodes` wire twin: the flat `Apply` payload `GltfReorderNodesPayload` and the phase wire `ReorderNodesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderNodesPayload {
  order: bigint[];
}

export type ReorderNodesMutation = GltfApplyPhase<GltfReorderNodesPayload>;

export const parseGltfReorderNodesPayload = gltfWireObject<GltfReorderNodesPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderNodesMutation = gltfWireApplyPhase(parseGltfReorderNodesPayload);
