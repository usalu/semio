/** 📦️ SQLite independently groups the original edit rows into the fixture's bounded release sequence. */
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

test("mutation pages preserve the edit's order and bound each release", () => {
  const law = JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const db = new Database(":memory:");
  db.exec("CREATE TABLE edit(position INTEGER PRIMARY KEY, bytes INTEGER)");
  const insert = db.query("INSERT INTO edit VALUES (?, ?)");
  for (let i = 0; i < law.items; i++) insert.run(i, law.itemBytes);
  const pages = db.query("SELECT count(*) AS slots, sum(bytes) AS released FROM edit GROUP BY position / ? ORDER BY min(position)").all(law.pageItems) as { slots: number; released: number }[];
  expect(db.query("SELECT position FROM edit ORDER BY position").values().flat()).toEqual(Array.from({ length: law.items }, (_, i) => i));
  expect(pages.map(page => page.slots)).toEqual(law.pageSlots);
  expect(pages.map(page => page.released).reverse()).toEqual(law.pageReleaseBytes);
  expect(db.query("SELECT sum(bytes) AS total FROM edit").get()).toEqual({ total: law.pageReleaseBytes.reduce((sum: number, release: number) => sum + release, 0) });
  expect(Math.max(...pages.map(page => page.released))).toBe(law.maximumReleaseBytes);
  db.close();
});
