/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfUnbindNodeCameraPayload,UnbindNodeCameraMutation} from "../../../../../🧬️schema/🧬️mutations/📷️node-camera/✂️unbind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📷️node-camera/✂️unbind/🟦️.ts";
/** ✂️ `unbind-node-camera` wire twin: the flat `Apply` payload `GltfUnbindNodeCameraPayload` and the phase wire `UnbindNodeCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfUnbindNodeCameraPayload = gltfWireObject<GltfUnbindNodeCameraPayload>({ node: gltfWireRequired(gltfWireIndex) });
export const parseUnbindNodeCameraMutation = gltfWirePhase(parseGltfUnbindNodeCameraPayload, parseGltfDiff);
