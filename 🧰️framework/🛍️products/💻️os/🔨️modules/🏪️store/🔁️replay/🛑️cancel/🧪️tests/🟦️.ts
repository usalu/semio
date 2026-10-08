import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

test("raw replay cancellation preserves original custody until separately admitted physical release", () => {
  const law = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE originals(position INTEGER PRIMARY KEY, owner TEXT)");
    const insert = database.query("INSERT INTO originals VALUES (?1,?2)");
    law.originals.forEach((owner: string, position: number) => insert.run(position, owner));
    expect(database.query<{ count: number }, []>("SELECT COUNT(*) AS count FROM originals").get()?.count).toBe(law.cancellation.retained);
    const released = database.query<{ bytes: number }, number[]>("SELECT CASE WHEN ?1>0 AND ?2>0 AND ?3>=?4 THEN ?4 ELSE 0 END AS bytes");
    for (const turn of law.release) expect(released.get(turn.items, turn.depth, turn.grant, turn.demand)?.bytes).toBe(turn.released);
    const finish = database.query<{ accepted: number; retained: number }, number[]>("SELECT CASE WHEN ?1=1 AND ?2=0 THEN 1 ELSE 0 END AS accepted, COUNT(*) AS retained FROM originals");
    for (const turn of law.finish) {
      const actual = finish.get(Number(turn.ready), Number(turn.cancelled));
      expect(Boolean(actual?.accepted)).toBe(turn.accepted);
      expect(actual?.retained).toBe(turn.originalsRetained);
    }
  } finally { database.close(); }
  const source = readFileSync(new URL("../../../🦀️.rs", import.meta.url), "utf8");
  const producer = readFileSync(new URL("../🦀️.rs", import.meta.url), "utf8");
  const drop = producer.slice(producer.indexOf("impl<P, M: Mutation<P>> Drop for EditReplay<P, M>"));
  expect(drop.length > 0).toBe(true);
  expect(drop.includes("close_cold")).toBe(false);
  expect(drop.includes("usize::MAX")).toBe(false);
  expect(producer.includes("pub fn begin_retirement(&mut self)")).toBe(true);
  expect(source.includes("raw_retirement: std::mem::ManuallyDrop")).toBe(true);
  expect(source.includes("pub fn finish(&mut self)")).toBe(true);
  expect(source.includes("std::mem::replace(&mut *self.original, ReplayOwnedState::terminal())")).toBe(true);
  expect(producer.includes("impl<P, M: Mutation<P>> Drop for EditReplayResult")).toBe(true);
  expect(producer.includes("original residual custody remain")).toBe(true);
  console.log("[DEBUG] raw replay cancellation keeps all five original owner families; SQLite admits only whole physical release");
});
