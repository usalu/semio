import { parseBinary64, encodeIeee754Cells, readBinary64, ieee754CellByteLength, type Binary64 } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🔺️ Handcrafted STL solid/facet/vertex relations with intrinsic numeric coordinates. */
import type { StlSnapshot, StlTriangle } from "../🟦️.ts";
import { artifactSqliteCheckpoint, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteDocumentReference, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, artifactSqliteTextBytes, artifactSqliteValueBudget, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase, SqliteRow } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted SQL kept byte-equal to the adjacent schema asset. */
export const STL_SQLITE_SCHEMA = "CREATE TABLE stl_solid (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  schema TEXT NOT NULL,\n  name TEXT NOT NULL\n);\nCREATE TABLE stl_facet (\n  id INTEGER PRIMARY KEY,\n  solid_id INTEGER NOT NULL REFERENCES stl_solid(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  normal_x REAL,\n  normal_y REAL,\n  normal_z REAL,\n  normal_x_ieee754_bits INTEGER NOT NULL,\n  normal_x_numeric_class TEXT NOT NULL CHECK(normal_x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  normal_y_ieee754_bits INTEGER NOT NULL,\n  normal_y_numeric_class TEXT NOT NULL CHECK(normal_y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  normal_z_ieee754_bits INTEGER NOT NULL,\n  normal_z_numeric_class TEXT NOT NULL CHECK(normal_z_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))\n);\nCREATE TABLE stl_vertex (\n  id INTEGER PRIMARY KEY,\n  facet_id INTEGER NOT NULL REFERENCES stl_facet(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal BETWEEN 0 AND 2),\n  x REAL,\n  y REAL,\n  z REAL,\n  x_ieee754_bits INTEGER NOT NULL,\n  x_numeric_class TEXT NOT NULL CHECK(x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  y_ieee754_bits INTEGER NOT NULL,\n  y_numeric_class TEXT NOT NULL CHECK(y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  z_ieee754_bits INTEGER NOT NULL,\n  z_numeric_class TEXT NOT NULL CHECK(z_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))\n);\n";

const COORDINATES = [{index:3,width:64},{index:4,width:64},{index:5,width:64}] as const;
function vector(value: readonly Binary64[]): void {
  if (!Array.isArray(value) || value.length !== 3 || value.some(coordinate => { parseBinary64(coordinate); return false; })) throw new Error("STL coordinates must contain three owned binary64 scalars");
}
function coordinates(row: SqliteRow): [Binary64,Binary64,Binary64] { return [readBinary64(row,3,COORDINATES),readBinary64(row,4,COORDINATES),readBinary64(row,5,COORDINATES)]; }

/** 📤️ Project exact normals and three independent vertex entities per ordered facet. */
export async function stlSnapshotToSqliteDatabase(snapshot: StlSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  const total = snapshot.triangles.length * 4;
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, total);
  if (!Number.isSafeInteger(total) || total + 1 > (options.maxRows ?? 1_000_000)) throw new Error("STL SQLite row limit");
  let bytes=artifactSqliteTextBytes(snapshot.schema)+artifactSqliteTextBytes(snapshot.solidName)+8;
  artifactSqliteValueBudget(bytes,options);
  for (let ordinal = 0; ordinal < snapshot.triangles.length; ordinal++) {
    const triangle = snapshot.triangles[ordinal]!;
    vector(triangle.normal);
    if (!Array.isArray(triangle.vertices) || triangle.vertices.length !== 3) throw new Error("STL facets require three vertices");
    for (const vertex of triangle.vertices) vector(vertex);
    bytes+=96;
    for(const value of triangle.normal)bytes+=ieee754CellByteLength(value,64);
    for(const vertex of triangle.vertices)for(const value of vertex)bytes+=ieee754CellByteLength(value,64);
    artifactSqliteValueBudget(bytes,options);
    if ((ordinal + 1) % 64 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, total);
  }
  const facets: SqliteRow[] = [];
  const vertices: SqliteRow[] = [];
  for (let ordinal = 0; ordinal < snapshot.triangles.length; ordinal++) {
    const triangle = snapshot.triangles[ordinal]!;
    const id = BigInt(ordinal + 1);
    facets.push({ rowid: id, values: encodeIeee754Cells([id,1n,BigInt(ordinal),...triangle.normal],COORDINATES,options.maxColumns) });
    for (let ordinal = 0; ordinal < triangle.vertices.length; ordinal++) {
      const vertexId = BigInt(vertices.length + 1);
      vertices.push({ rowid: vertexId, values: encodeIeee754Cells([vertexId,id,BigInt(ordinal),...triangle.vertices[ordinal]!],COORDINATES,options.maxColumns) });
    }
    if ((ordinal + 1) % 64 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", (ordinal + 1) * 4, total);
  }
  const database = artifactSqliteDatabase(STL_SQLITE_SCHEMA, [[{ rowid: 1n, values: [1n, snapshot.schema, snapshot.solidName] }], facets, vertices], options);
  await artifactSqliteCheckpoint(options, "projectSnapshot", total, total);
  return database;
}

