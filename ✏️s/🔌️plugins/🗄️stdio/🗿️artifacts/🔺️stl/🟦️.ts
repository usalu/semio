/** 🗄️ stdio.stl TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;

export type { StlSnapshot, StlTriangle } from "./🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export { STL_SQLITE_SCHEMA, stlSnapshotToSqliteDatabase, stlSnapshotFromSqliteDatabase, stlSnapshotValidateSqliteSubset } from "./🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
