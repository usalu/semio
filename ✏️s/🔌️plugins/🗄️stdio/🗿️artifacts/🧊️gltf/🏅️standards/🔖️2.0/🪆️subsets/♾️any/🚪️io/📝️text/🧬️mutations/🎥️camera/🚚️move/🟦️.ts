/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveCameraPayload,MoveCameraMutation} from "../../../../../🧬️schema/🧬️mutations/🎥️camera/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎥️camera/🚚️move/🟦️.ts";
/** 🚚️ `move-camera` wire twin: the flat `Apply` payload `GltfMoveCameraPayload` and the phase wire `MoveCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveCameraPayload = gltfWireObject<GltfMoveCameraPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveCameraMutation = gltfWirePhase(parseGltfMoveCameraPayload, parseGltfDiff);
