/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderSceneRootNodesPayload,ReorderSceneRootNodesMutation} from "../../../../../🧬️schema/🧬️mutations/🌲️scene-root/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🌲️scene-root/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-scene-root-nodes` wire twin: the flat `Apply` payload `GltfReorderSceneRootNodesPayload` and the phase wire `ReorderSceneRootNodesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderSceneRootNodesPayload = gltfWireObject<GltfReorderSceneRootNodesPayload>({ scene: gltfWireRequired(gltfWireIndex), order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderSceneRootNodesMutation = gltfWirePhase(parseGltfReorderSceneRootNodesPayload, parseGltfDiff);
