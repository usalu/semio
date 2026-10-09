import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
test("original protocol ticket keeps physical backing separate from its box shell", () => {
  const db = new Database(":memory:");
  db.run("CREATE TABLE backing (bytes BLOB NOT NULL)");
  for (const row of fixture.cases) {
    expect(Buffer.byteLength(row.text, "utf8")).toBeLessThanOrEqual(row.capacity);
    db.run("DELETE FROM backing");
    db.run("INSERT INTO backing VALUES (zeroblob(?))", [row.capacity]);
    expect(db.query("SELECT length(bytes) AS capacity FROM backing").get()).toEqual({capacity: row.capacity});
    console.log(`[DEBUG] protocol ticket case=${row.id} SQLite physical=${row.capacity} UTF8=${Buffer.byteLength(row.text)} original-body=${fixture.maximumBodyBytes}`);
  }
  db.close();
  const source = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  expect(source.includes("factory_ticket_demands<T:ErasedSnapshotRetirement+?Sized>")).toBe(true);
  expect(source.includes("close_factory_ticket<T:ErasedSnapshotRetirement+?Sized>")).toBe(true);
  expect(fixture.capacityAccounting).toBe("actual-admitted-cursor-birth");
  expect(fixture.physicalConservation).toBe("source+cursor-birth+terminal-box");
  expect(fixture.constructorBirthBytes).toBe(0);
  expect(fixture.refusalBirthBytes).toBe(0);
  expect(fixture.terminalDropBytes).toBe(0);
  const native = readFileSync(new URL("../🦀️.rs", import.meta.url), "utf8");
  const law = native.slice(native.indexOf("fn original_protocol_ticket_retires_actual"));
  expect(law).toContain("born += heap.0");
  expect(law).toContain("backing + shell + born");
  expect(law).toContain("maximum_capacity_bytes: demand.capacity_bytes - 1");
  expect(law).toContain("original.original().unwrap().as_ptr(), source_pointer");
});
