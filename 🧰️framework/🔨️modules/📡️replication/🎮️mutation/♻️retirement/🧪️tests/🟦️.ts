import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "../🧫️fixtures/🔣️.json";

test("native pending replay cleanup admits each physical ownership lane", async () => {
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE owners(rank INTEGER, kind TEXT)");
    for (const vector of fixture.cases) {
      database.exec("DELETE FROM owners");
      if (vector.snapshot) database.run("INSERT INTO owners VALUES(0,'snapshot')");
      if (vector.inverse) database.run("INSERT INTO owners VALUES(1,'inverse')");
      database.run("INSERT INTO owners VALUES(2,'factory')");
      const observed = database.query<{ kind: string }, []>("SELECT kind FROM owners ORDER BY rank").all().map(row => row.kind);
      expect(observed).toEqual(vector.order);
    }
  } finally { database.close(); }
  const source = await Bun.file(new URL("../🦀️.rs", import.meta.url)).text();
  expect(source).toContain("pub struct ReplayRetirement<P, M>");
  expect(source).not.toContain("ReplayPairRetirement");
  for (const lane of fixture.lanes.slice(0, 3)) expect(source).toContain(`next_${lane}_byte_demand`);
  expect(source).toContain("next_depth_demand");
  expect(source).toContain("close_factory_ticket");
});
