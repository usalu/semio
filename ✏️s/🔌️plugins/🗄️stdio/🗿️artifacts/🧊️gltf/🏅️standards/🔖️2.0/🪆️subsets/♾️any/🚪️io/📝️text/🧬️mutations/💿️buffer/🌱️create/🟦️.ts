/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateBufferPayload,CreateBufferMutation} from "../../../../../🧬️schema/🧬️mutations/💿️buffer/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/💿️buffer/🌱️create/🟦️.ts";
/** 🌱️ `create-buffer` wire twin: the flat `Apply` payload `GltfCreateBufferPayload` and the phase wire `CreateBufferMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireArray, gltfWireByte, gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, parseGltfBuffer } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateBufferPayload = gltfWireObject<GltfCreateBufferPayload>({ position: gltfWireRequired(gltfWireIndex), bytes: gltfWireRequired(gltfWireArray(gltfWireByte)), buffer: gltfWireOptional(parseGltfBuffer) });
export const parseCreateBufferMutation = gltfWireApplyPhase(parseGltfCreateBufferPayload);
