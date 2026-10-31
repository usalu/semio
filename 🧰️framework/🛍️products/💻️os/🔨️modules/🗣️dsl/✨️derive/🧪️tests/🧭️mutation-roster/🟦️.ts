/** 🧭️ The mutations derive's roster laws, checked from data: bounded-tail hashing of owner identities and the 14-field leaf records of the energy aggregate. Oracles: `Set`/`Map` over full identities and fast-glob. */
import { expect, test } from "bun:test";
import fg from "fast-glob";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🧭️mutation-roster/🔣️.json", import.meta.url), "utf8"));
const repository = resolve(import.meta.dir, "../../../../../../../..");
const encoder = new TextEncoder();

const owners = (row: any): string[] => {
  const tail = "t".repeat(row.sharedTail);
  const list = Array.from({ length: row.count }, (_, index) => `${row.prefix}${index}/${tail}`);
  if (row.duplicate) list[row.duplicate.index] = list[row.duplicate.of];
  return list;
};
const tailKey = (text: string): string => Buffer.from(encoder.encode(text).slice(-fixture.tailBytes)).toString("hex");
const firstDuplicate = (list: string[]) => {
  const seen = new Map<string, number>();
  for (const [index, owner] of list.entries()) {
    const first = seen.get(owner);
    if (first !== undefined) return { first, index };
    seen.set(owner, index);
  }
  return null;
};

test("owner identities hash a bounded tail yet compare in full", () => {
  for (const row of fixture.uniqueness) {
    const list = owners(row);
    expect(new Set(list.map(tailKey)).size, row.name).toBe(row.expectedBuckets);
    expect(firstDuplicate(list), row.name).toEqual(row.expected);
  }
});

test("the energy aggregate publishes one unique 14-field record per leaf", async () => {
  const law = fixture.records, base = join(repository, law.root);
  const files = await fg("**/🔣️.json", { cwd: base, onlyFiles: true, followSymbolicLinks: false });
  const rows = files.map(path => ({ path, record: JSON.parse(readFileSync(join(base, path), "utf8")) })).filter(row => "semanticKind" in row.record);
  expect(rows.length).toBe(law.leaves);
  for (const { path, record } of rows) {
    const keys = Object.keys(record).filter(key => key !== law.optionalExtra);
    expect(keys.length, path).toBe(law.fields);
    expect([...keys].sort(), path).toEqual([...law.requiredFields].sort());
    expect(resolve(repository, record.owner), path).toBe(resolve(base, path, ".."));
  }
  for (const field of law.uniqueFields) expect(new Set(rows.map(row => row.record[field])).size, field).toBe(law.leaves);
});
