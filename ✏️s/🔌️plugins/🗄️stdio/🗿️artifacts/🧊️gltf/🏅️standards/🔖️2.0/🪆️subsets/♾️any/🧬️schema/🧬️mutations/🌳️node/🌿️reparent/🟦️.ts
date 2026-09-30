/** 🌿️ `move-node-parent` wire twin: the flat `Apply` payload `GltfReparentNodePayload` and the phase wire `MoveNodeParentMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReparentNodePayload {
  parent: number;
  child: number;
  position: number;
}

export type MoveNodeParentMutation = GltfPhase<GltfReparentNodePayload, GltfDiff>;

export const parseGltfReparentNodePayload = gltfWireObject<GltfReparentNodePayload>({ parent: gltfWireRequired(gltfWireIndex), child: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveNodeParentMutation = gltfWirePhase(parseGltfReparentNodePayload, parseGltfDiff);
