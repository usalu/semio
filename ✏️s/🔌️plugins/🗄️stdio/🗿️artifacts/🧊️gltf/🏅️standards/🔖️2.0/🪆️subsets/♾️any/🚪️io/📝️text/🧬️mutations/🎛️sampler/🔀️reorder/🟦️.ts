/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfReorderSamplersPayload,ReorderSamplersMutation} from "../../../../../🧬️schema/🧬️mutations/🎛️sampler/🔀️reorder/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎛️sampler/🔀️reorder/🟦️.ts";
/** 🔀️ `reorder-samplers` wire twin: the flat `Apply` payload `GltfReorderSamplersPayload` and the phase wire `ReorderSamplersMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfReorderSamplersPayload = gltfWireObject<GltfReorderSamplersPayload>({ order: gltfWireRequired(gltfWireArray(gltfWireIndex)) });
export const parseReorderSamplersMutation = gltfWirePhase(parseGltfReorderSamplersPayload, parseGltfDiff);
