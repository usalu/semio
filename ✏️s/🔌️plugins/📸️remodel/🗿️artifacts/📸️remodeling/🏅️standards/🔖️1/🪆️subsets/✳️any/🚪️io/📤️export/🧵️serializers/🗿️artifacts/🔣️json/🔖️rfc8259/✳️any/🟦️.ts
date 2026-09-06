/** 🔣️ remodeling snapshot → `s.stdio.json@rfc8259`, fidelity `Exact`.
 *
 *  A genuine second writer: member order comes from this package's own `RecordSpec` tables, float
 *  lexemes are re-derived by `floatLexeme` (shortest round-tripping decimal at the field's own
 *  width, `.0` on whole values — `ryu`'s rule, which `serde_json` uses), `BTreeMap` members are
 *  re-sorted, and the layout is `serde_json::to_string_pretty`'s two-space indent. Nothing is
 *  echoed from Rust output.
 */

export { encodeRemodelingSnapshot, floatLexeme, remodelingSnapshotToJsonText, writeRecordJson, writeValueJson } from "../../../../../../../🧬️schema/📸️snapshot/🟦️.ts";
export { encodeRemodelingDiff, remodelingDiffToJsonText } from "../../../../../../../🧬️schema/🔺️diff/🟦️.ts";
