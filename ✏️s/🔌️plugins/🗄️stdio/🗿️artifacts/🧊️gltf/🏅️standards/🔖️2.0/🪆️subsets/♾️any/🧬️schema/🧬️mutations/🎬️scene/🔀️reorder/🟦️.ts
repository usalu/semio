/** 🔀️ `reorder-scenes` wire twin: the flat `Apply` payload `GltfReorderScenesPayload` and the phase wire `ReorderScenesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderScenesPayload {
  order: bigint[];
}

export type ReorderScenesMutation = GltfApplyPhase<GltfReorderScenesPayload>;

export const parseGltfReorderScenesPayload = gltfWireObject<GltfReorderScenesPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderScenesMutation = gltfWireApplyPhase(parseGltfReorderScenesPayload);
