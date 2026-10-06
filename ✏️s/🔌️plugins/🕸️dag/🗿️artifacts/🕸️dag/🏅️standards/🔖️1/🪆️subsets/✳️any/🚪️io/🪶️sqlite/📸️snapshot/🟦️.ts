/** 🕸️ The DAG parent persists its marker and literal Graph child independently of local scenes. */
import { parseDagArtifact, type DagArtifact } from "../../../🧬️schema/🟦️.ts";
import type { ArtifactDialect } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import type { SqliteDatabase, SqliteRow } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { ArtifactSqliteProjection, artifactSqliteTables, artifactSqliteInteger, artifactSqliteText, artifactSqliteCheckpoint, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";

export const DAG_SQLITE_SCHEMA = String.raw`CREATE TABLE dag_document (
 id INTEGER PRIMARY KEY,
 schema TEXT NOT NULL
);
CREATE TABLE dag_content_child (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES dag_document(id),
 child_id TEXT NOT NULL,
 artifact_id TEXT NOT NULL,
 artifact_kind TEXT NOT NULL,
 standard TEXT NOT NULL,
 subset TEXT NOT NULL
);
`;

/** 🎯️ The parent capability has one authored coordinate. */
export function validateDagSnapshotSqliteDialect(dialect: ArtifactDialect): void {
  if (dialect.artifactKind !== "s.dag.dag" || dialect.standard !== "1" || dialect.subset !== "*") throw new Error("DAG SQLite dialect mismatch");
}

/** 📤️ Project the actual two-field Snapshot with admission before allocating relational rows. */
export async function dagSnapshotToSqliteDatabase(snapshot: DagArtifact, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, 2);
  if ((options.maxRows ?? 1_000_000) < 2) throw new Error("DAG SQLite row limit");
  const value = parseDagArtifact(snapshot), out = await ArtifactSqliteProjection.create(DAG_SQLITE_SCHEMA, options);
  out.checkRowsAdditional(2);
  const document = await out.insert("dag_document", [value.schema]);
  const child = value.content, target = child.target;
  await out.insert("dag_content_child", [document, child.childId, target.artifactId, target.dialect.artifactKind, target.dialect.standard, target.dialect.subset]);
  await artifactSqliteCheckpoint(options, "projectSnapshot", 2, 2);
  return out.finish();
}

function singleton(rows: readonly SqliteRow[], width: number): SqliteRow {
  if (rows.length !== 1) throw new Error("DAG SQLite singleton cardinality");
  const row = rows[0]!;
  if (row.values.length !== width || row.rowid <= 0n || artifactSqliteInteger(row, 0) !== row.rowid) throw new Error("DAG SQLite identity or width");
  return row;
}

/** 📥️ Rebuild only the persisted parent; unresolved addresses remain independent literal fields. */
export async function dagSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<DagArtifact> {
  const tables = await artifactSqliteTables(database, DAG_SQLITE_SCHEMA, options);
  const document = singleton(tables[0]!, 2), child = singleton(tables[1]!, 7);
  if (artifactSqliteInteger(child, 1) !== document.rowid) throw new Error("DAG SQLite orphan child relationship");
  const value = parseDagArtifact({ schema: artifactSqliteText(document, 1), content: { childId: artifactSqliteText(child, 2), target: { artifactId: artifactSqliteText(child, 3), dialect: { artifactKind: artifactSqliteText(child, 4), standard: artifactSqliteText(child, 5), subset: artifactSqliteText(child, 6) } } } });
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 2, 2);
  return value;
}
