/** 🗑️ `delete-camera` wire twin: the flat `Apply` payload `GltfDeleteCameraPayload` and the phase wire `DeleteCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeleteCameraPayload {
  index: number;
}

export type DeleteCameraMutation = GltfPhase<GltfDeleteCameraPayload, GltfDiff>;

export const parseGltfDeleteCameraPayload = gltfWireObject<GltfDeleteCameraPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteCameraMutation = gltfWirePhase(parseGltfDeleteCameraPayload, parseGltfDiff);
