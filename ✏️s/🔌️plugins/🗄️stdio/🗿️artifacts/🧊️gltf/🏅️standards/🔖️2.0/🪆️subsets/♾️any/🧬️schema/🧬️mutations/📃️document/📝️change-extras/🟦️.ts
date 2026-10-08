/** 📝️ `change-document-extra-data` wire twin: the flat `Apply` payload `GltfChangeDocumentExtraDataPayload` and the phase wire `ChangeDocumentExtraDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireObject, gltfWireRequired, parseGltfJson } from "../../../📸️snapshot/🟦️.ts";
import { type GltfApplyPhase, gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeDocumentExtraDataPayload {
  data: GltfJson;
}

export type ChangeDocumentExtraDataMutation = GltfApplyPhase<GltfChangeDocumentExtraDataPayload>;

export const parseGltfChangeDocumentExtraDataPayload = gltfWireObject<GltfChangeDocumentExtraDataPayload>({ data: gltfWireRequired(parseGltfJson) });
export const parseChangeDocumentExtraDataMutation = gltfWireApplyPhase(parseGltfChangeDocumentExtraDataPayload);
