/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfCreateBufferViewPayload,CreateBufferViewMutation} from "../../../../../🧬️schema/🧬️mutations/🪟️buffer-view/🌱️create/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🪟️buffer-view/🌱️create/🟦️.ts";
/** 🌱️ `create-buffer-view` wire twin: the flat `Apply` payload `GltfCreateBufferViewPayload` and the phase wire `CreateBufferViewMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireOptional, parseGltfBufferView } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfCreateBufferViewPayload = gltfWireObject<GltfCreateBufferViewPayload>({ position: gltfWireRequired(gltfWireIndex), buffer: gltfWireRequired(gltfWireIndex), byteOffset: gltfWireRequired(gltfWireIndex), byteLength: gltfWireRequired(gltfWireIndex), bufferView: gltfWireOptional(parseGltfBufferView) });
export const parseCreateBufferViewMutation = gltfWireApplyPhase(parseGltfCreateBufferViewPayload);
