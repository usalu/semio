/** 🧫️ Shared XML semantic corpus with independent typed SQL queries and edits. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import type { XmlSnapshot, XmlNode } from "../../🟦️.ts";
import { xmlSnapshotToSqliteDatabase, xmlSnapshotFromSqliteDatabase, XML_SQLITE_SCHEMA } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const input: XmlSnapshot = { schema: fixture.schema, doc: {
  declaration: { version: "1.0", encoding: "UTF-8", standalone: true, quote: "single" },
  doctype: { prologPosition: 2, name: "catalog", externalId: { kind: "public", publicId: "-//Semio//DTD Example 1.0//EN", systemId: "catalog.dtd" }, declarations: [{ kind: "entity", parameter: false, name: "greeting", value: "Grüße 🌠" }, { kind: "entity", parameter: true, name: "parameter", value: "literal" }] },
  root: { kind: "element", name: "catalog", attrs: [{ name: "z", value: "last" }, { name: "a", value: "first" }], children: [
    { kind: "element", name: "item", attrs: [{ name: "code", value: "A" }], children: [{ kind: "text", text: "text & more" }, { kind: "cData", text: "<literal>" }, { kind: "comment", text: "inside" }, { kind: "processingInstruction", target: "render", data: "ready" }] },
    { kind: "element", name: "empty", attrs: [], children: [] },
  ] },
  prolog: [{ kind: "comment", text: "before" }, { kind: "processingInstruction", target: "build", data: "begin" }],
  epilog: [{ kind: "processingInstruction", target: "build", data: "end" }, { kind: "comment", text: "after" }],
} };

test("XML handwritten components, attributes and declarations expose shared independent SQL", async () => {
  expect(XML_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const database = await xmlSnapshotToSqliteDatabase(input);
  expect(await xmlSnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query(fixture.query).all()).toEqual([{ name: "z", value: "last" }, { name: "a", value: "first" }]);
    expect(db.query("SELECT DISTINCT kind FROM xml_node ORDER BY kind").all()).toEqual([...fixture.nodeKinds].sort().map(kind => ({ kind })));
    expect(db.query("SELECT name,parameter FROM xml_entity ORDER BY ordinal").all()).toEqual([{ name: fixture.entityNames[0], parameter: 0 }, { name: fixture.entityNames[1], parameter: 1 }]);
    expect(db.query("SELECT external_kind,prolog_position FROM xml_doctype").get()).toEqual({ external_kind: "public", prolog_position: 2 });
    db.query("UPDATE xml_attribute SET value=? WHERE element_node_id=1 AND ordinal=0").run(fixture.editedAttribute);
    db.run("UPDATE xml_declaration SET standalone=0");
    const edited = await xmlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect((edited.doc.root as Extract<XmlNode, { kind: "element" }>).attrs[0]!.value).toBe(fixture.editedAttribute);
    expect(edited.doc.declaration!.standalone).toBe(false);
  } finally { db.close(); }
});

test("XML SQL-edited missing components, ownership cycles, ordinals and metadata reject", async () => {
  for (const edit of [
    "DELETE FROM xml_element WHERE node_id=1",
    "UPDATE xml_child SET child_node_id=1 WHERE id=1",
    "UPDATE xml_attribute SET ordinal=12 WHERE id=1",
    "UPDATE xml_doctype SET prolog_position=100",
    "UPDATE xml_node SET kind='text' WHERE id=1",
    "UPDATE xml_entity SET doctype_id=999 WHERE id=1",
    "INSERT INTO xml_node VALUES(99,'element'); INSERT INTO xml_element VALUES(99,'island'); INSERT INTO xml_child VALUES(99,99,0,99)",
    "UPDATE xml_declaration SET encoding='UTF-16'",
  ]) {
    const db = Database.deserialize(await exportSqliteDatabase(await xmlSnapshotToSqliteDatabase(input)));
    try { db.run(edit); await expect(xmlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); }
    finally { db.close(); }
  }
});

test("XML optional metadata and empty document preserve typed semantics", async () => {
  for (const externalId of [undefined, { kind: "system" as const, systemId: "Grüße.dtd" }, { kind: "public" as const, publicId: "public identifier", systemId: "system.dtd" }]) {
    for (const standalone of [undefined, false, true]) {
      const value: XmlSnapshot = { ...input, doc: { ...input.doc, doctype: { ...input.doc.doctype!, externalId }, declaration: { version: "1.0", standalone, quote: "double" } } };
      expect(await xmlSnapshotFromSqliteDatabase(await xmlSnapshotToSqliteDatabase(value))).toEqual(value);
    }
  }
  const empty: XmlSnapshot = { schema: "empty.xml", doc: { prolog: [], epilog: [] } };
  const db = Database.deserialize(await exportSqliteDatabase(await xmlSnapshotToSqliteDatabase(empty)));
  try { expect(await xmlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(empty); }
  finally { db.close(); }
});

test("XML projection and reconstruction obey aggregate bounds and cancellation", async () => {
  const database = await xmlSnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 }, { maxValueBytes: 0 }]) {
    await expect(xmlSnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(xmlSnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
  const controller = new AbortController();
  const many: XmlSnapshot = { schema: "large.xml", doc: { root: { kind: "element", name: "root", attrs: Array.from({ length: 1000 }, (_, index) => ({ name: "attribute" + index, value: "text" })), children: [] }, prolog: [], epilog: [] } };
  let events = 0;
  await expect(xmlSnapshotToSqliteDatabase(many, { signal: controller.signal, onProgress: () => { if (++events === 2) controller.abort(); } })).rejects.toMatchObject({ name: "AbortError" });
  const reconstruct = new AbortController();
  await expect(xmlSnapshotFromSqliteDatabase(database, { signal: reconstruct.signal, onProgress: () => reconstruct.abort() })).rejects.toMatchObject({ name: "AbortError" });
  let root: XmlNode = { kind: "text", text: "leaf" };
  for (let depth = 0; depth < 2000; depth++) root = { kind: "element", name: "n", attrs: [], children: [root] };
  const deep = await xmlSnapshotFromSqliteDatabase(await xmlSnapshotToSqliteDatabase({ schema: "deep.xml", doc: { root, prolog: [], epilog: [] } }));
  let node = deep.doc.root!;
  let depth = 0;
  while (node.kind === "element") { depth++; node = node.children[0]!; }
  expect(depth).toBe(2000);
});

