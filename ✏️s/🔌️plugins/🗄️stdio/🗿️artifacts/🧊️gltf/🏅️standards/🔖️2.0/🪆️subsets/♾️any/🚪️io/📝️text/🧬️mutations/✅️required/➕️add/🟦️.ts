/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfRequireExtensionPayload,AddRequiredExtensionMutation} from "../../../../../🧬️schema/🧬️mutations/✅️required/➕️add/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/✅️required/➕️add/🟦️.ts";
/** ➕️ `add-required-extension` wire twin: the flat `Apply` payload `GltfRequireExtensionPayload` and the phase wire `AddRequiredExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfRequireExtensionPayload = gltfWireObject<GltfRequireExtensionPayload>({ extension: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseAddRequiredExtensionMutation = gltfWirePhase(parseGltfRequireExtensionPayload, parseGltfDiff);
