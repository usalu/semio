/** 🔀️ `reorder-node-children` wire twin: the flat `Apply` payload `GltfReorderNodeChildrenPayload` and the phase wire `ReorderNodeChildrenMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderNodeChildrenPayload {
  parent: bigint;
  order: bigint[];
}

export type ReorderNodeChildrenMutation = GltfApplyPhase<GltfReorderNodeChildrenPayload>;

export const parseGltfReorderNodeChildrenPayload = gltfWireObject<GltfReorderNodeChildrenPayload>({ parent: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderNodeChildrenMutation = gltfWireApplyPhase(parseGltfReorderNodeChildrenPayload);
