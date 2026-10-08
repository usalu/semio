/** 🚚️ `move-scene` wire twin: the flat `Apply` payload `GltfMoveScenePayload` and the phase wire `MoveSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveScenePayload {
  index: bigint;
  position: bigint;
}

export type MoveSceneMutation = GltfApplyPhase<GltfMoveScenePayload>;

export const parseGltfMoveScenePayload = gltfWireObject<GltfMoveScenePayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveSceneMutation = gltfWireApplyPhase(parseGltfMoveScenePayload);
