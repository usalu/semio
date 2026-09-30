/** 🔀️ `reorder-cameras` wire twin: the flat `Apply` payload `GltfReorderCamerasPayload` and the phase wire `ReorderCamerasMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderCamerasPayload {
  order: number[];
}

export type ReorderCamerasMutation = GltfPhase<GltfReorderCamerasPayload, GltfDiff>;

export const parseGltfReorderCamerasPayload = gltfWireObject<GltfReorderCamerasPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderCamerasMutation = gltfWirePhase(parseGltfReorderCamerasPayload, parseGltfDiff);
