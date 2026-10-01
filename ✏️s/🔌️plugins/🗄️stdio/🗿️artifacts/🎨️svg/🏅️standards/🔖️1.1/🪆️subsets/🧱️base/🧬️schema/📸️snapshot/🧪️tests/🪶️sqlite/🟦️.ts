/** 🧫️ SVG's shared typed XML corpus retains independent SVG table names. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { parseSvgSnapshot, type SvgSnapshot } from "../../🟦️.ts";
import { SVG_SQLITE_SCHEMA, svgSnapshotToSqliteDatabase, svgSnapshotFromSqliteDatabase } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";
const input: SvgSnapshot = { schema: fixture.schema, doc: {
  declaration: { version: "1.0", encoding: "UTF-8", quote: "single" },
  prolog: [{ kind: "comment", text: "before" }], epilog: [{ kind: "comment", text: "after" }],
  doctype: { prologPosition: 1, name: "svg", externalId: { kind: "system", systemId: "svg.dtd" }, declarations: [{ kind: "entity", parameter: false, name: "title", value: "Grüße 🌠" }] },
  root: { kind: "element", name: "svg", attrs: [{ name: "xmlns", value: "http://www.w3.org/2000/svg" }, { name: "viewBox", value: "0 0 100 100" }], children: [
    { kind: "element", name: "g", attrs: [{ name: "transform", value: "translate(10 20) rotate(30)" }], children: [
      { kind: "element", name: "path", attrs: [{ name: "id", value: "shape" }, { name: "d", value: "M0 0L10 20Z" }], children: [] },
      { kind: "element", name: "text", attrs: [], children: [{ kind: "text", text: "Grüße 🌠" }] },
      { kind: "element", name: "style", attrs: [], children: [{ kind: "cData", text: "path { fill: red; }" }] },
      { kind: "comment", text: "kept" }, { kind: "processingInstruction", target: "render", data: "ready" },
    ] },
  ] },
} };

test("SVG handcrafted thirteen tables preserve typed metadata, nodes and independently edited path attributes", async () => {
  expect(SVG_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  expect(parseSvgSnapshot(input)).toEqual(input);
  const database = await svgSnapshotToSqliteDatabase(input);
  expect(await svgSnapshotFromSqliteDatabase(database)).toEqual(input);
  expect(database.tables.every(table => table.name.startsWith("svg_"))).toBe(true);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query(fixture.query).get()).toEqual({ value: "M0 0L10 20Z" });
    expect(db.query("SELECT encoding,quote FROM svg_declaration").get()).toEqual({ encoding: "UTF-8", quote: "single" });
    expect(db.query("SELECT name,value FROM svg_entity").get()).toEqual({ name: "title", value: "Grüße 🌠" });
    db.query("UPDATE svg_attribute SET value=? WHERE name='d'").run(fixture.editedPath);
    const edited = await svgSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    const root = edited.doc.root!;
    if (root.kind !== "element" || root.children[0]?.kind !== "element" || root.children[0].children[0]?.kind !== "element") throw new Error("SVG path fixture tree");
    expect(root.children[0].children[0].attrs[1]!.value).toBe(fixture.editedPath);
    expect(edited.doc.declaration).toEqual(input.doc.declaration);
    expect(edited.doc.doctype).toEqual(input.doc.doctype);
  } finally { db.close(); }
});

test("SVG independent root, component, ownership, boundary and metadata edits reject", async () => {
  for (const edit of [
    "UPDATE svg_element SET name='html' WHERE node_id=1",
    "DELETE FROM svg_element WHERE node_id=1",
    "UPDATE svg_child SET child_node_id=1 WHERE id=1",
    "UPDATE svg_attribute SET ordinal=99 WHERE id=1",
    "UPDATE svg_doctype SET prolog_position=99",
    "UPDATE svg_entity SET doctype_id=999",
    "UPDATE svg_declaration SET encoding='UTF-16'",
  ]) {
    const db = Database.deserialize(await exportSqliteDatabase(await svgSnapshotToSqliteDatabase(input)));
    try { db.run(edit); await expect(svgSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); }
    finally { db.close(); }
  }
});

test("SVG namespaced roots, missing roots and bounds obey typed SVG semantics", async () => {
  const value: SvgSnapshot = { schema: "namespaced.svg", doc: { prolog: [], epilog: [], root: { kind: "element", name: "s:svg", attrs: [], children: [] } } };
  expect(await svgSnapshotFromSqliteDatabase(await svgSnapshotToSqliteDatabase(value))).toEqual(value);
  for (const root of [undefined,{ kind: "element" as const, name: "html", attrs: [], children: [] },{ kind: "text" as const, text: "svg" }]) await expect(svgSnapshotToSqliteDatabase({ ...value, doc: { ...value.doc, root } })).rejects.toThrow("svg root");
  const database = await svgSnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 },{ maxValueBytes: 0 }]) {
    await expect(svgSnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(svgSnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
});

test("SVG projection and reconstruction carry cancellation through typed XML helpers", async () => {
  for (const direction of ["project","reconstruct"]) {
    const database = await svgSnapshotToSqliteDatabase(input);
    const controller = new AbortController();
    const options = { signal: controller.signal, onProgress: () => controller.abort() };
    await expect(direction === "project" ? svgSnapshotToSqliteDatabase(input, options) : svgSnapshotFromSqliteDatabase(database, options)).rejects.toMatchObject({ name: "AbortError" });
  }
});

