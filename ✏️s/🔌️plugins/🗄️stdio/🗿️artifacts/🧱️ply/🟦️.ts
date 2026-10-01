/** 🗄️ stdio.ply TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;

export type { PlySnapshot, PlyFormat, PlyScalarType, PlyProperty, PlyValue, PlyRow, PlyElement } from "./🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export { parsePlySnapshot } from "./🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export { PLY_SQLITE_SCHEMA, plySnapshotToSqliteDatabase, plySnapshotFromSqliteDatabase, plySnapshotValidateSqliteSubset } from "./🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
