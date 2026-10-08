/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateImagePayload,CreateImageMutation} from "../../../../../🧬️schema/🧬️mutations/🖼️image/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🖼️image/🌱️create/🟦️.ts";
/** 🌱️ `create-image` wire twin: the flat `Apply` payload `GltfCreateImagePayload` and the phase wire `CreateImageMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, parseGltfImage } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateImagePayload = gltfWireObject<GltfCreateImagePayload>({ position: gltfWireRequired(gltfWireIndex), image: gltfWireOptional(parseGltfImage) });
export const parseCreateImageMutation = gltfWireApplyPhase(parseGltfCreateImagePayload);
