/** ✂️ `unbind-node-camera` wire twin: the flat `Apply` payload `GltfUnbindNodeCameraPayload` and the phase wire `UnbindNodeCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnbindNodeCameraPayload {
  node: number;
}

export type UnbindNodeCameraMutation = GltfPhase<GltfUnbindNodeCameraPayload, GltfDiff>;

export const parseGltfUnbindNodeCameraPayload = gltfWireObject<GltfUnbindNodeCameraPayload>({ node: gltfWireRequired(gltfWireIndex) });
export const parseUnbindNodeCameraMutation = gltfWirePhase(parseGltfUnbindNodeCameraPayload, parseGltfDiff);
