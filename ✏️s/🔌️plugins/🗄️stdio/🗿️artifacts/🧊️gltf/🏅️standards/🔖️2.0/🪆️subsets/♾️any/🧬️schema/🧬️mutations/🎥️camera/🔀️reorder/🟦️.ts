/** 🔀️ `reorder-cameras` wire twin: the flat `Apply` payload `GltfReorderCamerasPayload` and the phase wire `ReorderCamerasMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfReorderCamerasPayload {
  order: bigint[];
}

export type ReorderCamerasMutation = GltfApplyPhase<GltfReorderCamerasPayload>;

export const parseGltfReorderCamerasPayload = gltfWireObject<GltfReorderCamerasPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderCamerasMutation = gltfWireApplyPhase(parseGltfReorderCamerasPayload);
