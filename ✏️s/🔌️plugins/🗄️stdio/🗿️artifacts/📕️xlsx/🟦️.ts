/** 🗄️ stdio.xlsx TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export{validateXlsxSnapshotProfile}from"./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🟦️.ts";
export type{XlsxProfileDiagnostic}from"./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🟦️.ts";
export type {XlsxCell,XlsxCellValue,XlsxSheet,XlsxSnapshot,XlsxWorkbook,XlsxXmlPart,OpcPackage,OpcPart,OpcRelationship}from"./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export{XLSX_SQLITE_SCHEMA,xlsxSnapshotToSqliteDatabase,xlsxSnapshotFromSqliteDatabase}from"./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
