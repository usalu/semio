/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderScenesPayload,ReorderScenesMutation} from "../../../../../🧬️schema/🧬️mutations/🎬️scene/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎬️scene/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-scenes` wire twin: the flat `Apply` payload `GltfReorderScenesPayload` and the phase wire `ReorderScenesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderScenesPayload = gltfWireObject<GltfReorderScenesPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderScenesMutation = gltfWireApplyPhase(parseGltfReorderScenesPayload);
