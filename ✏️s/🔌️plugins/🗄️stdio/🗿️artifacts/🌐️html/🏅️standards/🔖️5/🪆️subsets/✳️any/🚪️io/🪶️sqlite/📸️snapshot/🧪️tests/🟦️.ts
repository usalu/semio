/** 🧫️ HTML native tree corpus with raw text and boolean versus empty attributes. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import { parseHtmlSnapshot, type HtmlSnapshot, type HtmlNode } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { HTML_SQLITE_SCHEMA, htmlSnapshotToSqliteDatabase, htmlSnapshotFromSqliteDatabase } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const input: HtmlSnapshot = { schema: fixture.schema, doctype: "html", root: { kind: "element", name: "html", attributes: [{ name: "lang", value: "en" }], children: [
  { kind: "element", name: "head", attributes: [], children: [
    { kind: "element", name: "style", attributes: [], children: [{ kind: "rawText", parentKind: "style", text: ".a > .b { color: red; }" }] },
    { kind: "element", name: "script", attributes: [], children: [{ kind: "rawText", parentKind: "script", text: "if (a < b) { x = '&raw'; }" }] },
  ] },
  { kind: "element", name: "body", attributes: [], children: [
    { kind: "element", name: "input", attributes: [{ name: fixture.booleanAttribute },{ name: fixture.emptyAttribute, value: "" }], children: [] },
    { kind: "element", name: "p", attributes: [{ name: "z", value: "last" },{ name: "a", value: "first" }], children: [{ kind: "text", text: "Grüße 🌠 & more" },{ kind: "comment", text: "kept" }] },
  ] },
] } };
test("HTML typed raw text and optional attributes expose independent semantic SQL", async () => {
  expect(HTML_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
  expect(parseHtmlSnapshot(input)).toEqual(input);
  const database = await htmlSnapshotToSqliteDatabase(input);
  expect(await htmlSnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    const attrs = db.query(fixture.query).all();
    expect(attrs).toContainEqual({ name: fixture.booleanAttribute, value: null, element: "input" });
    expect(attrs).toContainEqual({ name: fixture.emptyAttribute, value: "", element: "input" });
    expect(db.query("SELECT parent_kind FROM html_raw_text ORDER BY node_id").all()).toEqual(fixture.rawTextKinds.map(parent_kind => ({ parent_kind })));
    db.query("UPDATE html_attribute SET value=? WHERE name='value'").run(fixture.editedValue);
    db.run("UPDATE html_raw_text SET text='x < y && z > 0' WHERE parent_kind='script'");
    const edited = await htmlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    const projected = await htmlSnapshotToSqliteDatabase(edited);
    const oracle = Database.deserialize(await exportSqliteDatabase(projected));
    try {
      expect(oracle.query("SELECT value FROM html_attribute WHERE name='value'").get()).toEqual({ value: fixture.editedValue });
      expect(oracle.query("SELECT text FROM html_raw_text WHERE parent_kind='script'").get()).toEqual({ text: "x < y && z > 0" });
    } finally { oracle.close(); }
  } finally { db.close(); }
});

test("HTML independently edited missing components, ownership and ordinals reject", async () => {
  for (const edit of [
    "DELETE FROM html_element WHERE node_id=1",
    "UPDATE html_child SET child_node_id=1 WHERE id=1",
    "UPDATE html_child SET parent_element_node_id=999 WHERE id=1",
    "UPDATE html_attribute SET ordinal=99 WHERE id=1",
    "UPDATE html_attribute SET element_node_id=999 WHERE id=1",
    "UPDATE html_raw_text SET parent_kind='textarea'",
    "INSERT INTO html_node VALUES(99,'element'); INSERT INTO html_element VALUES(99,'island'); INSERT INTO html_child VALUES(99,99,0,99)",
  ]) {
    const db = Database.deserialize(await exportSqliteDatabase(await htmlSnapshotToSqliteDatabase(input)));
    try { db.run("PRAGMA ignore_check_constraints=ON"); db.run(edit); await expect(htmlSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); }
    finally { db.close(); }
  }
});

test("HTML scalar roots, undefined attributes, cycles and deep trees preserve native domain boundaries", async () => {
  for (const root of [{ kind: "text" as const, text: "text" },{ kind: "comment" as const, text: "comment" },{ kind: "rawText" as const, parentKind: "script" as const, text: "a < b" }]) {
    const value: HtmlSnapshot = { schema: "scalar.html", root };
    expect(await htmlSnapshotFromSqliteDatabase(await htmlSnapshotToSqliteDatabase(value))).toEqual(value);
  }
  const cycle: Extract<HtmlNode,{kind:"element"}> = { kind: "element", name: "cycle", attributes: [], children: [] };
  cycle.children.push(cycle);
  await expect(htmlSnapshotToSqliteDatabase({ schema: "cycle.html", root: cycle })).rejects.toThrow("cycle");
  let root: HtmlNode = { kind: "text", text: "leaf" };
  for (let depth = 0; depth < 2500; depth++) root = { kind: "element", name: "div", attributes: [], children: [root] };
  const restored = await htmlSnapshotFromSqliteDatabase(await htmlSnapshotToSqliteDatabase({ schema: "deep.html", root }));
  let node = restored.root, depth = 0;
  while (node.kind === "element") { depth++; node = node.children[0]!; }
  expect(depth).toBe(2500);
});

test("HTML bounds and cancellation apply to both typed semantic directions", async () => {
  const database = await htmlSnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 },{ maxValueBytes: 0 }]) {
    await expect(htmlSnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(htmlSnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
  const value: HtmlSnapshot = { schema: "large.html", root: { kind: "element", name: "div", attributes: Array.from({ length: 1000 }, (_, index) => ({ name: "a" + index, value: "text" })), children: [] } };
  const controller = new AbortController();let events = 0;
  await expect(htmlSnapshotToSqliteDatabase(value, { signal: controller.signal, onProgress: () => { if (++events === 2) controller.abort(); } })).rejects.toMatchObject({ name: "ValueError", kind: "canceled" });
  const restore = new AbortController();
  await expect(htmlSnapshotFromSqliteDatabase(database, { signal: restore.signal, onProgress: () => restore.abort() })).rejects.toMatchObject({ name: "ValueError", kind: "canceled" });
});


import "./🚦️cohort/🟦️.ts";
import "./🧹️lifecycle/🟦️.ts";
