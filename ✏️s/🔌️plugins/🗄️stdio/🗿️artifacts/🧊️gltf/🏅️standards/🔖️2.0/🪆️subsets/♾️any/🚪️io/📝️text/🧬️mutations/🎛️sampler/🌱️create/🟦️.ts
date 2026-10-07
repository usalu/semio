/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateSamplerPayload,CreateSamplerMutation} from "../../../../../🧬️schema/🧬️mutations/🎛️sampler/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎛️sampler/🌱️create/🟦️.ts";
/** 🌱️ `create-sampler` wire twin: the flat `Apply` payload `GltfCreateSamplerPayload` and the phase wire `CreateSamplerMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateSamplerPayload = gltfWireObject<GltfCreateSamplerPayload>({ position: gltfWireRequired(gltfWireIndex) });
export const parseCreateSamplerMutation = gltfWirePhase(parseGltfCreateSamplerPayload, parseGltfDiff);
