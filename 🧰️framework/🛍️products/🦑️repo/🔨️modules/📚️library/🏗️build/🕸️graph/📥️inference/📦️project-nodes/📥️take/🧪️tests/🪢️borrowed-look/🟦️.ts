/** 🪢️ An optional take observer releases its own admission when it withdraws from another lane's look. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import * as take from "../../🟨️.mjs";
import source from "../../../🟨️.mjs" with { type: "text" };
import cases from "./🔣️.json" with { type: "json" };

function deferred<Outcome>() {
  let resolve!: (outcome: Outcome) => void;
  const promise = new Promise<Outcome>(complete => { resolve = complete; });
  return { promise, resolve };
}

for (const row of cases) test(row.case, async () => {
  const observer = new AbortController();
  const reason = new Error("observer withdraws");
  const first = deferred<take.TakeOutcomeV1>();
  const owner = deferred<take.TakeOutcomeV1>();
  const firstEntered = deferred<void>();
  const joined = deferred<void>();
  const inflight = new Map<string, Promise<take.TakeOutcomeV1>>();
  const state = { flying: 0 };
  let ownerSettled = 0, exactReason = 0, arrived = 0, drained = 0;
  let tally = { looks: 0, found: 0 };
  void owner.promise.then(() => { ownerSettled = 1; });
  const begin = source.indexOf("  const look=(job");
  const end = source.indexOf("  let staleTold", begin);
  const look = new Function("taken", "inflight", "joinTakeLookV1", source.slice(begin, end) + "return look")(
    new Set(), inflight, Reflect.get(take, "joinTakeLookV1"),
  ) as (job: { name: string }, lane: string, signal?: AbortSignal) => Promise<take.TakeOutcomeV1>;
  const attempt = take.takeAdmittedV1(async (name: string, signal) => {
    if (name === "first") {
      firstEntered.resolve();
      return first.promise;
    }
    joined.resolve();
    try { return await look({ name }, "adopt", signal); }
    catch (error) { exactReason = Number(error === reason); throw error; }
  }, () => undefined, observer.signal, state);
  const poll = take.takePollV1({
    queue: () => ["first", "shared"].filter(name => !inflight.has(name)),
    attempt, arrived: () => { arrived++; }, stop: () => arrived > 0,
    signal: observer.signal, concurrency: 1, sleep: async () => {},
  }).then(outcome => { tally = outcome; drained = 1; });

  await firstEntered.promise;
  inflight.set("shared", owner.promise);
  first.resolve("miss");
  await joined.promise;
  if (row.cancel) observer.abort(reason);
  else owner.resolve("fetched");

  const database = new Database(":memory:");
  try {
    const oracle = database.query(
      "select 1 as drained, 0 as flying, 1 as held, 1 - ?1 as ownerSettled, 2 as looks, 1 - ?1 as found, 1 - ?1 as arrived, ?1 as exactReason",
    ).get(Number(row.cancel));
    expect(oracle).toEqual(row.expected);
    await Bun.sleep(0);
    expect({
      drained, flying: state.flying, held: Number(inflight.get("shared") === owner.promise),
      ownerSettled, ...tally, arrived, exactReason,
    }).toEqual(row.expected);
  } finally {
    owner.resolve("fetched");
    await poll;
    database.close();
  }
});
