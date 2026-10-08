/** 🚚️ `move-camera` wire twin: the flat `Apply` payload `GltfMoveCameraPayload` and the phase wire `MoveCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveCameraPayload {
  index: bigint;
  position: bigint;
}

export type MoveCameraMutation = GltfApplyPhase<GltfMoveCameraPayload>;

export const parseGltfMoveCameraPayload = gltfWireObject<GltfMoveCameraPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveCameraMutation = gltfWireApplyPhase(parseGltfMoveCameraPayload);
