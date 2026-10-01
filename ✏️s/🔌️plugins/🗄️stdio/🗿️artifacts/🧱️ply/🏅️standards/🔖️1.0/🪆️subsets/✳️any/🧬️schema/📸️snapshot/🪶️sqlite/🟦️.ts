import { parseBinary64, parseBinary32, encodeIeee754Cells, readBinary64, readBinary32, ieee754IsNull, type Binary64, type Binary32, type Ieee754Cell, type Ieee754Column } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🧱️ PLY typed element declarations, row cells and ordered list items. */
import type { PlySnapshot, PlyProperty, PlyScalarType, PlyValue } from "../🟦️.ts";
import { artifactSqliteCheckpoint, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteDocumentReference, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, artifactSqliteTextBytes, artifactSqliteValueBudget, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import { sqliteValueByteLength, type SqliteDatabase, type SqliteRow, type SqliteValue } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted schema byte-equal to the adjacent SQL asset. */
export const PLY_SQLITE_SCHEMA = "CREATE TABLE ply_document (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  schema TEXT NOT NULL,\n  format TEXT NOT NULL CHECK (format IN ('ascii', 'binary_little_endian', 'binary_big_endian'))\n);\nCREATE TABLE ply_comment (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES ply_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  content TEXT NOT NULL\n);\nCREATE TABLE ply_element (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES ply_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  name TEXT NOT NULL,\n  declared_count_high INTEGER NOT NULL CHECK (declared_count_high BETWEEN 0 AND 4294967295),\n  declared_count_low INTEGER NOT NULL CHECK (declared_count_low BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE ply_property (\n  id INTEGER PRIMARY KEY,\n  element_id INTEGER NOT NULL REFERENCES ply_element(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  name TEXT NOT NULL,\n  form TEXT NOT NULL CHECK (form IN ('scalar', 'list')),\n  scalar_kind TEXT CHECK (scalar_kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double')),\n  count_kind TEXT CHECK (count_kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double')),\n  value_kind TEXT CHECK (value_kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double')),\n  CHECK ((form = 'scalar' AND scalar_kind IS NOT NULL AND count_kind IS NULL AND value_kind IS NULL) OR (form = 'list' AND scalar_kind IS NULL AND count_kind IS NOT NULL AND value_kind IS NOT NULL))\n);\nCREATE TABLE ply_row (\n  id INTEGER PRIMARY KEY,\n  element_id INTEGER NOT NULL REFERENCES ply_element(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0)\n);\nCREATE TABLE ply_cell (\n  id INTEGER PRIMARY KEY,\n  row_id INTEGER NOT NULL REFERENCES ply_row(id),\n  property_id INTEGER NOT NULL REFERENCES ply_property(id),\n  kind TEXT NOT NULL CHECK (kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double', 'list')),\n  integer_value INTEGER,\n  real_value REAL,\n  real_value_ieee754_bits INTEGER,\n  real_value_numeric_class TEXT CHECK(real_value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  CHECK ((kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint') AND integer_value IS NOT NULL AND real_value IS NULL AND real_value_ieee754_bits IS NULL AND real_value_numeric_class IS NULL) OR (kind IN ('float', 'double') AND integer_value IS NULL AND real_value_ieee754_bits IS NOT NULL AND real_value_numeric_class IS NOT NULL AND ((real_value_numeric_class='nan' AND real_value IS NULL) OR (real_value_numeric_class!='nan' AND real_value IS NOT NULL))) OR (kind = 'list' AND integer_value IS NULL AND real_value IS NULL AND real_value_ieee754_bits IS NULL AND real_value_numeric_class IS NULL)),\n  CHECK(kind!='float' OR real_value_ieee754_bits BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE ply_list_item (\n  id INTEGER PRIMARY KEY,\n  cell_id INTEGER NOT NULL REFERENCES ply_cell(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  kind TEXT NOT NULL CHECK (kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double')),\n  integer_value INTEGER,\n  real_value REAL,\n  real_value_ieee754_bits INTEGER,\n  real_value_numeric_class TEXT CHECK(real_value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  CHECK ((kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint') AND integer_value IS NOT NULL AND real_value IS NULL AND real_value_ieee754_bits IS NULL AND real_value_numeric_class IS NULL) OR (kind IN ('float', 'double') AND integer_value IS NULL AND real_value_ieee754_bits IS NOT NULL AND real_value_numeric_class IS NOT NULL AND ((real_value_numeric_class='nan' AND real_value IS NULL) OR (real_value_numeric_class!='nan' AND real_value IS NOT NULL)))),\n  CHECK(kind!='float' OR real_value_ieee754_bits BETWEEN 0 AND 4294967295)\n);\n";
const TABLES = ["ply_document", "ply_comment", "ply_element", "ply_property", "ply_row", "ply_cell", "ply_list_item"] as const;
type Table = typeof TABLES[number];
const KINDS = ["char", "uChar", "short", "uShort", "int", "uInt", "float", "double"] as const;
const RANGES: Partial<Record<PlyScalarType, readonly [number, number]>> = { char: [-128,127], uChar: [0,255], short: [-32768,32767], uShort: [0,65535], int: [-2147483648,2147483647], uInt: [0,4294967295] };
const FORMATS = { ascii: "ascii", binaryLittleEndian: "binary_little_endian", binaryBigEndian: "binary_big_endian" } as const;
function text(value: string): string { artifactSqliteTextBytes(value); return value; }
function kind(value: string): PlyScalarType {
  const found = KINDS.find(kind => kind.toLowerCase() === value);
  if (!found) throw new Error("PLY scalar kind is unknown");
  return found;
}

const BINARY32=[{index:5,width:32}] as const;
const BINARY64=[{index:5,width:64}] as const;
function floatColumns(table:Table,cells:readonly Ieee754Cell[]):readonly Ieee754Column[]{return table==="ply_cell"||table==="ply_list_item"?(cells[2]==="float"?BINARY32:BINARY64):[];}
function scalar(type: PlyScalarType, value: number | Binary64 | Binary32): readonly [bigint | null, Binary64 | Binary32 | null] {
  if (!KINDS.includes(type)) throw new Error("PLY scalar kind must be typed");
  const range = RANGES[type];
  if (range) {
    if (typeof value !== "number" || !Number.isInteger(value) || value < range[0] || value > range[1]) throw new Error("PLY integer scalar is outside its declared width");
    return [BigInt(value), null];
  }
  return [null,type==="float"?parseBinary32(value):parseBinary64(value)];
}
function declaration(property: PlyProperty): SqliteValue[] {
  text(property.name);
  if (property.form === "scalar" && KINDS.includes(property.kind)) return [property.name, "scalar", property.kind.toLowerCase(), null, null];
  if (property.form !== "list" || !KINDS.includes(property.valueKind)) throw new Error("PLY property declaration is invalid");
  if (!KINDS.includes(property.countKind)) throw new Error("PLY list count kind is invalid");
  return [property.name, "list", null, property.countKind.toLowerCase(), property.valueKind.toLowerCase()];
}
type Emit = (table: Table, cells: Ieee754Cell[]) => Promise<bigint>;
async function visit(snapshot: PlySnapshot, emit: Emit): Promise<void> {
  const format = FORMATS[snapshot.format];
  if (!format) throw new Error("PLY format is invalid");
  await emit("ply_document", [text(snapshot.schema), format]);
  for (let ordinal = 0; ordinal < snapshot.comments.length; ordinal++) await emit("ply_comment", [1n, BigInt(ordinal), text(snapshot.comments[ordinal]!)]);
  for (let ordinal = 0; ordinal < snapshot.elements.length; ordinal++) {
    const element = snapshot.elements[ordinal]!;
    if (typeof element.count !== "bigint" || element.count < 0n || element.count > 0xffffffffffffffffn) throw new Error("PLY declared count must be an unsigned64 bigint");
    const elementId = await emit("ply_element", [1n, BigInt(ordinal), text(element.name), element.count >> 32n, element.count & 0xffffffffn]);
    const propertyIds: bigint[] = [];
    for (let ordinal = 0; ordinal < element.properties.length; ordinal++) propertyIds.push(await emit("ply_property", [elementId, BigInt(ordinal), ...declaration(element.properties[ordinal]!) ]));
    for (let ordinal = 0; ordinal < element.rows.length; ordinal++) {
      const row = element.rows[ordinal]!;
      if (row.values.length !== element.properties.length) throw new Error("PLY row must contain one value per property");
      const rowId = await emit("ply_row", [elementId, BigInt(ordinal)]);
      for (let index = 0; index < element.properties.length; index++) {
        const property = element.properties[index]!;
        const value = row.values[index]!;
        if (property.form === "scalar") {
          if (value.kind !== property.kind) throw new Error("PLY scalar value kind differs from its property");
          await emit("ply_cell", [rowId, propertyIds[index]!, property.kind.toLowerCase(), ...scalar(property.kind, value.value as number | Binary64 | Binary32)]);
        } else {
          if (value.kind !== "list" || !Array.isArray(value.value)) throw new Error("PLY list length exceeds its count kind");
          const cellId = await emit("ply_cell", [rowId, propertyIds[index]!, "list", null, null]);
          for (let ordinal = 0; ordinal < value.value.length; ordinal++) {
            const item = value.value[ordinal]!;
            if (item.kind !== property.valueKind) throw new Error("PLY list item kind differs from its property");
            await emit("ply_list_item", [cellId, BigInt(ordinal), property.valueKind.toLowerCase(), ...scalar(property.valueKind, item.value as number | Binary64 | Binary32)]);
          }
        }
      }
    }
  }
}

/** 📤️ Project PLY declarations and scalar cells after cancellable aggregate preflight. */
export async function plySnapshotToSqliteDatabase(snapshot: PlySnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, 0);
  const counts = new Map<Table, number>(TABLES.map(name => [name, 0]));
  let total = 0;
  let bytes = 0;
  await visit(snapshot, async (name, cells) => {
    if (++total > (options.maxRows ?? 1_000_000)) throw new Error("PLY SQLite row limit");
    bytes += 8;
    for (const value of encodeIeee754Cells([0n,...cells],floatColumns(name,cells),options.maxColumns).slice(1)) { bytes += sqliteValueByteLength(value); artifactSqliteValueBudget(bytes, options); }
    const count = counts.get(name)! + 1;
    counts.set(name, count);
    if (total % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, total);
    return BigInt(count);
  });
  const rows = new Map<Table, SqliteRow[]>(TABLES.map(name => [name, []]));
  let completed = 0;
  await visit(snapshot, async (name, cells) => {
    const table = rows.get(name)!;
    const id = BigInt(table.length + 1);
    table.push({ rowid: id, values: encodeIeee754Cells([id,...cells],floatColumns(name,cells),options.maxColumns) });
    if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", completed, total);
    return id;
  });
  const database = artifactSqliteDatabase(PLY_SQLITE_SCHEMA, TABLES.map(name => rows.get(name)!), options);
  await artifactSqliteCheckpoint(options, "projectSnapshot", total, total);
  return database;
}
function primitive(row: SqliteRow): Exclude<PlyValue, { kind: "list" }> {
  const type = kind(artifactSqliteText(row, 3));
  const integer = RANGES[type] !== undefined;
  if (row.values[integer ? 5 : 4] !== null) throw new Error("PLY scalar has conflicting payloads");
  if (type==="float") return {kind:type,value:readBinary32(row,5,BINARY32)};
  if (type==="double") return {kind:type,value:readBinary64(row,5,BINARY64)};
  if (!ieee754IsNull(row,5,BINARY64)) throw new Error("PLY integer scalar has conflicting IEEE payload");
  const value=Number(artifactSqliteInteger(row,4));scalar(type,value);return {kind:type,value};
}

