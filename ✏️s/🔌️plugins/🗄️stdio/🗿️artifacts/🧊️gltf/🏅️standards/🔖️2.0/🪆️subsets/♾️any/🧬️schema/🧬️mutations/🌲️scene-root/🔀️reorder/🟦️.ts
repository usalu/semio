/** 🔀️ `reorder-scene-root-nodes` wire twin: the flat `Apply` payload `GltfReorderSceneRootNodesPayload` and the phase wire `ReorderSceneRootNodesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderSceneRootNodesPayload {
  scene: bigint;
  order: bigint[];
}

export type ReorderSceneRootNodesMutation = GltfApplyPhase<GltfReorderSceneRootNodesPayload>;

export const parseGltfReorderSceneRootNodesPayload = gltfWireObject<GltfReorderSceneRootNodesPayload>({ scene: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderSceneRootNodesMutation = gltfWireApplyPhase(parseGltfReorderSceneRootNodesPayload);
