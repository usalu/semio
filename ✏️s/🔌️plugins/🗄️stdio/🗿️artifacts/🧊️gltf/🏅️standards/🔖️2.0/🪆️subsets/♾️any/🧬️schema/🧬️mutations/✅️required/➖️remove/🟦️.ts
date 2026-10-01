/** ➖️ `remove-required-extension` wire twin: the flat `Apply` payload `GltfUnrequireExtensionPayload` and the phase wire `RemoveRequiredExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfUnrequireExtensionPayload {
  extension: string;
}

export type RemoveRequiredExtensionMutation = GltfPhase<GltfUnrequireExtensionPayload, GltfDiff>;

export const parseGltfUnrequireExtensionPayload = gltfWireObject<GltfUnrequireExtensionPayload>({ extension: gltfWireRequired(gltfWireString) });
export const parseRemoveRequiredExtensionMutation = gltfWirePhase(parseGltfUnrequireExtensionPayload, parseGltfDiff);
