/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfUnrequireExtensionPayload,RemoveRequiredExtensionMutation} from "../../../../../🧬️schema/🧬️mutations/✅️required/➖️remove/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/✅️required/➖️remove/🟦️.ts";
/** ➖️ `remove-required-extension` wire twin: the flat `Apply` payload `GltfUnrequireExtensionPayload` and the phase wire `RemoveRequiredExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfUnrequireExtensionPayload = gltfWireObject<GltfUnrequireExtensionPayload>({ extension: gltfWireRequired(gltfWireString) });
export const parseRemoveRequiredExtensionMutation = gltfWirePhase(parseGltfUnrequireExtensionPayload, parseGltfDiff);
