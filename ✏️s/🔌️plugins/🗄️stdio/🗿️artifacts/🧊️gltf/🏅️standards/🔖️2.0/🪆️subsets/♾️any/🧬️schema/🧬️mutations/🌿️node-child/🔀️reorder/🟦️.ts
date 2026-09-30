/** 🔀️ `reorder-node-children` wire twin: the flat `Apply` payload `GltfReorderNodeChildrenPayload` and the phase wire `ReorderNodeChildrenMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderNodeChildrenPayload {
  parent: number;
  order: number[];
}

export type ReorderNodeChildrenMutation = GltfPhase<GltfReorderNodeChildrenPayload, GltfDiff>;

export const parseGltfReorderNodeChildrenPayload = gltfWireObject<GltfReorderNodeChildrenPayload>({ parent: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderNodeChildrenMutation = gltfWirePhase(parseGltfReorderNodeChildrenPayload, parseGltfDiff);
