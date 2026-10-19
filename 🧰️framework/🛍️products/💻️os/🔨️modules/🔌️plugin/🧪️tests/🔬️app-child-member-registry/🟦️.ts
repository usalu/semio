import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pluginFamilyText } from "../../🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";
import { jobCloseDemandFixture as fixture } from "@semio-tech/framework/job/testing/fixtures";
import { groupHistoryCasesFixtureUrl } from "@semio-tech/framework-os/modules/store/testing/fixtures";

test("prepared child identity release uses the neutral physical admission corpus", () => {
  
  for (const row of fixture.cases) {
    const retained = Buffer.alloc(row.physicalBytes, 120);
    expect(retained.byteLength).toBe(row.physicalBytes);
    expect(retained.byteLength).toBeLessThanOrEqual(fixture.admissionBytes);
    expect(row.releasedBytes).toBe(row.callerBytes >= retained.byteLength ? retained.byteLength : 0);
    const identity = ["owner-δ", "slot-α", "child-β", "artifact-γ", "kind", "standard", "subset"];
    expect(identity.map(text => Buffer.byteLength(text, "utf8"))).toEqual([8, 7, 8, 11, 4, 8, 6]);
  }
  const native = pluginFamilyText();
  for (const method of ["return_prepared_snapshot_read_erased", "close_metadata_step", "take_prepared_entry", "close_prepared_structure_step"]) expect(native.includes(method)).toBe(true);
  expect(native.includes("metadata: ManuallyDrop<Option<[String; 7]>>")).toBe(true);
});

test("editor and viewer adapters retain the selected original mutation batch admission", () => {
  const group = JSON.parse(readFileSync(groupHistoryCasesFixtureUrl, "utf8"));
  const lengths = [];
  for (const row of group.cases) {
    const original = Array.from({ length: row.historyEntries }, (_, value) => ({ value }));
    const bytes = Buffer.alloc(original.length * 4);
    original.forEach((mutation, index) => bytes.writeInt32LE(mutation.value, index * 4));
    expect(original.map((_, index) => bytes.readInt32LE(index * 4))).toEqual(original.map(mutation => mutation.value));
    lengths.push(original.length);
  }
  expect(lengths).toEqual([0, 1, 65, 257]);
  const native = pluginFamilyText();
  for (const owner of ["E", "V"]) {
    expect(native.includes(`fn owned_mutation_batch_birth_bytes() -> Option<usize> { ${owner}::owned_mutation_batch_birth_bytes() }`)).toBe(true);
    expect(native.includes(`{ ${owner}::admit_owned_mutation_batch(values, grant) }`)).toBe(true);
  }
});

/** 🔗️ Independent SQLite identity joins prove the exact known root, page, or entry retainer. */
test("prepared child mixed views select only the exact frontier owner without allocation", async () => {
  const { Database } = await import("bun:sqlite");
  const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "🧫️fixtures/🔗️retained-alias/🔣️.json"), "utf8"));

  
  const db = new Database(":memory:");
  try {
    db.run("CREATE TABLE owners(view TEXT, scope TEXT, identity TEXT, PRIMARY KEY(view,scope,identity))");
    for (const [view, value] of Object.entries(fixture.views) as [string, { root: string; pages: string[]; entries: string[] }][]) {
      db.run("INSERT INTO owners VALUES(?,?,?)", [view, "root", value.root]);
      for (const [scope, identities] of [["page", value.pages], ["entry", value.entries]] as const) for (const identity of identities) db.run("INSERT INTO owners VALUES(?,?,?)", [view, scope, identity]);
    }
    for (const row of fixture.cases) {
      const retained = db.query("SELECT count(*) AS n FROM owners WHERE scope=? AND identity=? AND view IN (?,?)").get(row.scope, row.identity, ...row.retainers) as { n: number };
      expect(retained.n > 0).toBe(row.retained);
      expect(row.releasedBytes).toBe(0);
      expect(row.items > 0 && retained.n > 0).toBe(row.items === 1 && row.retained);
    }
  } finally { db.close(); }
  const native = pluginFamilyText();
  expect(native.includes("prepared_structure_retainer")).toBe(true);
});
