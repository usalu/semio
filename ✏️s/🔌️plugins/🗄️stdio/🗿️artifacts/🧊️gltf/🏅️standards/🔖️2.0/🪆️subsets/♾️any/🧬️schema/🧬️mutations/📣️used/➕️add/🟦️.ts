/** ➕️ `add-used-extension` wire twin: the flat `Apply` payload `GltfDeclareUsedExtensionPayload` and the phase wire `AddUsedExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireIndex, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfDeclareUsedExtensionPayload {
  extension: string;
  position: bigint;
}

export type AddUsedExtensionMutation = GltfApplyPhase<GltfDeclareUsedExtensionPayload>;

export const parseGltfDeclareUsedExtensionPayload = gltfWireObject<GltfDeclareUsedExtensionPayload>({ extension: gltfWireRequired(gltfWireString), position: gltfWireRequired(gltfWireIndex) });
export const parseAddUsedExtensionMutation = gltfWireApplyPhase(parseGltfDeclareUsedExtensionPayload);
