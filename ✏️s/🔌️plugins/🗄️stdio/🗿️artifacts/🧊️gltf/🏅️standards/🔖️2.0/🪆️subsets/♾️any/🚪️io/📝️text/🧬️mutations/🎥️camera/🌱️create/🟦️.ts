/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateCameraPayload,CreateCameraMutation} from "../../../../../🧬️schema/🧬️mutations/🎥️camera/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎥️camera/🌱️create/🟦️.ts";
/** 🌱️ `create-camera` wire twin: the flat `Apply` payload `GltfCreateCameraPayload` and the phase wire `CreateCameraMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfCameraProjection, gltfWireIndex, gltfWireObject, gltfWireRequired, parseGltfCameraProjection } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateCameraPayload = gltfWireObject<GltfCreateCameraPayload>({ position: gltfWireRequired(gltfWireIndex), projection: gltfWireRequired(parseGltfCameraProjection) });
export const parseCreateCameraMutation = gltfWirePhase(parseGltfCreateCameraPayload, parseGltfDiff);
