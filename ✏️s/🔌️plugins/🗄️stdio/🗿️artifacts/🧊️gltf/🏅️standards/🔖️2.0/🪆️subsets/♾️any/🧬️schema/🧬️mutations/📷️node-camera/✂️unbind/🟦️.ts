/** ✂️ `unbind-node-camera` wire twin: the flat `Apply` payload `GltfUnbindNodeCameraPayload` and the phase wire `UnbindNodeCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindNodeCameraPayload {
  node: bigint;
}

export type UnbindNodeCameraMutation = GltfApplyPhase<GltfUnbindNodeCameraPayload>;

export const parseGltfUnbindNodeCameraPayload = gltfWireObject<GltfUnbindNodeCameraPayload>({ node: gltfWireRequired(gltfWireIndex) });
export const parseUnbindNodeCameraMutation = gltfWireApplyPhase(parseGltfUnbindNodeCameraPayload);
