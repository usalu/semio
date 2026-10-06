/** 🗄️ stdio.pptx TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export{validatePptxSnapshotProfile}from"./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🟦️.ts";
export type{PptxProfileDiagnostic}from"./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🟦️.ts";

export type{PptxSnapshot,PptxXmlPart,PptxPresentation,PptxSlide,PptxShape,PptxParagraph,PptxRun,PptxTransform}from"./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export{PPTX_SQLITE_SCHEMA,pptxSnapshotToSqliteDatabase,pptxSnapshotFromSqliteDatabase}from"./🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
