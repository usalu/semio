/** 🗄️ stdio.docx TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export{validateDocxSnapshotProfile}from"./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🟦️.ts";
export type{DocxProfileDiagnostic}from"./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🟦️.ts";

export type{DocxSnapshot,DocxXmlPart}from"./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export{DOCX_SQLITE_SCHEMA,docxSnapshotToSqliteDatabase,docxSnapshotFromSqliteDatabase}from"./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";

export type { DocxMutation, DocxSetPart, DocxPartContent } from "./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts";
export { parseDocxSetPart, parseDocxPartContent } from "./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦set-part/🟦️.ts";
export { encodeDocxSetPartText, decodeDocxSetPartText } from "./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🟦️.ts";
export { encodeDocxSetPartBinary, decodeDocxSetPartBinary } from "./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/💾️binary/🧬️mutations/🟦️.ts";
