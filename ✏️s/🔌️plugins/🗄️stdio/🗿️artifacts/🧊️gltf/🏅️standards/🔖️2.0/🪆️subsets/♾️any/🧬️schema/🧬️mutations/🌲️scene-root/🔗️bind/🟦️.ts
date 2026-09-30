/** 🔗️ `bind-scene-root-node` wire twin: the flat `Apply` payload `GltfBindSceneRootNodePayload` and the phase wire `BindSceneRootNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindSceneRootNodePayload {
  scene: number;
  node: number;
  position: number;
}

export type BindSceneRootNodeMutation = GltfPhase<GltfBindSceneRootNodePayload, GltfDiff>;

export const parseGltfBindSceneRootNodePayload = gltfWireObject<GltfBindSceneRootNodePayload>({ scene: gltfWireRequired(gltfWireIndex), node: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseBindSceneRootNodeMutation = gltfWirePhase(parseGltfBindSceneRootNodePayload, parseGltfDiff);
