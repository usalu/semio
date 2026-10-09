/** 🧾️ Native publication identity fields retain schema text and original page ownership. */
import { test, expect } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

test("prepared publication identity fields are native paged text owners", () => {
  const root = new URL("../", import.meta.url);
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, root), "utf8"));
  const fixture = read("🧫️fixtures/🔣️.json");
  expect(new Ajv2020({ strict: true }).compile(read("🧬️schema/🔣️.json"))(fixture)).toBe(true);
  const actor = fixture.actor.repeat(fixture.repeat);
  expect(Buffer.byteLength(actor)).toBeGreaterThan(fixture.copyCapacityLimit);
  const db = new Database(":memory:");
  db.exec("CREATE TABLE identities(actor TEXT,applied TEXT,tail TEXT)");
  db.query("INSERT INTO identities VALUES(?,?,?)").run(actor, fixture.id, fixture.id);
  expect(db.query("SELECT actor,applied,tail FROM identities").get()).toEqual({ actor, applied: fixture.id, tail: fixture.id });
  expect(JSON.parse(JSON.stringify({ actor, applied: fixture.id, tail: fixture.id }))).toEqual({ actor, applied: fixture.id, tail: fixture.id });
  db.close();
  console.log("[DEBUG] independent JSON UTF8 and SQLite exact original actor/applied/tail semantics retain giant actor beyond4096 with Unicode and NUL");
  const store = readFileSync(new URL("../../🦀️.rs", root), "utf8");
  const start = store.indexOf("pub struct ArtifactStoreOneItemPrepared<P, Mutation>");
  const fields = store.slice(start, store.indexOf("impl<P, Mutation>", start));
  expect(fields).toContain("local_actor: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>");
  expect(fields).toContain("applied_edit_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>");
  expect(fields).toContain("tail_edit_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>");
});
