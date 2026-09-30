/** ➕️ `add-used-extension` wire twin: the flat `Apply` payload `GltfDeclareUsedExtensionPayload` and the phase wire `AddUsedExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeclareUsedExtensionPayload {
  extension: string;
  position: number;
}

export type AddUsedExtensionMutation = GltfPhase<GltfDeclareUsedExtensionPayload, GltfDiff>;

export const parseGltfDeclareUsedExtensionPayload = gltfWireObject<GltfDeclareUsedExtensionPayload>({ extension: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseAddUsedExtensionMutation = gltfWirePhase(parseGltfDeclareUsedExtensionPayload, parseGltfDiff);
