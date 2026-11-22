import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import law from "./natural-source-limits.json";

test("paged natural source output covers the large fixture that the ordinary import refuses", () => {
  const db = new Database(":memory:");
  db.exec("CREATE TABLE limits (name TEXT PRIMARY KEY, bytes INTEGER)");
  const add = db.query("INSERT INTO limits VALUES (?, ?)");
  for (const [name, bytes] of Object.entries(law)) add.run(name, bytes);
  const read = (name: string) => db.query<{ bytes: number }, [string]>("SELECT bytes FROM limits WHERE name = ?").get(name)!.bytes;
  expect(read("sourceBytes")).toBeLessThanOrEqual(read("selectedMaximumBytes"));
  expect(read("sourceBytes")).toBeGreaterThan(read("ordinaryOutputBytes"));
  expect(read("imageBytes")).toBeLessThanOrEqual(read("naturalOutputBytes"));
  expect(read("naturalOutputBytes")).toBe(read("selectedMaximumBytes") * read("outputPerSourceByte"));
  db.close();
});
