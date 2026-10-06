import {artifactSqliteValueControl} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {jsonNumberMeaning} from "../../../🧬️schema/📸️snapshot/🔢️number/🟦️.ts";
/** 🧾️ RFC8259 JSON value/member/element relations mirror the adjacent handcrafted schema. */
import type { JsonSnapshot, JsonValue } from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import { artifactSqliteBoolean, artifactSqliteCheckpoint, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteInteger, artifactSqliteReal, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, artifactSqliteTextBytes, artifactSqliteValueBudget, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase, SqliteRow } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted SQL kept byte-equal to the adjacent schema asset. */
export const JSON_SQLITE_SCHEMA = "CREATE TABLE json_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL, root_value_id INTEGER NOT NULL REFERENCES json_value(id));\nCREATE TABLE json_value (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK(kind IN ('null', 'boolean', 'number', 'string', 'array', 'object')), boolean_value INTEGER CHECK(boolean_value IN (0, 1)), number_lexeme TEXT, string_value TEXT, number_value REAL, CHECK((kind IN ('null', 'array', 'object') AND number_value IS NULL AND boolean_value IS NULL AND number_lexeme IS NULL AND string_value IS NULL) OR (kind = 'boolean' AND number_value IS NULL AND boolean_value IS NOT NULL AND number_lexeme IS NULL AND string_value IS NULL) OR (kind = 'number' AND boolean_value IS NULL AND number_lexeme IS NOT NULL AND string_value IS NULL) OR (kind = 'string' AND number_value IS NULL AND boolean_value IS NULL AND number_lexeme IS NULL AND string_value IS NOT NULL)));\nCREATE TABLE json_object_member (id INTEGER PRIMARY KEY, object_value_id INTEGER NOT NULL REFERENCES json_value(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), key TEXT NOT NULL, value_id INTEGER NOT NULL REFERENCES json_value(id));\nCREATE TABLE json_array_element (id INTEGER PRIMARY KEY, array_value_id INTEGER NOT NULL REFERENCES json_value(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), value_id INTEGER NOT NULL REFERENCES json_value(id));\n";

function kind(value: JsonValue): string { return value.kind === "bool" ? "boolean" : value.kind; }
function rowLimit(rows: number, options: ArtifactSqliteOptions): void {
  if (!Number.isSafeInteger(rows) || rows > (options.maxRows ?? 1_000_000)) throw new Error("JSON SQLite row limit");
}
async function measure(snapshot: JsonSnapshot, options: ArtifactSqliteOptions): Promise<number> {
  let rows = 1;
  let bytes = artifactSqliteTextBytes(snapshot.schema) + 16;
  rowLimit(rows, options);
  artifactSqliteValueBudget(bytes, options);
  const stack: { value: JsonValue; exit: boolean }[] = [{ value: snapshot.value, exit: false }];
  const active = new Set<JsonValue>();
  let visited = 0;
  while (stack.length) {
    const { value, exit } = stack.pop()!;
    if (exit) { active.delete(value); continue; }
    if (active.has(value)) throw new Error("JSON snapshot contains a cycle");
    active.add(value);
    stack.push({ value, exit: true });
    rows++;
    bytes += 8 + artifactSqliteTextBytes(kind(value));
    switch (value.kind) {
      case "null": break;
      case "bool": if (typeof value.value !== "boolean") throw new Error("JSON boolean value is invalid"); bytes += 8; break;
      case "number": bytes += artifactSqliteTextBytes(value.lexeme)+8; artifactSqliteValueBudget(bytes, options); break;
      case "string": bytes += artifactSqliteTextBytes(value.value); break;
      case "array":
        rowLimit(rows + 2 * value.items.length, options);
        rows += value.items.length;
        bytes += 32 * value.items.length;
        for (let index = value.items.length - 1; index >= 0; index--) stack.push({ value: value.items[index]!, exit: false });
        break;
      case "object":
        rowLimit(rows + 2 * value.members.length, options);
        rows += value.members.length;
        bytes += 32 * value.members.length;
        for (let index = value.members.length - 1; index >= 0; index--) {
          const member = value.members[index]!;
          bytes += artifactSqliteTextBytes(member.key);
          artifactSqliteValueBudget(bytes, options);
          stack.push({ value: member.value, exit: false });
          if (index % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, rows);
        }
        break;
      default: throw new Error("JSON value kind is invalid");
    }
    rowLimit(rows, options);
    artifactSqliteValueBudget(bytes, options);
    if (++visited % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, rows);
  }
  return rows;
}

