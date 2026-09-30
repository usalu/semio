/** 🧩️ `change-document-extension-data` wire twin: the flat `Apply` payload `GltfChangeDocumentExtensionDataPayload` and the phase wire `ChangeDocumentExtensionDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireObject, gltfWireRequired, parseGltfJson } from "../../../📸️snapshot/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export interface GltfChangeDocumentExtensionDataPayload {
  data: GltfJson;
}

export type ChangeDocumentExtensionDataMutation = GltfPhase<GltfChangeDocumentExtensionDataPayload, GltfDiff>;

export const parseGltfChangeDocumentExtensionDataPayload = gltfWireObject<GltfChangeDocumentExtensionDataPayload>({ data: gltfWireRequired(parseGltfJson) });
export const parseChangeDocumentExtensionDataMutation = gltfWirePhase(parseGltfChangeDocumentExtensionDataPayload, parseGltfDiff);
