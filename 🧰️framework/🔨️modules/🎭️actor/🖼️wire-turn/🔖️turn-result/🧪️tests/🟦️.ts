import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { drainTypedOperationBatches } from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

for (const sample of fixture.cases) test(sample.name, async () => {
  using database = new Database(":memory:");
  const { idle } = database.query("SELECT min(CAST(key AS integer) + 1) AS idle FROM json_each(?) WHERE value <> 'more-work'")
    .get(JSON.stringify(sample.turns)) as { idle: number };
  const limit = sample.budget === 0 ? 0 : (sample.closeAfter ?? sample.turns.length);
  const stopped = sample.budget === 0 ? "budget" : idle <= limit ? "idle" : "closed";
  const measured = Math.min(idle, limit);
  expect({ polls: measured, stopped, nextWake: stopped === "idle" ? sample.nextWake : null, yields: measured - Number(stopped === "idle") })
    .toEqual(sample.expected);

  let settled = 0;
  let yields = 0;
  const outcome = await drainTypedOperationBatches(sample.budget, () => settled < limit, async () => {
    const status = sample.turns[settled++];
    return { status, nextWake: status === "idle" ? sample.nextWake : null };
  }, async () => { yields++; });
  expect({ ...outcome, yields }).toEqual(sample.expected);
  expect(settled).toBe(sample.expected.polls);
});
