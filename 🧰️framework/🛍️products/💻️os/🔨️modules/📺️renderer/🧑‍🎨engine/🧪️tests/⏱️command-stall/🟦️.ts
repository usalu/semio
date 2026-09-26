/** ⏱️ Laws of the command stall watch (`🔌️PluginRuntime/⏱️command-stall`): replays `🧫️fixtures/⏱️command-stall.json` on the
 * real per-actor ingress lane (`serializePerActor`) with the third-party fake clock (`@sinonjs/fake-timers` behind vitest)
 * as the oracle: a held lane is reported after the contract's bound, a settle clears the report, a cancel rejects the held
 * turn and releases the lane, and a queued turn is never reported while it waits. */
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import fixture from "../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/⏱️command-stall.json" with { type: "json" };
import { COMMAND_STALL_CONTRACT_V1, CommandCancelledErrorV1, commandStallBandTextV1, commandStallCancelTextV1, commandStallHeldSecondsV1, createCommandStallWatchV1 } from "../../🧱️elements/🔌️PluginRuntime/⏱️command-stall/🟦️.ts";
import { serializePerActor } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";

beforeEach(() => {
  vi.useFakeTimers();
  vi.spyOn(console, "warn").mockImplementation(() => undefined);
});
afterEach(() => {
  vi.restoreAllMocks();
  vi.useRealTimers();
});

describe("command stall watch (fixture)", () => {
  it("runs on the contract's own bound", () => {
    expect(fixture.boundMs).toBe(COMMAND_STALL_CONTRACT_V1.stallBoundMs);
  });

  for (const testCase of fixture.cases) {
    it(testCase.id, async () => {
      const watch = createCommandStallWatchV1({ now: () => Date.now(), setTimer: (run, delayMs) => setTimeout(run, delayMs), clearTimer: (handle) => clearTimeout(handle as ReturnType<typeof setTimeout>) }, fixture.boundMs);
      const actorId = `stall-law:${testCase.id}`;
      const resolved: string[] = [];
      const cancelled: string[] = [];
      for (const run of testCase.runs) {
        const turn = serializePerActor(actorId, () =>
          watch.watch({ actorId, programId: "law", commandId: run.command }, () => (run.settlesAtMs === null ? new Promise<void>(() => undefined) : new Promise<void>((resolve) => setTimeout(resolve, run.settlesAtMs!)))),
        );
        turn.then(
          () => resolved.push(run.command),
          (error: unknown) => {
            expect(error).toBeInstanceOf(CommandCancelledErrorV1);
            cancelled.push(run.command);
          },
        );
        if ("cancelAtMs" in run && typeof run.cancelAtMs === "number") setTimeout(() => {
          const stall = watch.stalls().find((entry) => entry.commandId === run.command);
          expect(stall, `${run.command} is reported when it is cancelled`).toBeDefined();
          expect(watch.cancel(stall!.id)).toBe(true);
        }, run.cancelAtMs);
      }
      let clock = 0;
      for (const check of testCase.expect) {
        await vi.advanceTimersByTimeAsync(check.atMs - clock);
        clock = check.atMs;
        expect(watch.stalls().map((stall) => stall.commandId), `stalled at ${check.atMs} ms`).toEqual(check.stalled);
        expect([...resolved].sort(), `resolved at ${check.atMs} ms`).toEqual([...check.resolved].sort());
        expect([...cancelled].sort(), `cancelled at ${check.atMs} ms`).toEqual([...check.cancelled].sort());
      }
    });
  }

  it("reads at least the bound for a reported stall, even on a display clock that has not ticked since the report", () => {
    const bound = COMMAND_STALL_CONTRACT_V1.stallBoundMs;
    expect(commandStallHeldSecondsV1({ startedAtMs: 1_000 }, 1_000)).toBe(bound / 1_000);
    expect(commandStallHeldSecondsV1({ startedAtMs: 1_000 }, 0)).toBe(bound / 1_000);
    expect(commandStallHeldSecondsV1({ startedAtMs: 1_000 }, 1_000 + bound + 7_000)).toBe(bound / 1_000 + 7);
    expect(commandStallBandTextV1("semio · dag", null, commandStallHeldSecondsV1({ startedAtMs: 500 }, 500), "de")).toContain(`seit ${bound / 1_000} s`);
  });

  it("names the program and the command in English and German, and never offers a cancel for a settled turn", async () => {
    expect(commandStallBandTextV1("semio · trinity", "Clear Selection", 7, "en")).toBe("semio · trinity has not answered “Clear Selection” for 7 s — later input waits for it.");
    expect(commandStallBandTextV1("semio · trinity", "Auswahl aufheben", 7, "de")).toBe("semio · trinity antwortet seit 7 s nicht auf „Auswahl aufheben“ — spätere Eingaben warten darauf.");
    expect(commandStallBandTextV1("semio · dag", null, 12, "de")).toContain("„eine Interaktion“");
    expect([commandStallCancelTextV1("en"), commandStallCancelTextV1("de")]).toEqual(["Cancel command", "Befehl abbrechen"]);
    const watch = createCommandStallWatchV1({ now: () => Date.now(), setTimer: (run, delayMs) => setTimeout(run, delayMs), clearTimer: (handle) => clearTimeout(handle as ReturnType<typeof setTimeout>) }, 10);
    await watch.watch({ actorId: "settled", programId: "law", commandId: "addNode" }, async () => undefined);
    expect(watch.cancel(1)).toBe(false);
  });
});
