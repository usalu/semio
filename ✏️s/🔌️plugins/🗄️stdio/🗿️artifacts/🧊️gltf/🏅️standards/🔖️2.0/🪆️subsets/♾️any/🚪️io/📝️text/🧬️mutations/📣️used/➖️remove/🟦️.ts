/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfWithdrawUsedExtensionPayload,RemoveUsedExtensionMutation} from "../../../../../🧬️schema/🧬️mutations/📣️used/➖️remove/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📣️used/➖️remove/🟦️.ts";
/** ➖️ `remove-used-extension` wire twin: the flat `Apply` payload `GltfWithdrawUsedExtensionPayload` and the phase wire `RemoveUsedExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfWithdrawUsedExtensionPayload = gltfWireObject<GltfWithdrawUsedExtensionPayload>({ extension: gltfWireRequired(gltfWireString) });
export const parseRemoveUsedExtensionMutation = gltfWireApplyPhase(parseGltfWithdrawUsedExtensionPayload);
