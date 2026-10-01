import { parseBinary64, encodeIeee754Cells, readBinary64, ieee754IsNull, type Binary64, type Ieee754Cell, type Ieee754Column } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🗽️ Wavefront OBJ geometry, ordered memberships and face-boundary relations. */
import type { ObjSnapshot } from "../🟦️.ts";
import { artifactSqliteCheckpoint, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteDocumentReference, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, artifactSqliteTextBytes, artifactSqliteValueBudget, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import { sqliteValueByteLength, type SqliteDatabase, type SqliteRow, type SqliteValue } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted SQL kept byte-equal to the adjacent schema asset. */
export const OBJ_SQLITE_SCHEMA = "CREATE TABLE obj_document (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  schema TEXT NOT NULL,\n  material_library TEXT\n);\nCREATE TABLE obj_vertex (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES obj_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  x REAL,\n  y REAL,\n  z REAL,\n  w REAL,\n  x_ieee754_bits INTEGER NOT NULL,\n  x_numeric_class TEXT NOT NULL CHECK(x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  y_ieee754_bits INTEGER NOT NULL,\n  y_numeric_class TEXT NOT NULL CHECK(y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  z_ieee754_bits INTEGER NOT NULL,\n  z_numeric_class TEXT NOT NULL CHECK(z_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  w_ieee754_bits INTEGER,\n  w_numeric_class TEXT CHECK(w_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  CHECK((w_ieee754_bits IS NULL AND w_numeric_class IS NULL AND w IS NULL) OR (w_ieee754_bits IS NOT NULL AND w_numeric_class IS NOT NULL))\n);\nCREATE TABLE obj_texcoord (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES obj_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  u REAL,\n  v REAL,\n  w REAL,\n  u_ieee754_bits INTEGER NOT NULL,\n  u_numeric_class TEXT NOT NULL CHECK(u_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  v_ieee754_bits INTEGER NOT NULL,\n  v_numeric_class TEXT NOT NULL CHECK(v_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  w_ieee754_bits INTEGER,\n  w_numeric_class TEXT CHECK(w_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  CHECK((w_ieee754_bits IS NULL AND w_numeric_class IS NULL AND w IS NULL) OR (w_ieee754_bits IS NOT NULL AND w_numeric_class IS NOT NULL))\n);\nCREATE TABLE obj_normal (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES obj_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  x REAL,\n  y REAL,\n  z REAL,\n  x_ieee754_bits INTEGER NOT NULL,\n  x_numeric_class TEXT NOT NULL CHECK(x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  y_ieee754_bits INTEGER NOT NULL,\n  y_numeric_class TEXT NOT NULL CHECK(y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  z_ieee754_bits INTEGER NOT NULL,\n  z_numeric_class TEXT NOT NULL CHECK(z_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))\n);\nCREATE TABLE obj_face (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES obj_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0)\n);\nCREATE TABLE obj_face_vertex (\n  id INTEGER PRIMARY KEY,\n  face_id INTEGER NOT NULL REFERENCES obj_face(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  vertex_id INTEGER NOT NULL REFERENCES obj_vertex(id),\n  texcoord_id INTEGER REFERENCES obj_texcoord(id),\n  normal_id INTEGER REFERENCES obj_normal(id)\n);\nCREATE TABLE obj_group (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES obj_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  name TEXT NOT NULL\n);\nCREATE TABLE obj_group_face (\n  id INTEGER PRIMARY KEY,\n  group_id INTEGER NOT NULL REFERENCES obj_group(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  face_id INTEGER NOT NULL REFERENCES obj_face(id)\n);\nCREATE TABLE obj_object (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES obj_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  name TEXT NOT NULL\n);\nCREATE TABLE obj_object_face (\n  id INTEGER PRIMARY KEY,\n  object_id INTEGER NOT NULL REFERENCES obj_object(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  face_id INTEGER NOT NULL REFERENCES obj_face(id)\n);\nCREATE TABLE obj_face_boundary (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES obj_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  face_id INTEGER REFERENCES obj_face(id)\n);\nCREATE TABLE obj_material_range (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES obj_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  first_boundary_id INTEGER NOT NULL REFERENCES obj_face_boundary(id),\n  material TEXT NOT NULL\n);\nCREATE TABLE obj_smoothing_range (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES obj_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  first_boundary_id INTEGER NOT NULL REFERENCES obj_face_boundary(id),\n  group_number INTEGER CHECK (group_number BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE obj_unknown_statement (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES obj_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  source_line_ordinal_high INTEGER NOT NULL CHECK (source_line_ordinal_high BETWEEN 0 AND 4294967295),\n  source_line_ordinal_low INTEGER NOT NULL CHECK (source_line_ordinal_low BETWEEN 0 AND 4294967295),\n  raw TEXT NOT NULL\n);\n";
const TABLES = ["obj_document", "obj_vertex", "obj_texcoord", "obj_normal", "obj_face", "obj_face_vertex", "obj_group", "obj_group_face", "obj_object", "obj_object_face", "obj_face_boundary", "obj_material_range", "obj_smoothing_range", "obj_unknown_statement"] as const;
type Table = typeof TABLES[number];
const VERTEX_COLUMNS=[{index:3,width:64},{index:4,width:64},{index:5,width:64},{index:6,width:64}] as const;
const VECTOR_COLUMNS=[{index:3,width:64},{index:4,width:64},{index:5,width:64}] as const;
function floatColumns(table:Table):readonly Ieee754Column[]{return table==="obj_vertex"?VERTEX_COLUMNS:table==="obj_texcoord"||table==="obj_normal"?VECTOR_COLUMNS:[];}
function coordinate(value: Binary64): Binary64 { parseBinary64(value); return value; }
function optionalReal(value: Binary64 | undefined): Binary64 | null { return value === undefined ? null : coordinate(value); }
function integer(value: number, maximum = Number.MAX_SAFE_INTEGER): bigint {
  if (!Number.isSafeInteger(value) || value < 0 || value > maximum) throw new Error("OBJ source index or integer is out of range");
  return BigInt(value);
}
function reference(value: number, size: number): bigint { return integer(value, Math.min(size - 1, 4294967295)) + 1n; }
function optionalReference(value: number | undefined, size: number): bigint | null { return value === undefined ? null : reference(value, size); }
function text(value: string): string { artifactSqliteTextBytes(value); return value; }
type Emit = (name: Table, cells: Ieee754Cell[]) => Promise<void>;
async function visit(snapshot: ObjSnapshot, emit: Emit): Promise<void> {
  await emit("obj_document", [text(snapshot.schema), snapshot.mtllib === undefined ? null : text(snapshot.mtllib)]);
  for (let ordinal = 0; ordinal < snapshot.vertices.length; ordinal++) {
    const vertex = snapshot.vertices[ordinal]!;
    await emit("obj_vertex", [1n, BigInt(ordinal), coordinate(vertex.x), coordinate(vertex.y), coordinate(vertex.z), optionalReal(vertex.w)]);
  }
  for (let ordinal = 0; ordinal < snapshot.texcoords.length; ordinal++) {
    const texcoord = snapshot.texcoords[ordinal]!;
    await emit("obj_texcoord", [1n, BigInt(ordinal), coordinate(texcoord.u), coordinate(texcoord.v), optionalReal(texcoord.w)]);
  }
  for (let ordinal = 0; ordinal < snapshot.normals.length; ordinal++) {
    const normal = snapshot.normals[ordinal]!;
    await emit("obj_normal", [1n, BigInt(ordinal), coordinate(normal.x), coordinate(normal.y), coordinate(normal.z)]);
  }
  for (let ordinal = 0; ordinal < snapshot.faces.length; ordinal++) {
    const face = snapshot.faces[ordinal]!;
    if (face.vertices.length < 3) throw new Error("OBJ faces must have at least three vertices");
    const faceId = BigInt(ordinal + 1);
    await emit("obj_face", [1n, BigInt(ordinal)]);
    for (let ordinal = 0; ordinal < face.vertices.length; ordinal++) {
      const vertex = face.vertices[ordinal]!;
      await emit("obj_face_vertex", [faceId, BigInt(ordinal), reference(vertex.vertex, snapshot.vertices.length), optionalReference(vertex.texcoord, snapshot.texcoords.length), optionalReference(vertex.normal, snapshot.normals.length)]);
    }
  }
  for (let ordinal = 0; ordinal < snapshot.groups.length; ordinal++) {
    const group = snapshot.groups[ordinal]!;
    const groupId = BigInt(ordinal + 1);
    await emit("obj_group", [1n, BigInt(ordinal), text(group.name)]);
    for (let ordinal = 0; ordinal < group.faces.length; ordinal++) await emit("obj_group_face", [groupId, BigInt(ordinal), reference(group.faces[ordinal]!, snapshot.faces.length)]);
  }
  for (let ordinal = 0; ordinal < snapshot.objects.length; ordinal++) {
    const object = snapshot.objects[ordinal]!;
    const objectId = BigInt(ordinal + 1);
    await emit("obj_object", [1n, BigInt(ordinal), text(object.name)]);
    for (let ordinal = 0; ordinal < object.faces.length; ordinal++) await emit("obj_object_face", [objectId, BigInt(ordinal), reference(object.faces[ordinal]!, snapshot.faces.length)]);
  }
  for (let ordinal = 0; ordinal <= snapshot.faces.length; ordinal++) await emit("obj_face_boundary", [1n, BigInt(ordinal), ordinal === snapshot.faces.length ? null : BigInt(ordinal + 1)]);
  for (let ordinal = 0; ordinal < snapshot.usemtl.length; ordinal++) {
    const range = snapshot.usemtl[ordinal]!;
    await emit("obj_material_range", [1n, BigInt(ordinal), integer(range.faceIndexFrom, snapshot.faces.length) + 1n, text(range.material)]);
  }
  for (let ordinal = 0; ordinal < snapshot.smoothingGroups.length; ordinal++) {
    const range = snapshot.smoothingGroups[ordinal]!;
    await emit("obj_smoothing_range", [1n, BigInt(ordinal), integer(range.faceIndexFrom, snapshot.faces.length) + 1n, range.group === undefined ? null : integer(range.group, 4294967295)]);
  }
  for (let ordinal = 0; ordinal < snapshot.unknownStatements.length; ordinal++) {
    const statement = snapshot.unknownStatements[ordinal]!;
    if (typeof statement.lineIndex !== "bigint" || statement.lineIndex < 0n || statement.lineIndex > 18446744073709551615n) throw new Error("OBJ SQLite source line index is out of unsigned64 range");
    await emit("obj_unknown_statement", [1n, BigInt(ordinal), statement.lineIndex >> 32n, statement.lineIndex & 4294967295n, text(statement.raw)]);
  }
}

/** 📤️ Project explicit OBJ entities after a bounded domain preflight, without a native OBJ payload. */
export async function objSnapshotToSqliteDatabase(snapshot: ObjSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, 0);
  let total = 0;
  let bytes = 0;
  await visit(snapshot, async (name, cells) => {
    if (++total > (options.maxRows ?? 1_000_000)) throw new Error("OBJ SQLite row limit");
    bytes += 8;
    for (const value of encodeIeee754Cells([0n,...cells],floatColumns(name),options.maxColumns).slice(1)) { bytes += sqliteValueByteLength(value); artifactSqliteValueBudget(bytes, options); }
    if (total % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, total);
  });
  const rows = new Map<Table, SqliteRow[]>(TABLES.map(name => [name, []]));
  let completed = 0;
  await visit(snapshot, async (name, cells) => {
    const table = rows.get(name)!;
    const id = BigInt(table.length + 1);
    table.push({ rowid: id, values: encodeIeee754Cells([id,...cells],floatColumns(name),options.maxColumns) });
    if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", completed, total);
  });
  const database = artifactSqliteDatabase(OBJ_SQLITE_SCHEMA, TABLES.map(name => rows.get(name)!), options);
  await artifactSqliteCheckpoint(options, "projectSnapshot", total, total);
  return database;
}

