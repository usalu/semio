/** 🌱️ `create-camera` wire twin: the flat `Apply` payload `GltfCreateCameraPayload` and the phase wire `CreateCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfCameraProjection, gltfWireIndex, gltfWireObject, gltfWireRequired, parseGltfCameraProjection } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateCameraPayload {
  position: number;
  projection: GltfCameraProjection;
}

export type CreateCameraMutation = GltfPhase<GltfCreateCameraPayload, GltfDiff>;

export const parseGltfCreateCameraPayload = gltfWireObject<GltfCreateCameraPayload>({ position: gltfWireRequired(gltfWireIndex), projection: gltfWireRequired(parseGltfCameraProjection) });
export const parseCreateCameraMutation = gltfWirePhase(parseGltfCreateCameraPayload, parseGltfDiff);
