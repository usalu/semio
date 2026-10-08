/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfBindNodeCameraPayload,BindNodeCameraMutation} from "../../../../../🧬️schema/🧬️mutations/📷️node-camera/🔗️bind/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📷️node-camera/🔗️bind/🟦️.ts";
/** 🔗️ `bind-node-camera` wire twin: the flat `Apply` payload `GltfBindNodeCameraPayload` and the phase wire `BindNodeCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfBindNodeCameraPayload = gltfWireObject<GltfBindNodeCameraPayload>({ node: gltfWireRequired(gltfWireIndex), camera: gltfWireRequired(gltfWireIndex) });
export const parseBindNodeCameraMutation = gltfWireApplyPhase(parseGltfBindNodeCameraPayload);
