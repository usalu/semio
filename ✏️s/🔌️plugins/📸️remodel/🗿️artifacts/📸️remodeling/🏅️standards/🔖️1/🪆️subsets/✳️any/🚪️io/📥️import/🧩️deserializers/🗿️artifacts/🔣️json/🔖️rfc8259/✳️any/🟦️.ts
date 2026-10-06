/** 🔣️ `s.stdio.json@rfc8259` → remodeling snapshot, fidelity `Exact`.
 *
 *  Validation is total: an unknown key, a missing key that carries no `#[serde(default)]`, a wrong
 *  JSON type, an unknown enum lexeme or a mis-sized fixed array is a hard `RemodelingCodecError`
 *  naming the offending path. Nothing is coerced silently, so a fixture that drifts from the schema
 *  fails loudly here rather than surviving into an apply.
 */

export {RemodelingCodecError} from "../../../../../../../🧬️schema/📸️snapshot/🟦️.ts";
export {decodeRecord,decodeValue} from "../../../../../../📝️text/📸️snapshot/🔣️json/🟦️.ts";
export {decodeRemodelingSnapshot, remodelingSnapshotFromJsonText} from "../../../../../../📝️text/📸️snapshot/🔣️json/🟦️.ts";
export {decodeRemodelingDiff} from "../../../../../../📝️text/🔺️diff/🔣️json/🟦️.ts";
export {decodeRemodelingMutation} from "../../../../../../📝️text/🧬️mutations/🔣️json/🟦️.ts";

/** 🚫 Kept as the historical alias for this leaf's own refusal type. */
export {RemodelingCodecError as RemodelingDecodeError} from "../../../../../../../🧬️schema/📸️snapshot/🟦️.ts";