function indices(rows: readonly SqliteRow[]): Map<bigint, number> { return new Map(rows.map((row, ordinal) => [row.rowid, ordinal])); }
function resolved(row: SqliteRow, column: number, index: ReadonlyMap<bigint, number>): number {
  const value = index.get(artifactSqliteInteger(row, column));
  if (value === undefined) throw new Error("OBJ relationship references an unknown entity");
  return value;
}
function optionalResolved(row: SqliteRow, column: number, index: ReadonlyMap<bigint, number>): number | undefined { return row.values[column] === null ? undefined : resolved(row, column, index); }
function optionalCoordinate(row: SqliteRow, column: number,columns:readonly Ieee754Column[]): Binary64 | undefined { return ieee754IsNull(row,column,columns)?undefined:readBinary64(row,column,columns); }

/** 📥️ Restore OBJ geometry and ordered memberships through explicit relationships and face boundaries. */
export async function objSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<ObjSnapshot> {
  const total = database.tables.reduce((sum, table) => sum + table.rows.length, 0);
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  const tableRows = await artifactSqliteTables(database, OBJ_SQLITE_SCHEMA, options);
  const rows = new Map<Table, readonly SqliteRow[]>(TABLES.map((name, index) => [name, tableRows[index]!]));
  const document = artifactSqliteDocument(rows.get("obj_document")!);
  let checked = 0;
  let completed = 0;
  const prepare = async (): Promise<void> => { if (++checked % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total); };
  const tick = async (): Promise<void> => { if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", completed, total); };
  const entities = async (name: Table): Promise<SqliteRow[]> => {
    const result = rows.get(name)!;
    for (const row of result) {
      if (row.rowid < 1n) throw new Error("OBJ entities require positive identifiers");
      artifactSqliteDocumentReference(row, 1);
      await prepare();
    }
    return artifactSqliteOrderedRows(result, 2);
  };
  const vertices = await entities("obj_vertex");
  const texcoords = await entities("obj_texcoord");
  const normals = await entities("obj_normal");
  const faces = await entities("obj_face");
  const groups = await entities("obj_group");
  const objects = await entities("obj_object");
  const boundaries = await entities("obj_face_boundary");
  const materials = await entities("obj_material_range");
  const smoothing = await entities("obj_smoothing_range");
  const statements = await entities("obj_unknown_statement");
  const vertexIndices = indices(vertices);
  const texcoordIndices = indices(texcoords);
  const normalIndices = indices(normals);
  const faceIndices = indices(faces);
  const boundaryIndices = indices(boundaries);
  if (boundaries.length !== faces.length + 1) throw new Error("OBJ requires one boundary for every face and its end");
  for (let ordinal = 0; ordinal < boundaries.length; ordinal++) {
    const boundary = boundaries[ordinal]!;
    if (ordinal === faces.length ? boundary.values[3] !== null : resolved(boundary, 3, faceIndices) !== ordinal) throw new Error("OBJ face boundary references the wrong face");
    await prepare();
  }
  const children = async (name: Table, owners: ReadonlyMap<bigint, number>): Promise<Map<bigint, SqliteRow[]>> => {
    const result = new Map<bigint, SqliteRow[]>();
    for (const row of rows.get(name)!) {
      if (row.rowid < 1n) throw new Error("OBJ relationships require positive identifiers");
      const parent = artifactSqliteInteger(row, 1);
      if (!owners.has(parent)) throw new Error("OBJ relationship has an unknown owner");
      const group = result.get(parent) ?? [];
      group.push(row);
      result.set(parent, group);
      await prepare();
    }
    for (const [owner, list] of result) { result.set(owner, artifactSqliteOrderedRows(list, 2)); await prepare(); }
    return result;
  };
  const faceChildren = await children("obj_face_vertex", faceIndices);
  const groupChildren = await children("obj_group_face", indices(groups));
  const objectChildren = await children("obj_object_face", indices(objects));
  const snapshot: ObjSnapshot = { schema: artifactSqliteText(document, 1), mtllib: document.values[2] === null ? undefined : artifactSqliteText(document, 2), vertices: [], texcoords: [], normals: [], faces: [], groups: [], objects: [], usemtl: [], smoothingGroups: [], unknownStatements: [] };
  for (const row of vertices) { snapshot.vertices.push({ x: readBinary64(row,3,VERTEX_COLUMNS), y: readBinary64(row,4,VERTEX_COLUMNS), z: readBinary64(row,5,VERTEX_COLUMNS), w: optionalCoordinate(row,6,VERTEX_COLUMNS) }); await tick(); }
  for (const row of texcoords) { snapshot.texcoords.push({ u: readBinary64(row,3,VECTOR_COLUMNS), v: readBinary64(row,4,VECTOR_COLUMNS), w: optionalCoordinate(row,5,VECTOR_COLUMNS) }); await tick(); }
  for (const row of normals) { snapshot.normals.push({ x: readBinary64(row,3,VECTOR_COLUMNS), y: readBinary64(row,4,VECTOR_COLUMNS), z: readBinary64(row,5,VECTOR_COLUMNS) }); await tick(); }
  for (const row of faces) {
    const owned = faceChildren.get(row.rowid) ?? [];
    if (owned.length < 3) throw new Error("OBJ faces must have at least three vertices");
    const face: ObjSnapshot["faces"][number] = { vertices: [] };
    for (const row of owned) { face.vertices.push({ vertex: resolved(row, 3, vertexIndices), texcoord: optionalResolved(row, 4, texcoordIndices), normal: optionalResolved(row, 5, normalIndices) }); await tick(); }
    snapshot.faces.push(face);
    await tick();
  }
  for (const row of groups) {
    const group = { name: artifactSqliteText(row, 3), faces: [] as number[] };
    for (const member of groupChildren.get(row.rowid) ?? []) { group.faces.push(resolved(member, 3, faceIndices)); await tick(); }
    snapshot.groups.push(group);
    await tick();
  }
  for (const row of objects) {
    const object = { name: artifactSqliteText(row, 3), faces: [] as number[] };
    for (const member of objectChildren.get(row.rowid) ?? []) { object.faces.push(resolved(member, 3, faceIndices)); await tick(); }
    snapshot.objects.push(object);
    await tick();
  }
  for (const row of materials) { snapshot.usemtl.push({ faceIndexFrom: resolved(row, 3, boundaryIndices), material: artifactSqliteText(row, 4) }); await tick(); }
  for (const row of smoothing) {
    const value = row.values[4] === null ? undefined : artifactSqliteInteger(row, 4);
    if (value !== undefined && (value < 0n || value > 4294967295n)) throw new Error("OBJ smoothing group must be an unsigned 32-bit integer");
    snapshot.smoothingGroups.push({ faceIndexFrom: resolved(row, 3, boundaryIndices), group: value === undefined ? undefined : Number(value) });
    await tick();
  }
  for (const row of statements) {
    const high = artifactSqliteInteger(row, 3);
    const low = artifactSqliteInteger(row, 4);
    if (high < 0n || high > 4294967295n || low < 0n || low > 4294967295n) throw new Error("OBJ source line words must be unsigned 32-bit integers");
    snapshot.unknownStatements.push({ lineIndex: (high << 32n) | low, raw: artifactSqliteText(row, 5) });
    await tick();
  }
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", total, total);
  return snapshot;
}
/** 🛂️ Validate the exact owned dialect and document at the semantic I/O boundary. */
export async function objSnapshotValidateSqliteSubset(snapshot:ObjSnapshot,dialect:import("../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts").ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<void>{
  await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);
  if(dialect.artifactKind!=="s.stdio.obj"||dialect.standard!=="3.0"||dialect.subset!=="geometry")throw new Error("geometry owned SQLite dialect differs");
  const table=database.tables.find(table=>table.name.toLowerCase()==="obj_document");
  const row=table?artifactSqliteDocument(table.rows):undefined;
  if(!row||artifactSqliteText(row,1)!==snapshot.schema)throw new Error("geometry document identity differs from its semantic projection");
}
