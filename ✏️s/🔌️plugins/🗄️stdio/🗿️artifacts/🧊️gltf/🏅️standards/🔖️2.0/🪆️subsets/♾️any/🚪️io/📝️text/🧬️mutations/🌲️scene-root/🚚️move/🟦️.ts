/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveSceneRootNodePayload,MoveSceneRootNodeMutation} from "../../../../../🧬️schema/🧬️mutations/🌲️scene-root/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌲️scene-root/🚚️move/🟦️.ts";
/** 🚚️ `move-scene-root-node` wire twin: the flat `Apply` payload `GltfMoveSceneRootNodePayload` and the phase wire `MoveSceneRootNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveSceneRootNodePayload = gltfWireObject<GltfMoveSceneRootNodePayload>({ scene: gltfWireRequired(gltfWireIndex), node: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveSceneRootNodeMutation = gltfWireApplyPhase(parseGltfMoveSceneRootNodePayload);
