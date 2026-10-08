/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfMoveSamplerPayload,MoveSamplerMutation} from "../../../../../🧬️schema/🧬️mutations/🎛️sampler/🚚️move/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🎛️sampler/🚚️move/🟦️.ts";
/** 🚚️ `move-sampler` wire twin: the flat `Apply` payload `GltfMoveSamplerPayload` and the phase wire `MoveSamplerMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfMoveSamplerPayload = gltfWireObject<GltfMoveSamplerPayload>({ index: gltfWireRequired(gltfWireIndex), position: gltfWireRequired(gltfWireIndex) });
export const parseMoveSamplerMutation = gltfWireApplyPhase(parseGltfMoveSamplerPayload);
