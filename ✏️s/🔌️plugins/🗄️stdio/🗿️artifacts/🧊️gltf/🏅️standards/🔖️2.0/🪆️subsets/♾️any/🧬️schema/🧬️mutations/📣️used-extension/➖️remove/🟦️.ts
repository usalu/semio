/** ➖️ `remove-used-extension` wire twin: the flat `Apply` payload `GltfWithdrawUsedExtensionPayload` and the phase wire `RemoveUsedExtensionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfWithdrawUsedExtensionPayload {
  extension: string;
}

export type RemoveUsedExtensionMutation = GltfPhase<GltfWithdrawUsedExtensionPayload, GltfDiff>;

export const parseGltfWithdrawUsedExtensionPayload = gltfWireObject<GltfWithdrawUsedExtensionPayload>({ extension: gltfWireRequired(gltfWireString) });
export const parseRemoveUsedExtensionMutation = gltfWirePhase(parseGltfWithdrawUsedExtensionPayload, parseGltfDiff);
