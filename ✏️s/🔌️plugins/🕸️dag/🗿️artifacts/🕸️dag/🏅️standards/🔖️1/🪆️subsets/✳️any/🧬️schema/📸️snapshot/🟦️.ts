/** 📸️ Dag snapshot has the exact durable artifact contract. */
export { parseDagArtifact as parseDagSnapshot } from "../🟦️.ts";
export type { DagArtifact as DagSnapshot } from "../🟦️.ts";
export { dagSnapshotToSqliteDatabase, dagSnapshotFromSqliteDatabase, validateDagSnapshotSqliteDialect, DAG_SQLITE_SCHEMA } from "./🪶️sqlite/🟦️.ts";
