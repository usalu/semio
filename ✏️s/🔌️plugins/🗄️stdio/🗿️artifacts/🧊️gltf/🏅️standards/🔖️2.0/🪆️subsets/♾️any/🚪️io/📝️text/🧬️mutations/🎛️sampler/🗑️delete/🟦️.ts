/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeleteSamplerPayload,DeleteSamplerMutation} from "../../../../../🧬️schema/🧬️mutations/🎛️sampler/🗑️delete/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎛️sampler/🗑️delete/🟦️.ts";
/** 🗑️ `delete-sampler` wire twin: the flat `Apply` payload `GltfDeleteSamplerPayload` and the phase wire `DeleteSamplerMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeleteSamplerPayload = gltfWireObject<GltfDeleteSamplerPayload>({ index: gltfWireRequired(gltfWireIndex) });
export const parseDeleteSamplerMutation = gltfWirePhase(parseGltfDeleteSamplerPayload, parseGltfDiff);
