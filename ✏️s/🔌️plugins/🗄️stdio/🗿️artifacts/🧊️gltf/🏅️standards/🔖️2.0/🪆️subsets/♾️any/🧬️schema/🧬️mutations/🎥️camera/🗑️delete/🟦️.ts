/** 🗑️ `delete-camera` wire twin: the flat `Apply` payload `GltfDeleteCameraPayload` and the phase wire `DeleteCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteCameraPayload {
  index: bigint;
}

export type DeleteCameraMutation = GltfApplyPhase<GltfDeleteCameraPayload>;

export const parseGltfDeleteCameraPayload = gltfWireObject<GltfDeleteCameraPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteCameraMutation = gltfWireApplyPhase(parseGltfDeleteCameraPayload);
