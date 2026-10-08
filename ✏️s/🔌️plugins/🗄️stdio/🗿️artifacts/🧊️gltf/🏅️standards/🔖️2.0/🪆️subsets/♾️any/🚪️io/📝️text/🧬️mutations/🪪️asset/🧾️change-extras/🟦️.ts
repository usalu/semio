/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangeAssetExtraDataPayload,ChangeAssetExtraDataMutation} from "../../../../../🧬️schema/🧬️mutations/🪪️asset/🧾️change-extras/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🪪️asset/🧾️change-extras/🟦️.ts";
/** 🧾️ `change-asset-extra-data` wire twin: the flat `Apply` payload `GltfChangeAssetExtraDataPayload` and the phase wire `ChangeAssetExtraDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireObject, gltfWireRequired, parseGltfJson } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangeAssetExtraDataPayload = gltfWireObject<GltfChangeAssetExtraDataPayload>({ data: gltfWireRequired(parseGltfJson) });
export const parseChangeAssetExtraDataMutation = gltfWireApplyPhase(parseGltfChangeAssetExtraDataPayload);
