/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderCamerasPayload,ReorderCamerasMutation} from "../../../../../🧬️schema/🧬️mutations/🎥️camera/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎥️camera/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-cameras` wire twin: the flat `Apply` payload `GltfReorderCamerasPayload` and the phase wire `ReorderCamerasMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderCamerasPayload = gltfWireObject<GltfReorderCamerasPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderCamerasMutation = gltfWireApplyPhase(parseGltfReorderCamerasPayload);
