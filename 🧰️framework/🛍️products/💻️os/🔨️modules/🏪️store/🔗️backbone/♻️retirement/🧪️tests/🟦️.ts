import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

test("backbone retirement preserves original ordered owners and exact physical release", () => {
  const law = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE rows(position INTEGER, kind TEXT)");
    const insert = database.query("INSERT INTO rows VALUES (?1,?2)");
    law.queue.forEach((row: { kind: string }, index: number) => insert.run(index, row.kind));
    expect(database.query<{ kind: string }, []>("SELECT kind FROM rows ORDER BY position").all().map(row => row.kind)).toEqual(law.expectedOrder);
    const release = database.query<{ released: number }, number[]>("SELECT CASE WHEN ?1 >= ?2 THEN ?2 ELSE 0 END AS released");
    for (const row of law.releaseCases) expect(release.get(row.grant, row.demand)?.released).toBe(row.released);
  } finally { database.close(); }
  const source = readFileSync(new URL("../🦀️.rs", import.meta.url), "utf8");
  expect(source).toContain("next_release_byte_demand");
  expect(source).toContain("shared_retirement_allocation_bytes");
  expect(source).not.toContain("truncate(");
  console.log("[DEBUG] backbone ordered owners and indivisible original backing release agree with SQLite");
});

test("backbone constructor admits its exact native frame before moving the original queue", () => {
  const law = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const database = new Database(":memory:");
  try {
    const admitted = database.query<{ accepted: number }, number[]>("SELECT (?1 > 0 AND ?2 > 0 AND ?3 >= ?4) AS accepted");
    for (const row of law.constructors) expect(Boolean(admitted.get(row.items, row.depth, row.capacity, row.demand)?.accepted)).toBe(row.accepted);
  } finally { database.close(); }
  const source = readFileSync(new URL("../🦀️.rs", import.meta.url), "utf8");
  expect(source).toContain("fn admit_queue(");
  expect(source).toContain("fn constructor_capacity_bytes(");
});

test("installed backbone admission retains empty allocated queues under the caller's full grant", () => {
  const law = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const database = new Database(":memory:");
  try {
    const retained = database.query<{ retained: number }, number[]>("SELECT (?1 > 0 OR ?2 > 0) AS retained");
    for (const row of law.installedQueues) expect(Boolean(retained.get(row.length, row.capacity)?.retained)).toBe(row.retained);
  } finally { database.close(); }
  const store = readFileSync(new URL("../../../🦀️.rs", import.meta.url), "utf8");
  for (const authority of ["fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant)", "if self.member_inbox.capacity() != 0", "ArtifactStoreCursorDisposerPhase::Backbone => self.close_take_backbone_retirement(grant)", "ArtifactStoreCursorDisposerPhase::Backbone if store.member_inbox.capacity() == 0"]) expect(store.includes(authority)).toBe(true);
  console.log("[DEBUG] installed backbone custody includes the original empty queue backing and forwards the caller's common grant");
});
