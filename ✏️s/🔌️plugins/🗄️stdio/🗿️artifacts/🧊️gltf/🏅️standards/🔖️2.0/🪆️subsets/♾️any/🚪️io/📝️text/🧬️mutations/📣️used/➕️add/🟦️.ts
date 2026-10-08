/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfDeclareUsedExtensionPayload,AddUsedExtensionMutation} from "../../../../../🧬️schema/🧬️mutations/📣️used/➕️add/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📣️used/➕️add/🟦️.ts";
/** ➕️ `add-used-extension` wire twin: the flat `Apply` payload `GltfDeclareUsedExtensionPayload` and the phase wire `AddUsedExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfDeclareUsedExtensionPayload = gltfWireObject<GltfDeclareUsedExtensionPayload>({ extension: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseAddUsedExtensionMutation = gltfWireApplyPhase(parseGltfDeclareUsedExtensionPayload);
