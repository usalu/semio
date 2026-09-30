/** 🚚️ `move-camera` wire twin: the flat `Apply` payload `GltfMoveCameraPayload` and the phase wire `MoveCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfMoveCameraPayload {
  index: number;
  position: number;
}

export type MoveCameraMutation = GltfPhase<GltfMoveCameraPayload, GltfDiff>;

export const parseGltfMoveCameraPayload = gltfWireObject<GltfMoveCameraPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveCameraMutation = gltfWirePhase(parseGltfMoveCameraPayload, parseGltfDiff);
