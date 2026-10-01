/** 🗄️ stdio.las TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;

export type { LasSnapshot, LasHeader, LasPoint, LasVlr } from "./🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/🟦️.ts";
export { LAS_SQLITE_SCHEMA, lasSnapshotToSqliteDatabase, lasSnapshotFromSqliteDatabase, lasSnapshotValidateSqliteSubset } from "./🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
