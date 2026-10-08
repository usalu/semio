/** 🌱️ `create-camera` wire twin: the flat `Apply` payload `GltfCreateCameraPayload` and the phase wire `CreateCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfCameraProjection, gltfWireIndex, gltfWireObject, gltfWireRequired, parseGltfCameraProjection, gltfWireOptional, type GltfCamera, parseGltfCamera } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfCreateCameraPayload {
  position: bigint;
  projection: GltfCameraProjection;
  camera?: GltfCamera;
}

export type CreateCameraMutation = GltfApplyPhase<GltfCreateCameraPayload>;

export const parseGltfCreateCameraPayload = gltfWireObject<GltfCreateCameraPayload>({ position: gltfWireRequired(gltfWireIndex), projection: gltfWireRequired(parseGltfCameraProjection), camera: gltfWireOptional(parseGltfCamera) });
export const parseCreateCameraMutation = gltfWireApplyPhase(parseGltfCreateCameraPayload);
