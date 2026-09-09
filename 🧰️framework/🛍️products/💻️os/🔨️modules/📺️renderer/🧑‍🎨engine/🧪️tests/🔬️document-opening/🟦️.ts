import { describe, expect, it, vi } from "vitest";
import Ajv from "ajv";
import deepEqual from "fast-deep-equal";
import documentOpeningFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🛂️admission/📄️document/🔣️.json";
import rendererSchema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import { admitDocumentOpeningV1, BackgroundDocumentSessionsV1, DocumentAttachmentLaneV1, LatestDocumentReplacementV1, runDocumentOpeningAttemptV1 } from "../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";

describe("Shell document opening", () => {
  it("publishes direct document readiness only for the exact acknowledged mounted actor", async () => {
    const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🧫️fixtures/🖥️mounted/🔣️.json");
    const { browserDocumentMountIsCurrentV1 } = await import("../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts");
    const validate = new Ajv({ strict: true }).addSchema(rendererSchema).compile({ $ref: `${rendererSchema.$id}#/$defs/BrowserDocumentMountFixtureV1` });
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const observed: string[] = [];
    let mounted!: () => void, attaching!: () => void;
    const ready = new Promise<void>(resolve => { mounted = resolve; });
    const started = new Promise<void>(resolve => { attaching = resolve; });
    const opening = runDocumentOpeningAttemptV1({
      deadlineMs: 1000,
      current: () => true,
      socket: async () => { observed.push("socket"); },
      attach: async () => { observed.push("attach"); attaching(); await ready; },
      commit: () => { observed.push("commit"); },
      close: () => { observed.push("close"); },
      detach: async () => { observed.push("detach"); },
      retire: () => { observed.push("retire"); },
    });
    try {
      await started;
      for (const changed of [{ clientInstanceId: "foreign" }, { scope: { spaceId: "foreign", documentId: "map-a" } }, { scope: { spaceId: "space-a", documentId: "foreign" } }, { activationGeneration: "42" }, { verifiedSurfaceId: "foreign" }, { instanceId: 1 }, { revision: 4 }]) expect(browserDocumentMountIsCurrentV1(fixture.opening, fixture.retained, { ...fixture.receipt, ...changed })).toBe(false);
      expect(browserDocumentMountIsCurrentV1({ ...fixture.opening, instanceId: 8 }, fixture.retained, fixture.receipt)).toBe(false);
      expect(browserDocumentMountIsCurrentV1(fixture.opening, { ...fixture.retained, scope: { spaceId: "foreign", documentId: "map-a" } }, fixture.receipt)).toBe(false);
      expect(observed).toEqual(["socket", "attach"]);
      expect(browserDocumentMountIsCurrentV1(fixture.opening, fixture.retained, fixture.receipt)).toBe(true);
      observed.push("mounted");
      mounted();
      expect(await opening).toBe(true);
      expect(deepEqual(observed, fixture.sequence)).toBe(true);
    } finally {
      mounted();
      await opening;
    }
  });

  it("coalesces paused cold pairs and binds only the latest retained pair", async () => {
    const sequence: string[] = [];
    const queue = new LatestDocumentReplacementV1<string>();
    let release!: () => void, started!: () => void;
    const gate = new Promise<void>(resolve => { release = resolve; });
    const entered = new Promise<void>(resolve => { started = resolve; });
    const apply = async (value: string, current: () => boolean) => {
      sequence.push(`load-${value}`);
      if (value === "a") { started(); await gate; }
      if (current()) sequence.push(`bind-${value}`);
    };
    const first = queue.replace("a", apply);
    await entered;
    const second = queue.replace("b", apply);
    const third = queue.replace("c", apply);
    expect(await second).toBe(false);
    release();
    expect(await first).toBe(false);
    expect(await third).toBe(true);
    expect(deepEqual(sequence, documentOpeningFixture.latestColdReplacement)).toBe(true);
    expect(queue.pending).toBe(false);
  });

  it("cleans a failed cold lane without retiring a queued same-owner successor", async () => {
    const sequence: string[] = [];
    const lane = new DocumentAttachmentLaneV1(async () => { sequence.push("detach"); });
    let release!: () => void, started!: () => void;
    const gate = new Promise<void>(resolve => { release = resolve; });
    const entered = new Promise<void>(resolve => { started = resolve; });
    const first = lane.replace("a", () => true, async () => { sequence.push("load"); started(); await gate; throw new Error("load failed"); });
    const failed = expect(first).rejects.toThrow("load failed");
    await entered;
    const second = lane.replace("a", () => true, async () => { sequence.push("bind"); });
    release();
    await Promise.all([failed, second]);
    expect(deepEqual(sequence, documentOpeningFixture.failedColdReplacement)).toBe(true);
    await lane.close("a");
    expect(lane.idle).toBe(true);
    await expect(lane.replace("b", () => true, async () => { throw new Error("bind failed"); })).rejects.toThrow("bind failed");
    expect(lane.idle).toBe(true);
  });

  it("atomically retires, cold-loads and binds before a close can finish", async () => {
    const sequence: string[] = [];
    const validate = new Ajv({ strict: true }).addSchema(rendererSchema).compile({ type: "array", items: { $ref: `${rendererSchema.$id}#/$defs/DocumentOpeningAttachmentStepV1` } });
    expect(validate(documentOpeningFixture.coldReplacement)).toBe(true);
    let release!: () => void, started!: () => void;
    const gate = new Promise<void>(resolve => { release = resolve; });
    const entered = new Promise<void>(resolve => { started = resolve; });
    const lane = new DocumentAttachmentLaneV1(async () => { sequence.push("detach"); });
    await lane.attach("a", () => true, async () => { sequence.push("attach-a"); });
    const replacement = lane.replace("a", () => true, async () => {
      sequence.push("load"); started(); await gate; sequence.push("bind");
    });
    await entered;
    const closed = lane.close("a");
    expect(lane.idle).toBe(false);
    release();
    await Promise.all([replacement, closed]);
    expect(deepEqual(sequence, documentOpeningFixture.coldReplacement)).toBe(true);
    expect(lane.idle).toBe(true);
  });

  it("never lets a background admission replace an existing document or app instance", () => {
    for (const row of documentOpeningFixture.admissions) {
      const closed: string[] = [];
      const owners = new Map([
        ["document-a", { plugin: "plugin-a", session: { instanceId: 1 }, clientInstanceId: "owner-a" }],
        ["document-c", { plugin: "plugin-b", session: { instanceId: 2 }, clientInstanceId: "owner-c" }],
      ]);
      const admitted = admitDocumentOpeningV1({ ...row, plugin: "plugin-a" }, owners, (key, owner) => { closed.push(`${key}:${owner}`); owners.delete(key); });
      expect(admitted).toBe(row.admitted);
      expect(deepEqual(closed, row.closed)).toBe(true);
      expect(owners.has("document-c")).toBe(true);
    }
  });

  it("serializes background admissions and reaps them on invalidation, failure, and close", async () => {
    for (const [scenario, expected] of Object.entries(documentOpeningFixture.backgroundSessions)) {
      const observed: string[] = [];
      const sessions = new BackgroundDocumentSessionsV1<number>();
      let generation = 1;
      let fail = scenario === "retry";
      let complete!: () => void;
      const gate = new Promise<void>(resolve => { complete = resolve; });
      let begin!: () => void;
      const started = new Promise<void>(resolve => { begin = resolve; });
      const port = {
        current: (value: number) => value === generation,
        create: async () => {
          if (fail) { fail = false; observed.push("create-failed"); throw new Error("create rejected"); }
          observed.push(`create-${generation}`);
          begin();
          if (scenario === "closedDuringCreate") await gate;
          return generation;
        },
        release: async (value: number) => { observed.push(`release-${value}`); },
        visit: async (value: number) => { observed.push(`visit-${value}`); },
      };
      if (scenario === "closedDuringCreate") {
        const first = sessions.run("space-a", port);
        await started;
        const closed = sessions.close();
        complete();
        await Promise.all([first, closed]);
      } else if (scenario === "retry") {
        await expect(sessions.run("space-a", port)).rejects.toThrow("create rejected");
        await sessions.run("space-a", port);
      } else if (scenario === "replacement" || scenario === "explicitRetirement" || scenario === "authorityChanged") {
        await sessions.run("space-a", port);
        if (scenario === "explicitRetirement") await sessions.retire("space-a", value => value === 1);
        generation++;
        if (scenario === "authorityChanged") {
          await sessions.retain(value => value === generation);
          expect(observed).toEqual(["create-1", "visit-1", "release-1"]);
        }
        await sessions.run("space-a", port);
        if (scenario === "explicitRetirement") await sessions.retire("space-a", value => value === 1);
      } else await Promise.all([sessions.run("space-a", port), sessions.run("space-a", port)]);
      await sessions.close();
      await sessions.run("space-a", port);
      expect(deepEqual(observed, expected), scenario).toBe(true);
    }
    console.log("[DEBUG] Background document sessions: reuse=1 generation-retired=1 late-create-reaped=1 retry=1 explicit-close=1 authority-retired=1");
  });

  it("retires exact failed document admissions and socket timers without disturbing their replacement", async () => {
    const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(rendererSchema);
    const transition = ajv.getSchema(`${rendererSchema.$id}#/$defs/DocumentOpeningTransitionV1`)!;
    const admission = ajv.getSchema(`${rendererSchema.$id}#/$defs/DocumentOpeningAdmissionV1`)!;
    const closeFailure = ajv.getSchema(`${rendererSchema.$id}#/$defs/DocumentOpeningCloseFailureV1`)!;
    const background = ajv.getSchema(`${rendererSchema.$id}#/$defs/DocumentOpeningBackgroundSequenceV1`)!;
    expect(documentOpeningFixture.cases.every((row) => transition(row))).toBe(true);
    expect(documentOpeningFixture.admissions.every((row) => admission(row))).toBe(true);
    expect(documentOpeningFixture.closeFailures.every((row) => closeFailure(row))).toBe(true);
    expect(Object.values(documentOpeningFixture.backgroundSessions).every((row) => background(row))).toBe(true);
    vi.useFakeTimers();
    try {
      for (const row of documentOpeningFixture.cases) {
        const sequence: string[] = [];
        let mounted = "opening-a";
        let route: string | null = "opening-a";
        const work = runDocumentOpeningAttemptV1({
          current: () => mounted === "opening-a",
          socket: async () => {
            sequence.push("socket");
            if (row.fail === "socket") throw new Error("socket rejected");
            if (row.fail === "deadline") await new Promise(() => {});
            if (row.replace === "socket") mounted = route = "opening-b";
          },
          attach: async () => {
            sequence.push("attach");
            if (row.fail === "ready") await new Promise(() => {});
            if (row.fail === "attach") throw new Error("attach rejected");
            if (row.replace === "attach") mounted = route = "opening-b";
          },
          commit: () => { sequence.push("commit"); },
          close: () => { sequence.push("close"); if (route === "opening-a") route = null; },
          detach: async () => { sequence.push("detach"); },
          retire: () => { sequence.push("retire"); },
          deadlineMs: 25,
        }).then(ready => ready ? "ready" : "retired", () => "failed");
        await vi.runAllTimersAsync();
        expect(await work, row.id).toBe(row.outcome);
        expect(deepEqual(sequence, row.sequence), row.id).toBe(true);
        expect(vi.getTimerCount(), row.id).toBe(row.timers);
        expect(route, row.id).toBe(row.replace !== "none" ? "opening-b" : row.outcome === "failed" ? null : "opening-a");
      }
    } finally { vi.useRealTimers(); }
    console.log("[DEBUG] Document opening: neutral=7 failed-owner-cleanup=4 replacement-preserved=2 timer-leaks=0");
  }, 2_000);

  it("finishes physical cleanup even when route retirement throws", async () => {
    for (const row of documentOpeningFixture.closeFailures) {
      const sequence: string[] = [];
      await expect(runDocumentOpeningAttemptV1({
        current: () => row.attached,
        socket: async () => { sequence.push("socket"); },
        attach: async () => { sequence.push("attach"); throw new Error("attach rejected"); },
        commit: () => {},
        close: () => { sequence.push("close"); throw new Error("close rejected"); },
        detach: async () => { sequence.push("detach"); },
        retire: () => { sequence.push("retire"); },
        deadlineMs: 25,
      })).rejects.toThrow("close rejected");
      expect(deepEqual(sequence, row.sequence)).toBe(true);
    }
  });

  it("retires the previously attached document before admitting another document on the same app instance", async () => {
    const sequence: string[] = [];
    const lane = new DocumentAttachmentLaneV1(async () => { sequence.push("detach"); });
    await lane.attach("a", () => true, async () => { sequence.push("attach-a"); });
    await lane.attach("b", () => true, async () => { sequence.push("attach-b"); });
    await lane.close("a");
    expect(deepEqual(sequence, documentOpeningFixture.successorAttachment)).toBe(true);
    await lane.close("b");
    expect(lane.idle).toBe(true);
  });

  it("orders late attachment cleanup before replacement attachment and never detaches the replacement", async () => {
    const observed: string[] = [];
    let resolve!: () => void;
    const gate = new Promise<void>(done => { resolve = done; });
    const lane = new DocumentAttachmentLaneV1(async () => { observed.push("detach"); });
    const first = lane.attach("a", () => true, async () => { observed.push("attach-a"); await gate; observed.push("attached-a"); });
    await Promise.resolve();
    const closed = lane.close("a");
    let drained = false;
    const drain = lane.drain().then(() => { drained = true; });
    const replacement = lane.attach("b", () => true, async () => { observed.push("attach-b"); });
    expect(observed).toEqual(["attach-a"]);
    expect(drained).toBe(false);
    resolve();
    await Promise.all([first, closed, replacement, drain]);
    expect(drained).toBe(true);
    await lane.close("a");
    expect(deepEqual(observed, documentOpeningFixture.lateAttachment)).toBe(true);
    expect(lane.idle).toBe(false);
    await lane.close("b");
    expect(lane.idle).toBe(true);
    expect(observed.filter(event => event === "detach")).toHaveLength(2);
    console.log("[DEBUG] Document attachment: late-attach-retired=1 replacement-preserved=1 idle-lane-reaped=1");
  });
});
