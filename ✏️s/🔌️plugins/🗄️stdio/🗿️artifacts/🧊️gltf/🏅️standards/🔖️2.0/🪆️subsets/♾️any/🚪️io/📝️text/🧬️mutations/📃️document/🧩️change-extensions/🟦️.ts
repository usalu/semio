/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangeDocumentExtensionDataPayload,ChangeDocumentExtensionDataMutation} from "../../../../../🧬️schema/🧬️mutations/📃️document/🧩️change-extensions/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📃️document/🧩️change-extensions/🟦️.ts";
/** 🧩️ `change-document-extension-data` wire twin: the flat `Apply` payload `GltfChangeDocumentExtensionDataPayload` and the phase wire `ChangeDocumentExtensionDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireObject, gltfWireRequired, parseGltfJson } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { type GltfDiff, type GltfPhase, gltfWirePhase, parseGltfDiff } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangeDocumentExtensionDataPayload = gltfWireObject<GltfChangeDocumentExtensionDataPayload>({ data: gltfWireRequired(parseGltfJson) });
export const parseChangeDocumentExtensionDataMutation = gltfWirePhase(parseGltfChangeDocumentExtensionDataPayload, parseGltfDiff);
