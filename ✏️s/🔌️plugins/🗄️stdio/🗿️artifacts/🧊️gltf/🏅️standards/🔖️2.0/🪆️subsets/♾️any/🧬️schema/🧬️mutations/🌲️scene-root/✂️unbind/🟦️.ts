/** ✂️ `unbind-scene-root-node` wire twin: the flat `Apply` payload `GltfUnbindSceneRootNodePayload` and the phase wire `UnbindSceneRootNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindSceneRootNodePayload {
  scene: bigint;
  node: bigint;
}

export type UnbindSceneRootNodeMutation = GltfPhase<GltfUnbindSceneRootNodePayload, GltfDiff>;

export const parseGltfUnbindSceneRootNodePayload = gltfWireObject<GltfUnbindSceneRootNodePayload>({ scene: gltfWireRequired(gltfWireIndex), node: gltfWireRequired(gltfWireIndex) });
export const parseUnbindSceneRootNodeMutation = gltfWirePhase(parseGltfUnbindSceneRootNodePayload, parseGltfDiff);
