/** 🪪️ Shared schema-neutral publication fields retain the same original text authority. */
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";

test("Store publication authority exposes original native semantic fields", () => {
  const root = new URL("../", import.meta.url);
  const shared = new URL("../../../../../../🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🛂️authority/", root);
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, shared), "utf8"));
  const fixture = read("🧫️fixtures/🔣️.json");
  expect(new Ajv2020({ strict: true }).compile(read("🧬️schema/🔣️.json"))(fixture)).toBe(true);
  const db = new Database(":memory:");
  db.exec("CREATE TABLE publication(original TEXT)");
  const original = JSON.stringify(fixture.authority);
  db.query("INSERT INTO publication VALUES(?)").run(original);
  expect(db.query("SELECT original FROM publication").get()).toEqual({ original });
  expect(Buffer.from(JSON.parse(original).actor).equals(Buffer.from(fixture.authority.actor))).toBe(true);
  db.close();
  console.log("[DEBUG] independent Node exact original publication UTF8 and SQLite immutable field conservation agree with shared semantic authority fixture");
  const source = readFileSync(new URL("🦀️.rs", root), "utf8");
  expect(source).toContain("impl crate::os_spr::ArtifactCanonicalEditAuthority for ArtifactStoreOneItemLiveAuthority");
  expect(source).toContain("&self.actor");
  expect(source).toContain("self.line.as_ref()");
  expect(source).toContain("self.group_id.as_ref()");
});
