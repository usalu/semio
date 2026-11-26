/** ⏳️ Laws of the document-load gate over the language-neutral `🔣️.json` fixture: which inputs a load drops, defers or passes, and that
 * deferred inputs replay in arrival order exactly once the last load settles, never earlier and never past their capacity. */
import { describe, expect, test } from "bun:test";
import fixture from "./🔣️.json" with { type: "json" };
import { dispatchAfterDocumentLoadV1, createDocumentLoadGateV1, documentLoadRefusalDetailV1, documentLoadRunningV1, documentLoadWaitPolicyV1, noteDocumentLoadEndV1 } from "../../🧱️elements/🏛️ShellHost/⏳️document-load-gate/🟦️.ts";

describe("document load wait policy", () => {
  for (const row of fixture.policy) test(`${row.action} (${row.kind ?? "undeclared"}) ${row.expect}`, () => {
    expect(documentLoadWaitPolicyV1(row.action, row.kind ?? undefined, fixture.ephemeralNoteActionId)).toBe(row.expect as "pass" | "drop" | "defer");
  });
});

describe("document load gate", () => {
  test("is idle without a load and settles at once", async () => {
    const gate = createDocumentLoadGateV1();
    expect(gate.loading("sourcing", 1)).toBe(false);
    expect(await gate.settled("sourcing", 1)).toBe("settled");
  });

  test("replays deferred inputs in arrival order after the last overlapping load releases", async () => {
    const gate = createDocumentLoadGateV1();
    const first = gate.begin("sourcing", 1);
    const second = gate.begin("sourcing", 1);
    const order: string[] = [];
    const parked = ["a", "b", "c"].map((name) => gate.settled("sourcing", 1).then(() => order.push(name)));
    expect(gate.deferred("sourcing", 1)).toBe(3);
    first();
    first();
    await Promise.resolve();
    expect(order).toEqual([]);
    expect(gate.loading("sourcing", 1)).toBe(true);
    second();
    await Promise.all(parked);
    expect(order).toEqual(["a", "b", "c"]);
    expect(gate.loading("sourcing", 1)).toBe(false);
    expect(gate.deferred("sourcing", 1)).toBe(0);
  });

  test("keeps instances and plugins apart", async () => {
    const gate = createDocumentLoadGateV1();
    const release = gate.begin("sourcing", 1);
    expect(gate.loading("sourcing", 2)).toBe(false);
    expect(gate.loading("puzzle", 1)).toBe(false);
    expect(await gate.settled("sourcing", 2)).toBe("settled");
    release();
  });

  test("answers queue-full past its capacity without parking", async () => {
    const gate = createDocumentLoadGateV1(fixture.capacity);
    const release = gate.begin("sourcing", 1);
    const parked = [gate.settled("sourcing", 1), gate.settled("sourcing", 1)];
    expect(await gate.settled("sourcing", 1)).toBe("queue-full");
    expect(gate.deferred("sourcing", 1)).toBe(fixture.capacity);
    release();
    expect(await Promise.all(parked)).toEqual(["settled", "settled"]);
  });

  test("a cancelled deferred input leaves the queue and the others still replay", async () => {
    const gate = createDocumentLoadGateV1();
    const release = gate.begin("sourcing", 1);
    const abort = new AbortController();
    const cancelled = gate.settled("sourcing", 1, abort.signal).then(() => "settled", (reason) => reason);
    const kept = gate.settled("sourcing", 1);
    abort.abort("cancelled");
    expect(await cancelled).toBe("cancelled");
    expect(gate.deferred("sourcing", 1)).toBe(1);
    release();
    expect(await kept).toBe("settled");
  });

  test("a load that begins again after settling parks new inputs again", async () => {
    const gate = createDocumentLoadGateV1();
    gate.begin("sourcing", 1)();
    const release = gate.begin("sourcing", 1);
    let replayed = false;
    const parked = gate.settled("sourcing", 1).then(() => { replayed = true; });
    await Promise.resolve();
    expect(replayed).toBe(false);
    release();
    await parked;
    expect(replayed).toBe(true);
  });
});

describe("document load end", () => {
  for (const row of fixture.loadEnds) test(`${row.cancelled === null ? "a loaded document" : row.cancelled ? "a cancelled load" : "a failed load"} ${row.refusal === null ? "leaves its action applied" : `refuses its action as ${row.refusal}`}`, () => {
    const owner: { documentLoad?: "cancelled" | "failed" } = {};
    if (row.cancelled !== null) noteDocumentLoadEndV1(owner, row.cancelled);
    expect(owner.documentLoad ?? null).toBe(row.documentLoad as "cancelled" | "failed" | null);
    expect(documentLoadRefusalDetailV1(owner) ?? null).toBe(row.refusal);
  });
});

describe("document load cancel", () => {
  test("a cancelled load reads as stopping until its last release, and only while one runs", () => {
    const gate = createDocumentLoadGateV1();
    gate.markCancelling("sourcing", 1);
    expect(gate.cancelling("sourcing", 1)).toBe(false);
    const release = gate.begin("sourcing", 1);
    expect(gate.cancelling("sourcing", 1)).toBe(false);
    gate.markCancelling("sourcing", 1);
    expect(gate.cancelling("sourcing", 1)).toBe(true);
    expect(gate.cancelling("sourcing", 2)).toBe(false);
    release();
    expect(gate.cancelling("sourcing", 1)).toBe(false);
  });

  test("only a pending or running status shows the loading band", () => {
    expect(["pending", "running", "ready", "cancelled", "fault"].map(documentLoadRunningV1)).toEqual([true, true, false, false, false]);
  });
});


test("scheduled document work waits for its own load and retries only a refused pre-dispatch", async () => {
  for (const row of fixture.scheduled) {
    const gate = createDocumentLoadGateV1(), release = gate.begin("procedural", 9);
    let calls = 0, current = true;
    const loading = new Error("document.loading"), fatal = new Error("unrelated");
    const task = dispatchAfterDocumentLoadV1(gate, "procedural", 9, () => current, async () => {
      calls++;
      if (row.case === "refusal" && calls === 1) throw loading;
      if (row.case === "fatal") throw fatal;
    }, error => error === loading);
    expect(calls).toBe(0);
    current = row.live;
    release();
    if (row.case === "fatal") await expect(task).rejects.toBe(fatal);
    else await task;
    expect(calls).toBe(row.attempts);
  }
});
