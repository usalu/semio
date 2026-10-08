/** ✂️ `unbind-scene-root-node` wire twin: the flat `Apply` payload `GltfUnbindSceneRootNodePayload` and the phase wire `UnbindSceneRootNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindSceneRootNodePayload {
  scene: bigint;
  node: bigint;
}

export type UnbindSceneRootNodeMutation = GltfApplyPhase<GltfUnbindSceneRootNodePayload>;

export const parseGltfUnbindSceneRootNodePayload = gltfWireObject<GltfUnbindSceneRootNodePayload>({ scene: gltfWireRequired(gltfWireIndex), node: gltfWireRequired(gltfWireIndex) });
export const parseUnbindSceneRootNodeMutation = gltfWireApplyPhase(parseGltfUnbindSceneRootNodePayload);
