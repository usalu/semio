/** ➖️ `remove-required-extension` wire twin: the flat `Apply` payload `GltfUnrequireExtensionPayload` and the phase wire `RemoveRequiredExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnrequireExtensionPayload {
  extension: string;
}

export type RemoveRequiredExtensionMutation = GltfApplyPhase<GltfUnrequireExtensionPayload>;

export const parseGltfUnrequireExtensionPayload = gltfWireObject<GltfUnrequireExtensionPayload>({ extension: gltfWireRequired(gltfWireString) });
export const parseRemoveRequiredExtensionMutation = gltfWireApplyPhase(parseGltfUnrequireExtensionPayload);
