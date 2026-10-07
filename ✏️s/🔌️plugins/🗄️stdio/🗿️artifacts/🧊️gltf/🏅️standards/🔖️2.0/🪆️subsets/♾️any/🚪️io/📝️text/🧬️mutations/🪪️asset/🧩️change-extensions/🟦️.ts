/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangeAssetExtensionDataPayload,ChangeAssetExtensionDataMutation} from "../../../../../🧬️schema/🧬️mutations/🪪️asset/🧩️change-extensions/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🪪️asset/🧩️change-extensions/🟦️.ts";
/** 🧩️ `change-asset-extension-data` wire twin: the flat `Apply` payload `GltfChangeAssetExtensionDataPayload` and the phase wire `ChangeAssetExtensionDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireObject, gltfWireRequired, parseGltfJson } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangeAssetExtensionDataPayload = gltfWireObject<GltfChangeAssetExtensionDataPayload>({ data: gltfWireRequired(parseGltfJson) });
export const parseChangeAssetExtensionDataMutation = gltfWirePhase(parseGltfChangeAssetExtensionDataPayload, parseGltfDiff);