interface Parent { value: JsonValue; parent?: bigint; ordinal?: number; key?: string }

/** 📤️ Project exact JSON primitives and explicit object/array ownership without native serialization. */
export async function jsonSnapshotToSqliteDatabase(snapshot: JsonSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, 0);
  const total = await measure(snapshot, options);
  const values: SqliteRow[] = [];
  const members: SqliteRow[] = [];
  const elements: SqliteRow[] = [];
  const stack: Parent[] = [{ value: snapshot.value }];
  while (stack.length) {
    const { value, parent, ordinal, key } = stack.pop()!;
    const id = BigInt(values.length + 1);
    values.push({ rowid: id, values: [id, kind(value), value.kind === "bool" ? BigInt(Number(value.value)) : null, value.kind === "number" ? value.lexeme : null, value.kind === "string" ? value.value : null, value.kind === "number" ? (await jsonNumberMeaning(value.lexeme,artifactSqliteValueControl(options,"projectSnapshot",1+values.length+members.length+elements.length,total))).numeric : null] });
    if (parent !== undefined) {
      const rows = key === undefined ? elements : members;
      const linkId = BigInt(rows.length + 1);
      rows.push({ rowid: linkId, values: key === undefined ? [linkId, parent, BigInt(ordinal!), id] : [linkId, parent, BigInt(ordinal!), key, id] });
    }
    if (value.kind === "array") for (let index = value.items.length - 1; index >= 0; index--) stack.push({ value: value.items[index]!, parent: id, ordinal: index });
    if (value.kind === "object") for (let index = value.members.length - 1; index >= 0; index--) {
      const member = value.members[index]!;
      stack.push({ value: member.value, parent: id, ordinal: index, key: member.key });
    }
    if (values.length % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 1 + values.length + members.length + elements.length, total);
  }
  const database = await artifactSqliteDatabase(JSON_SQLITE_SCHEMA, [[{ rowid: 1n, values: [1n, snapshot.schema, 1n] }], values, members, elements], options);
  await artifactSqliteCheckpoint(options, "projectSnapshot", total, total);
  return database;
}

