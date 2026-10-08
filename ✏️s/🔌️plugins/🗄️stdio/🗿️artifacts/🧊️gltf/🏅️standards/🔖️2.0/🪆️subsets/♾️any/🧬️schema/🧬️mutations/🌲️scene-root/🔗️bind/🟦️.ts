/** 🔗️ `bind-scene-root-node` wire twin: the flat `Apply` payload `GltfBindSceneRootNodePayload` and the phase wire `BindSceneRootNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindSceneRootNodePayload {
  scene: bigint;
  node: bigint;
  position: bigint;
}

export type BindSceneRootNodeMutation = GltfApplyPhase<GltfBindSceneRootNodePayload>;

export const parseGltfBindSceneRootNodePayload = gltfWireObject<GltfBindSceneRootNodePayload>({ scene: gltfWireRequired(gltfWireIndex), node: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseBindSceneRootNodeMutation = gltfWireApplyPhase(parseGltfBindSceneRootNodePayload);
