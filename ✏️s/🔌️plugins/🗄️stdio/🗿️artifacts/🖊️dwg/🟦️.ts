/** 🗄️ stdio.dwg TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export type { DwgSnapshot } from "./🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export { DWG_SQLITE_SCHEMA,dwgSnapshotToSqliteDatabase,dwgSnapshotFromSqliteDatabase,dwgSnapshotValidateSqliteSubset } from "./🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
