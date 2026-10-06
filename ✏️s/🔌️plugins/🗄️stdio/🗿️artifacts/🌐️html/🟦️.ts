/** 🎪 stdio.html TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;

export { HTML_SQLITE_SCHEMA, htmlSnapshotToSqliteDatabase, htmlSnapshotFromSqliteDatabase } from "./🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
export type { HtmlSnapshot, HtmlNode, HtmlAttr, RawTextKind } from "./🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
