import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import law from "./metadata-retirement.json";

test("reserved commit retains its view through ingress close and retires the last owner", () => {
  const db = new Database(":memory:");
  db.exec("CREATE TABLE owner (name TEXT PRIMARY KEY, view TEXT)");
  const view = JSON.stringify({ window: law.window, utility: law.utility });
  const add = db.query("INSERT INTO owner VALUES (?, ?)");
  const remove = db.query("DELETE FROM owner WHERE name = ?");
  const count = db.query<{ count: number }, []>("SELECT COUNT(*) AS count FROM owner");
  add.run(law.phases[0]!, view);
  add.run(law.phases[1]!, view);
  const before = count.get()!.count;
  remove.run(law.phases[0]!);
  const retained = count.get()!.count;
  expect(db.query("SELECT view FROM owner").get()).toEqual({ view });
  remove.run(law.phases[1]!);
  expect([before, retained, count.get()!.count]).toEqual(law.expectedOwners);
  db.close();
});
