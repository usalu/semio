/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteCameraPayload,DeleteCameraMutation} from "../../../../../🧬️schema/🧬️mutations/🎥️camera/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎥️camera/🗑️delete/🟦️.ts";
/** 🗑️ `delete-camera` wire twin: the flat `Apply` payload `GltfDeleteCameraPayload` and the phase wire `DeleteCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteCameraPayload = gltfWireObject<GltfDeleteCameraPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteCameraMutation = gltfWirePhase(parseGltfDeleteCameraPayload, parseGltfDiff);
