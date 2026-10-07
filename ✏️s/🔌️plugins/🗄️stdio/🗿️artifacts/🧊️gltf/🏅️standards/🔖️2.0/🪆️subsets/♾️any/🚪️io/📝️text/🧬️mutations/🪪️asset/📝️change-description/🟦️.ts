/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangeAssetDescriptiveMetadataPayload,ChangeAssetDescriptiveMetadataMutation} from "../../../../../🧬️schema/🧬️mutations/🪪️asset/📝️change-description/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/🪪️asset/📝️change-description/🟦️.ts";
/** 📝️ `change-asset-descriptive-metadata` wire twin: the flat `Apply` payload `GltfChangeAssetDescriptiveMetadataPayload` and the phase wire `ChangeAssetDescriptiveMetadataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireNullable, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangeAssetDescriptiveMetadataPayload = gltfWireObject<GltfChangeAssetDescriptiveMetadataPayload>({ generator: gltfWireRequired(gltfWireNullable(gltfWireString)), copyright: gltfWireRequired(gltfWireNullable(gltfWireString)), minVersion: gltfWireRequired(gltfWireNullable(gltfWireString)) });
export const parseChangeAssetDescriptiveMetadataMutation = gltfWirePhase(parseGltfChangeAssetDescriptiveMetadataPayload, parseGltfDiff);
