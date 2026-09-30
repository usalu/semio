/** ➕️ `add-required-extension` wire twin: the flat `Apply` payload `GltfRequireExtensionPayload` and the phase wire `AddRequiredExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfRequireExtensionPayload {
  extension: string;
  position: number;
}

export type AddRequiredExtensionMutation = GltfPhase<GltfRequireExtensionPayload, GltfDiff>;

export const parseGltfRequireExtensionPayload = gltfWireObject<GltfRequireExtensionPayload>({ extension: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseAddRequiredExtensionMutation = gltfWirePhase(parseGltfRequireExtensionPayload, parseGltfDiff);
