import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import law from "./🔣️.json";

test("both Note surfaces admit their native archive issuer with its exact source quote", () => {
  const read = (path: string) => readFileSync(new URL(path, import.meta.url), "utf8");
  const store = read("../🦀️.rs");
  expect(store).toContain("type NoteSemanticProducer = store::ColdMutationApplyFactory<Snapshot, Mutation>");
  expect(store).toContain("bounded_artifact_store_owners_shared_semantic_source_demands::<Snapshot, Mutation, NoteSemanticProducer>(PRODUCER_CHILDREN, 0)");
  expect(store).toContain("bounded_artifact_store_owners_with_shared_semantic_preparation(grant, PRODUCER_CHILDREN, 0, NoteSemanticProducer::default)");

  for (const surface of ["✏️editor", "👁️viewer"]) {
    const implementation = read(`../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/${surface}/🦀️.rs`);
    expect(implementation).toMatch(/fn document_store_owners_source_demands\([^\n]*\{\s*semio_s_artifact_note_note_mutations::host::owned::note_store_owners_source_demands\(\)/);
    expect(implementation).toMatch(/fn build_document_store_owners\([^\n]*\{\s*Some\(semio_s_artifact_note_note_mutations::host::owned::admit_note_store_owners\(grant\)\)/);
  }
});

test("the archive issuer keeps both ports on one source and denies a short source grant", () => {
  const db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE source (id INTEGER PRIMARY KEY); CREATE TABLE port (name TEXT PRIMARY KEY, source_id INTEGER NOT NULL REFERENCES source(id));");
    const admit = db.query("INSERT INTO source SELECT 1 WHERE ? AND ? AND ? AND ?");
    const attach = db.query("INSERT INTO port SELECT ?, id FROM source");
    const currencies = ["items", "copy", "capacity", "depth"];
    for (const currency of law.deniedCurrencies) {
      expect(admit.run(...currencies.map((current) => Number(current !== currency))).changes).toBe(0);
    }
    expect(admit.run(1, 1, 1, 1).changes).toBe(1);
    for (const port of law.ports) expect(attach.run(port).changes).toBe(1);
    expect(db.query("SELECT COUNT(DISTINCT source_id) AS producers FROM port").get()).toEqual({ producers: law.distinctProducers });
    const retire = db.transaction(() => {
      db.exec("DELETE FROM port; DELETE FROM source;");
      expect(db.query("SELECT COUNT(*) AS producers FROM source").get()).toEqual({ producers: 0 });
    });
    retire();
  } finally {
    db.close();
  }
});
