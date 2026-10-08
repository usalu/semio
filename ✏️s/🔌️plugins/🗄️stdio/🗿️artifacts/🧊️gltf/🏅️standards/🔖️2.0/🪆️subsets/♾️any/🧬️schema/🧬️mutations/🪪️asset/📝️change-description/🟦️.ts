/** 📝️ `change-asset-descriptive-metadata` wire twin: the flat `Apply` payload `GltfChangeAssetDescriptiveMetadataPayload` and the phase wire `ChangeAssetDescriptiveMetadataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { gltfWireNullable, gltfWireObject, gltfWireRequired, gltfWireString } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeAssetDescriptiveMetadataPayload {
  generator: string | null;
  copyright: string | null;
  minVersion: string | null;
}

export type ChangeAssetDescriptiveMetadataMutation = GltfApplyPhase<GltfChangeAssetDescriptiveMetadataPayload>;

export const parseGltfChangeAssetDescriptiveMetadataPayload = gltfWireObject<GltfChangeAssetDescriptiveMetadataPayload>({ generator: gltfWireRequired(gltfWireNullable(gltfWireString)), copyright: gltfWireRequired(gltfWireNullable(gltfWireString)), minVersion: gltfWireRequired(gltfWireNullable(gltfWireString)) });
export const parseChangeAssetDescriptiveMetadataMutation = gltfWireApplyPhase(parseGltfChangeAssetDescriptiveMetadataPayload);
