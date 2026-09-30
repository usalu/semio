/** 🗑️ `delete-scene` wire twin: the flat `Apply` payload `GltfDeleteScenePayload` and the phase wire `DeleteSceneMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteScenePayload {
  index: number;
}

export type DeleteSceneMutation = GltfPhase<GltfDeleteScenePayload, GltfDiff>;

export const parseGltfDeleteScenePayload = gltfWireObject<GltfDeleteScenePayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteSceneMutation = gltfWirePhase(parseGltfDeleteScenePayload, parseGltfDiff);
