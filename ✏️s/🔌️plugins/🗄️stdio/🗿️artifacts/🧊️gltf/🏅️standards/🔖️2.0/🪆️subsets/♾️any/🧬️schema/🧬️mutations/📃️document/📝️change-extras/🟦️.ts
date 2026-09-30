/** 📝️ `change-document-extra-data` wire twin: the flat `Apply` payload `GltfChangeDocumentExtraDataPayload` and the phase wire `ChangeDocumentExtraDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireObject, gltfWireRequired, parseGltfJson } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeDocumentExtraDataPayload {
  data: GltfJson;
}

export type ChangeDocumentExtraDataMutation = GltfPhase<GltfChangeDocumentExtraDataPayload, GltfDiff>;

export const parseGltfChangeDocumentExtraDataPayload = gltfWireObject<GltfChangeDocumentExtraDataPayload>({ data: gltfWireRequired(parseGltfJson) });
export const parseChangeDocumentExtraDataMutation = gltfWirePhase(parseGltfChangeDocumentExtraDataPayload, parseGltfDiff);
