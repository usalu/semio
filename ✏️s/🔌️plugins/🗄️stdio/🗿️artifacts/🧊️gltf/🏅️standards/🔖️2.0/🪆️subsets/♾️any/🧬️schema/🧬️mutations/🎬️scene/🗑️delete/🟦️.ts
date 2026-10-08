/** 🗑️ `delete-scene` wire twin: the flat `Apply` payload `GltfDeleteScenePayload` and the phase wire `DeleteSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteScenePayload {
  index: bigint;
}

export type DeleteSceneMutation = GltfApplyPhase<GltfDeleteScenePayload>;

export const parseGltfDeleteScenePayload = gltfWireObject<GltfDeleteScenePayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteSceneMutation = gltfWireApplyPhase(parseGltfDeleteScenePayload);
