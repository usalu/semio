import { describe, expect, it, vi } from "vitest";
import Ajv from "ajv";
import deepEqual from "fast-deep-equal";
import documentOpeningFixture from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🚪️opening/📄️document/🔣️.json";
import documentOpeningSchema from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🚪️opening/📄️document/🧬️.schema.json";
import { runDocumentOpeningAttemptV1 } from "../../../../🧱️elements/🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🚪️opening/📄️document/🟦️.ts";

describe("Shell document opening", () => {
  it("retires exact failed document admissions and socket timers without disturbing their replacement", async () => {
    expect(new Ajv({ strict: true }).compile(documentOpeningSchema)(documentOpeningFixture)).toBe(true);
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
            if (row.fail === "attach") throw new Error("attach rejected");
            if (row.replace === "attach") mounted = route = "opening-b";
          },
          commit: () => { sequence.push("commit"); },
          close: () => { sequence.push("close"); if (route === "opening-a") route = null; },
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
    console.log("[DEBUG] Document opening: neutral=6 failed-owner-cleanup=3 replacement-preserved=2 timer-leaks=0");
  });
});