/** 📥️ Reconstruct ordered JSON ownership, rejecting cycles, aliases and disconnected values. */
export async function jsonSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<JsonSnapshot> {
  const total = database.tables.reduce((sum, table) => sum + table.rows.length, 0);
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  const [documents, valueRows, memberRows, elementRows] = await artifactSqliteTables(database, JSON_SQLITE_SCHEMA, options);
  const document = artifactSqliteDocument(documents!);
  const root = artifactSqliteInteger(document, 2);
  const values = new Map<bigint, SqliteRow>();
  let checked = 0;
  for (const row of valueRows!) {
    const tag = artifactSqliteText(row, 1);
    const empty = (index: number): boolean => row.values[index] === null;
    switch (tag) {
      case "null": case "array": case "object": if (!empty(2) || !empty(3) || !empty(4) || !empty(5)) throw new Error("JSON primitive payload mismatch"); break;
      case "boolean": artifactSqliteBoolean(row, 2); if (!empty(3) || !empty(4) || !empty(5)) throw new Error("JSON primitive payload mismatch"); break;
      case "number": {const expected=(await jsonNumberMeaning(artifactSqliteText(row,3),artifactSqliteValueControl(options,"reconstructSnapshot",0,total))).numeric;if(!empty(2)||!empty(4)||(expected===null?!empty(5):empty(5)||artifactSqliteReal(row,5)!==expected))throw new Error("JSON derived numeric value disagrees with its owned lexeme");break;}
      case "string": artifactSqliteText(row, 4); if (!empty(2) || !empty(3) || !empty(5)) throw new Error("JSON primitive payload mismatch"); break;
      default: throw new Error("JSON primitive kind is invalid");
    }
    values.set(row.rowid, row);
    if (++checked % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  }
  if (!values.has(root)) throw new Error("JSON document root is dangling");
  const owners = new Set<bigint>([root]);
  const links = new Map<bigint, SqliteRow[]>();
  for (const [rows, tag, childColumn] of [[memberRows!, "object", 4], [elementRows!, "array", 3]] as const) {
    for (const row of rows) {
      const parent = artifactSqliteInteger(row, 1);
      const child = artifactSqliteInteger(row, childColumn);
      if (!values.has(child) || !values.has(parent) || artifactSqliteText(values.get(parent)!, 1) !== tag) throw new Error("JSON relationship parent or child is invalid");
      if (owners.has(child)) throw new Error("JSON value has multiple owners or a cycle");
      owners.add(child);
      if (tag === "object") artifactSqliteText(row, 3);
      const group = links.get(parent) ?? [];
      group.push(row);
      links.set(parent, group);
      if (++checked % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
    }
  }
  if (owners.size !== values.size) throw new Error("JSON value has no owner");
  for (const [parent, rows] of links) {
    links.set(parent, artifactSqliteOrderedRows(rows, 2));
    if (++checked % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  }
  const visited = new Set<bigint>();
  const reconstructed = new Map<bigint, JsonValue>();
  const stack = [{ id: root, finish: false }];
  let completed = 1;
  while (stack.length) {
    const { id, finish } = stack.pop()!;
    const row = values.get(id)!;
    const tag = artifactSqliteText(row, 1);
    const children = links.get(id) ?? [];
    if (!finish) {
      if (visited.has(id)) throw new Error("JSON relationships contain a cycle");
      visited.add(id);
      stack.push({ id, finish: true });
      for (let index = children.length - 1; index >= 0; index--) stack.push({ id: artifactSqliteInteger(children[index]!, tag === "object" ? 4 : 3), finish: false });
      if (visited.size % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", completed, total);
      continue;
    }
    let value: JsonValue;
    switch (tag) {
      case "null": value = { kind: "null" }; break;
      case "boolean": value = { kind: "bool", value: artifactSqliteBoolean(row, 2) }; break;
      case "number": value = { kind: "number", lexeme: artifactSqliteText(row, 3) }; break;
      case "string": value = { kind: "string", value: artifactSqliteText(row, 4) }; break;
      case "array": value = { kind: "array", items: [] }; break;
      case "object": value = { kind: "object", members: [] }; break;
      default: throw new Error("JSON value kind is invalid");
    }
    for (const child of children) {
      const childId = artifactSqliteInteger(child, tag === "object" ? 4 : 3);
      const childValue = reconstructed.get(childId);
      if (!childValue) throw new Error("JSON child was not reconstructed");
      reconstructed.delete(childId);
      if (value.kind === "array") value.items.push(childValue);
      else if (value.kind === "object") value.members.push({ key: artifactSqliteText(child, 3), value: childValue });
      if (++checked % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", completed, total);
    }
    reconstructed.set(id, value);
    links.delete(id);
    completed += 1 + children.length;
    if (completed % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", completed, total);
  }
  if (visited.size !== values.size || links.size) throw new Error("JSON relationships are disconnected or cyclic");
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", total, total);
  return { schema: artifactSqliteText(document, 1), value: reconstructed.get(root)! };
}
