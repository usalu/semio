/** 🚪️ Native JSON member lowering with canonical semantic models. */
import type {GltfChangeDocumentExtraDataPayload,ChangeDocumentExtraDataMutation} from "../../../../../🧬️schema/🧬️mutations/📃️document/📝️change-extras/🟦️.ts";
export type * from "../../../../../🧬️schema/🧬️mutations/📃️document/📝️change-extras/🟦️.ts";
/** 📝️ `change-document-extra-data` wire twin: the flat `Apply` payload `GltfChangeDocumentExtraDataPayload` and the phase wire `ChangeDocumentExtraDataMutation`, exactly as `./🦀️.rs` writes them.
 * @see ./🧬️schema/🔣️.json */
import { type GltfJson, gltfWireObject, gltfWireRequired, parseGltfJson } from "../../../📸️snapshot/🔣️json/🟦️.ts";
import { gltfWireApplyPhase } from "../../../🔺️diff/🟦️.ts";

export const parseGltfChangeDocumentExtraDataPayload = gltfWireObject<GltfChangeDocumentExtraDataPayload>({ data: gltfWireRequired(parseGltfJson) });
export const parseChangeDocumentExtraDataMutation = gltfWireApplyPhase(parseGltfChangeDocumentExtraDataPayload);
