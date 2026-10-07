/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateImagePayload,CreateImageMutation} from "../../../../../🧬️schema/🧬️mutations/🖼️image/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🖼️image/🌱️create/🟦️.ts";
/** 🌱️ `create-image` wire twin: the flat `Apply` payload `GltfCreateImagePayload` and the phase wire `CreateImageMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateImagePayload = gltfWireObject<GltfCreateImagePayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateImageMutation = gltfWirePhase(parseGltfCreateImagePayload, parseGltfDiff);
