/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfBindSceneRootNodePayload,BindSceneRootNodeMutation} from "../../../../../🧬️schema/🧬️mutations/🌲️scene-root/🔗️bind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌲️scene-root/🔗️bind/🟦️.ts";
/** 🔗️ `bind-scene-root-node` wire twin: the flat `Apply` payload `GltfBindSceneRootNodePayload` and the phase wire `BindSceneRootNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfBindSceneRootNodePayload = gltfWireObject<GltfBindSceneRootNodePayload>({ scene: gltfWireRequired(gltfWireIndex), node: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseBindSceneRootNodeMutation = gltfWireApplyPhase(parseGltfBindSceneRootNodePayload);