/** 📥️ Reconstruct PLY declarations through checked element, row, property and list ownership. */
export async function plySnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<PlySnapshot> {
  const total = database.tables.reduce((sum, table) => sum + table.rows.length, 0);
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  const [documents, comments, elements, properties, rows, cells, items] = await artifactSqliteTables(database, PLY_SQLITE_SCHEMA, options);
  const document = artifactSqliteDocument(documents!);
  const formatText = artifactSqliteText(document, 2);
  const format = (Object.keys(FORMATS) as PlySnapshot["format"][]).find(key => FORMATS[key] === formatText);
  if (!format) throw new Error("PLY format is invalid");
  let prepared = 0;
  const prepare = async (): Promise<void> => { if (++prepared % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total); };
  for (const table of [comments!, elements!, properties!, rows!, cells!, items!]) for (const row of table) {
    if (row.rowid < 1n) throw new Error("PLY entity identifiers must be positive");
    await prepare();
  }
  for (const row of [...comments!, ...elements!]) { artifactSqliteDocumentReference(row, 1); await prepare(); }
  const own = async (children: readonly SqliteRow[], parents: readonly SqliteRow[], ordered: boolean): Promise<Map<bigint, SqliteRow[]>> => {
    const identities = new Set(parents.map(row => row.rowid));
    const groups = new Map<bigint, SqliteRow[]>();
    for (const row of children) {
      const owner = artifactSqliteInteger(row, 1);
      if (!identities.has(owner)) throw new Error("PLY relationship has an unknown owner");
      const list = groups.get(owner) ?? [];
      list.push(row); groups.set(owner, list);
      await prepare();
    }
    if (ordered) for (const [owner, list] of groups) { groups.set(owner, artifactSqliteOrderedRows(list, 2)); await prepare(); }
    return groups;
  };
  const propertyGroups = await own(properties!, elements!, true);
  const rowGroups = await own(rows!, elements!, true);
  const cellGroups = await own(cells!, rows!, false);
  const itemGroups = await own(items!, cells!, true);
  let completed = 1;
  const tick = async (): Promise<void> => { if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", completed, total); };
  const snapshot: PlySnapshot = { schema: artifactSqliteText(document, 1), format, comments: [], elements: [] };
  for (const row of artifactSqliteOrderedRows(comments!, 2)) { snapshot.comments.push(artifactSqliteText(row, 3)); await tick(); }
  for (const element of artifactSqliteOrderedRows(elements!, 2)) {
    const ownedProperties = propertyGroups.get(element.rowid) ?? [];
    const ownedRows = rowGroups.get(element.rowid) ?? [];
    const high = artifactSqliteInteger(element,4), low = artifactSqliteInteger(element,5);
    if(high<0n||high>0xffffffffn||low<0n||low>0xffffffffn)throw new Error("PLY declared count word exceeds unsigned32");
    const parsedProperties: PlyProperty[] = [];
    for (const row of ownedProperties) {
      const name = artifactSqliteText(row, 3);
      const form = artifactSqliteText(row, 4);
      if (form === "scalar") {
        if (row.values[6] !== null || row.values[7] !== null) throw new Error("PLY scalar property has list kinds");
        parsedProperties.push({ form, name, kind: kind(artifactSqliteText(row, 5)) });
      } else if (form === "list") {
        if (row.values[5] !== null) throw new Error("PLY list property has a scalar kind");
        const countKind = kind(artifactSqliteText(row, 6));
        if(!KINDS.includes(countKind))throw new Error("PLY count kind is invalid");
        parsedProperties.push({ form, name, countKind, valueKind: kind(artifactSqliteText(row, 7)) });
      } else throw new Error("PLY property form is invalid");
      await tick();
    }
    const result: PlySnapshot["elements"][number] = { name: artifactSqliteText(element, 3), count: (high << 32n) | low, properties: parsedProperties, rows: [] };
    const propertyIndices = new Map(ownedProperties.map((row, index) => [row.rowid, index]));
    for (const row of ownedRows) {
      const ownedCells = cellGroups.get(row.rowid) ?? [];
      const values = new Array<PlyValue>(parsedProperties.length);
      for (const cell of ownedCells) {
        const index = propertyIndices.get(artifactSqliteInteger(cell, 2));
        if (index === undefined || values[index] !== undefined) throw new Error("PLY cell property must belong uniquely to its row element");
        const property = parsedProperties[index]!;
        const ownedItems = itemGroups.get(cell.rowid) ?? [];
        if (property.form === "scalar") {
          if (ownedItems.length !== 0) throw new Error("PLY scalar cells cannot own list items");
          const value = primitive(cell);
          if (value.kind !== property.kind) throw new Error("PLY cell kind differs from its scalar declaration");
          values[index] = value;
        } else {
          if (artifactSqliteText(cell, 3) !== "list" || cell.values[4] !== null || !ieee754IsNull(cell,5,BINARY64)) throw new Error("PLY list cell shape or count is invalid");
          const value: PlyValue = { kind: "list", value: [] };
          for (const item of ownedItems) {
            const scalar = primitive(item);
            if (scalar.kind !== property.valueKind) throw new Error("PLY list item kind differs from its declaration");
            value.value.push(scalar); await tick();
          }
          values[index] = value;
        }
        await tick();
      }
      if (ownedCells.length !== values.length) throw new Error("PLY row must contain one cell per property");
      result.rows.push({ values }); await tick();
    }
    snapshot.elements.push(result); await tick();
  }
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", total, total);
  return snapshot;
}
/** 🛂️ Validate the exact owned dialect and document at the semantic I/O boundary. */
export async function plySnapshotValidateSqliteSubset(snapshot:PlySnapshot,dialect:import("../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts").ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<void>{
  await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);
  if(dialect.artifactKind!=="s.stdio.ply"||dialect.standard!=="1.0"||dialect.subset!=="*")throw new Error("geometry owned SQLite dialect differs");
  const table=database.tables.find(table=>table.name.toLowerCase()==="ply_document");
  const row=table?artifactSqliteDocument(table.rows):undefined;
  if(!row||artifactSqliteText(row,1)!==snapshot.schema)throw new Error("geometry document identity differs from its semantic projection");
}
