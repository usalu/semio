/** 🔀️ `reorder-scenes` wire twin: the flat `Apply` payload `GltfReorderScenesPayload` and the phase wire `ReorderScenesMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderScenesPayload {
  order: number[];
}

export type ReorderScenesMutation = GltfPhase<GltfReorderScenesPayload, GltfDiff>;

export const parseGltfReorderScenesPayload = gltfWireObject<GltfReorderScenesPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderScenesMutation = gltfWirePhase(parseGltfReorderScenesPayload, parseGltfDiff);
