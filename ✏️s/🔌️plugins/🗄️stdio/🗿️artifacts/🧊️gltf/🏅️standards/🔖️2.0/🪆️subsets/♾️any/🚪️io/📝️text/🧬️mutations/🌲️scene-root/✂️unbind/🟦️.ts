/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfUnbindSceneRootNodePayload,UnbindSceneRootNodeMutation} from "../../../../../🧬️schema/🧬️mutations/🌲️scene-root/✂️unbind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌲️scene-root/✂️unbind/🟦️.ts";
/** ✂️ `unbind-scene-root-node` wire twin: the flat `Apply` payload `GltfUnbindSceneRootNodePayload` and the phase wire `UnbindSceneRootNodeMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfUnbindSceneRootNodePayload = gltfWireObject<GltfUnbindSceneRootNodePayload>({ scene: gltfWireRequired(gltfWireIndex), node: gltfWireRequired(gltfWireIndex) });
export const parseUnbindSceneRootNodeMutation = gltfWirePhase(parseGltfUnbindSceneRootNodePayload, parseGltfDiff);
