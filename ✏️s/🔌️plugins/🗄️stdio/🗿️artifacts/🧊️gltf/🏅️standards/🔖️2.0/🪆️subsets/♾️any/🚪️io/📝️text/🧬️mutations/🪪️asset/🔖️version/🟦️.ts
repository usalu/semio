/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangeAssetVersionPayload,ChangeAssetVersionMutation} from "../../../../../🧬️schema/🧬️mutations/🪪️asset/🔖️version/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🪪️asset/🔖️version/🟦️.ts";
/** 🔖️ `change-asset-version` wire twin: the flat `Apply` payload `GltfChangeAssetVersionPayload` and the phase wire `ChangeAssetVersionMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangeAssetVersionPayload = gltfWireObject<GltfChangeAssetVersionPayload>({ version: gltfWireRequired(gltfWireString) });
export const parseChangeAssetVersionMutation = gltfWireApplyPhase(parseGltfChangeAssetVersionPayload);
