/** 🧫️ Compares the authored primary-owner payloads through actual Source SQLite producers and Bun's independent reader. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";
import fixture from "../../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json";
import { binarySnapshotToSqliteDatabase, binarySnapshotFromSqliteDatabase } from "../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
import { txtSnapshotToSqliteDatabase, txtSnapshotFromSqliteDatabase } from "../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
import { jsonSnapshotToSqliteDatabase, jsonSnapshotFromSqliteDatabase } from "../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
import type { JsonSnapshot, JsonValue } from "../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";

type LogicalDatabase = Awaited<ReturnType<typeof binarySnapshotToSqliteDatabase>>;

async function assertOwner<S>(index: number, source: S, project: (source: S) => Promise<LogicalDatabase>, reconstruct: (database: LogicalDatabase) => Promise<S>): Promise<void> {
  const witness = fixture.witnesses[index]!;
  const bytes = await exportSqliteDatabase(await project(source));
  const db = Database.deserialize(bytes);
  try {
    expect(db.query("PRAGMA integrity_check").all()).toEqual(fixture.integrity);
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual(fixture.foreignKeys);
    const tables = db.query("SELECT name FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name <> 'semio_snapshot' ORDER BY name").all() as { name: string }[];
    expect(tables.map(row => row.name)).toEqual(Object.keys(witness.domainRows).sort());
    for (const { name } of tables) {
      expect(name).toMatch(/^[a-z_]+$/);
      const rows = witness.domainRows[name as keyof typeof witness.domainRows];
      if (rows === undefined) throw new Error(`unauthored domain table ${name}`);
      expect(db.query(`SELECT * FROM ${name} ORDER BY id`).all()).toEqual(rows);
    }
    expect(source).toEqual(await reconstruct(await importSqliteDatabase(new Uint8Array(db.serialize()))));
    console.log(`[DEBUG] Source primary owner ${witness.kind}/${witness.standard}/${witness.subset}: complete domain tables=${tables.length}, bytes=${bytes.length}, full owner restored`);
  } finally { db.close(); }
}

test("primary Binary Source retains all authored bytes and complete SQLite rows", async () => {
  const source = { schema: "stdio.binary", bytes: [104, 101, 108, 108, 111] };
  expect(Buffer.from(source.bytes).toString("hex")).toBe(fixture.witnesses[0]!.naturalText.split("\n")[1]);
  await assertOwner(0, source, binarySnapshotToSqliteDatabase, binarySnapshotFromSqliteDatabase);
});

test("primary Txt Source retains the authored newline and complete SQLite rows", async () => {
  const source = { schema: "stdio.txt", lines: ["Hello, stdio.txt!"], trailingNewline: true, lineEnding: "lf" as const };
  expect(source.lines.join("\n") + "\n").toBe(fixture.witnesses[1]!.naturalText);
  await assertOwner(1, source, txtSnapshotToSqliteDatabase, txtSnapshotFromSqliteDatabase);
});

const geojson: JsonSnapshot = { schema: "stdio.json", value: { kind: "object", members: [
  { key: "type", value: { kind: "string", value: "Feature" } },
  { key: "geometry", value: { kind: "object", members: [
    { key: "type", value: { kind: "string", value: "Point" } },
    { key: "coordinates", value: { kind: "array", items: [{ kind: "number", lexeme: "7.5" }, { kind: "number", lexeme: "46.25" }] } },
  ] } },
  { key: "properties", value: { kind: "object", members: [
    { key: "name", value: { kind: "string", value: "Grüße" } },
    { key: "visible", value: { kind: "bool", value: true } },
    { key: "nullable", value: { kind: "null" } },
    { key: "tags", value: { kind: "array", items: [{ kind: "string", value: "a" }, { kind: "string", value: "b" }] } },
  ] } },
  { key: "id", value: { kind: "string", value: "site-1" } },
] } };

function logicalJson(value: JsonValue): unknown {
  switch (value.kind) {
    case "null": return null;
    case "number": return Number(value.lexeme);
    case "bool": case "string": return value.value;
    case "array": return value.items.map(logicalJson);
    case "object": return Object.fromEntries(value.members.map(member => [member.key, logicalJson(member.value)]));
  }
}

test("primary GeoJSON Source retains the full Feature owner and complete SQLite rows", async () => {
  const witness = fixture.witnesses[2]!;
  expect(JSON.parse(witness.naturalText)).toEqual(witness.logicalValue);
  expect(logicalJson(geojson.value)).toEqual(witness.logicalValue);
  await assertOwner(2, geojson, jsonSnapshotToSqliteDatabase, jsonSnapshotFromSqliteDatabase);
});

test("primary dependent schema contracts compile independently with exact original leaves", async () => {
  const { default: Ajv } = await import("ajv");
  const dependencies = (await import("../../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧩️dependencies/🔣️.json")).default;
  const ajv = new Ajv({ strict: false, validateFormats: false });
  const read = async (path: string) => JSON.parse(await Bun.file(new URL("../../../../../../" + path, import.meta.url)).text());
  for (const document of dependencies.documents) {
    const schema = await read(document.schemaPath);
    expect(schema.$id).toBe(document.id);
    ajv.addSchema(schema);
  }
  for (const contract of dependencies.contracts) {
    expect(typeof ajv.compile(await read(contract.schemaPath))).toBe("function");
    console.log("[DEBUG] independent structural contract " + contract.scope + ": exact original dependencies resolved");
  }
});

test("primary dependent declarations explicitly own every published structural schema export", async () => {
  const dependencies = (await import("../../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧩️dependencies/🔣️.json")).default;
  for (const document of dependencies.documents) for (const owner of document.definitions) {
    const definition = JSON.parse(await Bun.file(new URL("../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/" + owner.artifact + "/📜️artifact-definition.json", import.meta.url)).text());
    const claim = owner.scope + "#" + owner.export;
    expect(definition.runtime_capabilities.filter((row: {category:string,claims:{namespace:string,value:string}[]}) => row.category === "schema" && row.claims.some(value => value.namespace === "schema-export" && value.value === claim))).toHaveLength(1);
    console.log("[DEBUG] declared structural dependency " + definition.id + ": " + claim);
  }
});

