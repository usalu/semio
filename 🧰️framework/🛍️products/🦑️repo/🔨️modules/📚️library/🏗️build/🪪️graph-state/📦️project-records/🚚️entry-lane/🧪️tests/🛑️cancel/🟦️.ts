import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { existsSync, mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createGraphFleetClientV1 } from "../../../../🛰️exchange/📡️client/🟦️.ts";
import { createClosureEntryLaneV1 } from "../../🟦️.ts";
import cases from "./🔣️.json" with {type:"json"};

for (const row of cases) test("cold closure entry cancellation settles " + row.phase, async () => {
  const root = mkdtempSync(join(tmpdir(), "ec-")), controller = new AbortController(), reason = Error("original closure cancelled");
  const db = new Database(":memory:"), events: string[] = [];
  let entered!: () => void, unblock!: () => void, cancelled = false, releases = 0;
  const begun = new Promise<void>(resolve => { entered = resolve; });
  const claim = { state: "granted", untilMilliseconds: Date.now() + 60_000 };
  const pause = async (signal?: AbortSignal | null) => {
    let stop!: () => void;
    try {
      await new Promise<void>((resolve, reject) => {
        unblock = resolve;
        stop = () => { cancelled = true; reject(signal?.reason); };
        if (signal?.aborted) stop();
        else signal?.addEventListener("abort", stop, { once: true });
        events.push("began");
        entered();
      });
    } finally {
      signal?.removeEventListener("abort", stop);
      events.push("settled");
    }
  };
  const request = (async (input: RequestInfo | URL, init?: RequestInit) => {
    const url = new URL(String(input));
    if (url.pathname.endsWith("/release")) { releases++; events.push("released"); return new Response(); }
    if (url.pathname.endsWith("/await")) { await pause(init?.signal); return Response.json({ state: "stored" }); }
    if (init?.method === "POST") {
      if (row.phase === "late-grant") { await pause(init.signal); return Response.json(claim); }
      if (row.phase === "held-await") return Response.json({ ...claim, state: "held", by: "elsewhere" }, { status: 409 });
      return Response.json(claim);
    }
    if (row.phase === "lookup") await pause(init?.signal);
    return new Response(undefined, { status: 404 });
  }) as typeof fetch;
  const client = createGraphFleetClientV1({ endpoint: "http://graph.invalid", instance: "test", token: undefined, upload: true, stateDirectory: root }, { fetch: request });
  const lane = createClosureEntryLaneV1({ root, environment: {}, client, owner: "fixture" })!;
  const take = lane.take({ fn: "native-command" }, { export: async () => [], import: async () => { throw Error("Unexpected import"); } }, controller.signal);
  const result = take.then(value => ({ value }), error => ({ error }));
  try {
    await begun;
    events.push("cancel");
    controller.abort(reason);
    unblock();
    const outcome = await result;
    expect("error" in outcome ? outcome.error : undefined).toBe(reason);
    expect(cancelled).toBe(row.transportCancelled);
    expect(releases).toBe(row.releaseCount);
    expect(lane.stats()).toMatchObject({ computed: 0, fetched: 0, imported: 0 });
    expect(existsSync(join(root, "down"))).toBe(false);
    db.exec("create table lifecycle(event text)");
    for (const event of events) db.query("insert into lifecycle values(?)").run(event);
    expect(db.query("select count(*) released from lifecycle where event='released' and rowid > (select rowid from lifecycle where event='settled')").get()).toEqual({ released: row.releaseCount });
  } finally {
    await result;
    db.close();
    rmSync(root, { recursive: true, force: true });
  }
});
