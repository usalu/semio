/** 🌿️ `move-node-parent` wire twin: the flat `Apply` payload `GltfReparentNodePayload` and the phase wire `MoveNodeParentMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfReparentNodePayload {
  parent: bigint;
  child: bigint;
  position: bigint;
}

export type MoveNodeParentMutation = GltfApplyPhase<GltfReparentNodePayload>;

export const parseGltfReparentNodePayload = gltfWireObject<GltfReparentNodePayload>({ parent: gltfWireRequired(gltfWireIndex), child: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveNodeParentMutation = gltfWireApplyPhase(parseGltfReparentNodePayload);
