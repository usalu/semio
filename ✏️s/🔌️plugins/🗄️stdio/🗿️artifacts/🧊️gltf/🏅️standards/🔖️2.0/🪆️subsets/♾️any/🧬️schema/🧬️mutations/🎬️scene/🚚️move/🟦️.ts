/** 🚚️ `move-scene` wire twin: the flat `Apply` payload `GltfMoveScenePayload` and the phase wire `MoveSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveScenePayload {
  index: bigint;
  position: bigint;
}

export type MoveSceneMutation = GltfPhase<GltfMoveScenePayload, GltfDiff>;

export const parseGltfMoveScenePayload = gltfWireObject<GltfMoveScenePayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveSceneMutation = gltfWirePhase(parseGltfMoveScenePayload, parseGltfDiff);
