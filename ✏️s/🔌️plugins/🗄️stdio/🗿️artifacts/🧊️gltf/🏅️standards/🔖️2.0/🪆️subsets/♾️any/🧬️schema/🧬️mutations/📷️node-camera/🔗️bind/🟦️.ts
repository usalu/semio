/** 🔗️ `bind-node-camera` wire twin: the flat `Apply` payload `GltfBindNodeCameraPayload` and the phase wire `BindNodeCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfBindNodeCameraPayload {
  node: bigint;
  camera: bigint;
}

export type BindNodeCameraMutation = GltfPhase<GltfBindNodeCameraPayload, GltfDiff>;

export const parseGltfBindNodeCameraPayload = gltfWireObject<GltfBindNodeCameraPayload>({ node: gltfWireRequired(gltfWireIndex), camera: gltfWireRequired(gltfWireIndex) });
export const parseBindNodeCameraMutation = gltfWirePhase(parseGltfBindNodeCameraPayload, parseGltfDiff);