/** 📥️ Restore ordered facets with exactly three owned vertices and persisted normals. */
export async function stlSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<StlSnapshot> {
  const total = database.tables.filter(table => table.name.toLowerCase() !== "stl_solid").reduce((sum, table) => sum + table.rows.length, 0);
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  const [solids, facets, vertexRows] = await artifactSqliteTables(database, STL_SQLITE_SCHEMA, options);
  const solid = artifactSqliteDocument(solids!);
  const ids = new Set<bigint>();
  for (let ordinal = 0; ordinal < facets!.length; ordinal++) {
    const row = facets![ordinal]!;
    if (row.rowid < 1n) throw new Error("STL facet identities must be positive");
    ids.add(row.rowid);
    if ((ordinal + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  }
  const vertices = new Map<bigint, SqliteRow[]>();
  for (let ordinal = 0; ordinal < vertexRows!.length; ordinal++) {
    const row = vertexRows![ordinal]!;
    const parent = artifactSqliteInteger(row, 1);
    if (!ids.has(parent)) throw new Error("STL vertex has an unknown facet");
    const group = vertices.get(parent) ?? [];
    group.push(row);
    vertices.set(parent, group);
    if ((ordinal + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  }
  const triangles: StlTriangle[] = [];
  for (const facet of artifactSqliteOrderedRows(facets!, 2)) {
    artifactSqliteDocumentReference(facet, 1);
    const owned = vertices.get(facet.rowid) ?? [];
    if (owned.length !== 3) throw new Error("STL facets must own exactly three vertices");
    const ordered = artifactSqliteOrderedRows(owned, 2);
    triangles.push({ normal: coordinates(facet), vertices: [coordinates(ordered[0]!), coordinates(ordered[1]!), coordinates(ordered[2]!)] });
    if (triangles.length % 64 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", triangles.length * 4, total);
  }
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", total, total);
  return { schema: artifactSqliteText(solid, 1), solidName: artifactSqliteText(solid, 2), triangles };
}
/** 🛂️ Validate the exact owned dialect and document at the semantic I/O boundary. */
export async function stlSnapshotValidateSqliteSubset(snapshot:StlSnapshot,dialect:import("../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts").ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<void>{
  await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);
  if(dialect.artifactKind!=="s.stdio.stl"||dialect.standard!=="ascii"||dialect.subset!=="*")throw new Error("geometry owned SQLite dialect differs");
  const table=database.tables.find(table=>table.name.toLowerCase()==="stl_solid");
  const row=table?artifactSqliteDocument(table.rows):undefined;
  if(!row||artifactSqliteText(row,1)!==snapshot.schema)throw new Error("geometry document identity differs from its semantic projection");
}
