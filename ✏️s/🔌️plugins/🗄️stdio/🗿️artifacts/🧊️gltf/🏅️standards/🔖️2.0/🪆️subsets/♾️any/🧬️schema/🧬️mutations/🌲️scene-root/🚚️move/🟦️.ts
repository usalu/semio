/** 🚚️ `move-scene-root-node` wire twin: the flat `Apply` payload `GltfMoveSceneRootNodePayload` and the phase wire `MoveSceneRootNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveSceneRootNodePayload {
  scene: number;
  node: number;
  position: number;
}

export type MoveSceneRootNodeMutation = GltfPhase<GltfMoveSceneRootNodePayload, GltfDiff>;

export const parseGltfMoveSceneRootNodePayload = gltfWireObject<GltfMoveSceneRootNodePayload>({ scene: gltfWireRequired(gltfWireIndex), node: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveSceneRootNodeMutation = gltfWirePhase(parseGltfMoveSceneRootNodePayload, parseGltfDiff);
