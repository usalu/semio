/** 🖼️ The asset decode path, runtime handback and parked presenter wake tests. */
import { readFileSync } from "node:fs";
import { dirname } from "node:path";
import { expect, it } from "vitest";
import { FrameTurnScheduler } from "@semio-tech/framework-renderer-wgpu/frame-turn-scheduler";
import { readFrameWorkerDirectory } from "../../📨️browser-frame-transport/📖️frame-worker-source/🟦️.ts";
import { BROWSER_WORKER_RS, FRAME_TURN_FIXTURE, FRAME_TURN_SCHEDULER_TS, FRAME_WORKER_TS, TwoKindFrameTurnScheduler } from "../🧰️step-budget-fixtures/🟦️.ts";
import type { WorkerTurnOwner } from "../🧰️step-budget-fixtures/🟦️.ts";

/** 🔔 Registers the off frame path asset decode, sealed asset retry, parked presenter checkout and reentrant wake tests. */
export function registerAssetDecodeAndPresenterWakeTests(): void {
  it("keeps asset decode off the public frame path and wakes presentation exactly when state changes", () => {
    const worker = readFrameWorkerDirectory(dirname(FRAME_WORKER_TS));
    const decodeAt = worker.indexOf("function runAssetDecodeTurn");
    expect(decodeAt).toBeGreaterThanOrEqual(0);
    const decode = worker.slice(decodeAt, worker.indexOf("\nfunction ", decodeAt + 1));
    expect(decode).not.toContain("runtime!.tick(");
    expect(decode).not.toContain('post({ kind: "frame"');
    expect(decode).toContain('post({ kind: "wake"');
    expect(decode).toContain('result.kind === "published"');
    const scheduler = readFileSync(FRAME_TURN_SCHEDULER_TS, "utf8");
    expect(scheduler).toContain('"assetDecode"');
    expect(scheduler).toContain("lastDispatched");
    expect(scheduler).not.toContain("Blocked");
  });
  it("retries a sealed asset after runtime handback without publishing an empty frame", () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8")) as {
      readonly runtimeWakeRetry: {
        readonly beforeHandback: "idle";
        readonly wakeOwners: readonly WorkerTurnOwner[];
        readonly afterHandback: "published";
        readonly publicMessages: readonly string[];
        readonly frameMessages: number;
      };
    };
    const callbacks: (() => void)[] = [];
    const owners: WorkerTurnOwner[] = [];
    const outcomes: string[] = [];
    const publicMessages: string[] = [];
    const frameMessages: string[] = [];
    let interactionAvailable = false;
    const scheduler = new TwoKindFrameTurnScheduler(
      (callback) => callbacks.push(callback),
      () => {
        owners.push("frame");
        return false;
      },
      () => true,
      () => {
        owners.push("assetDecode");
        if (!interactionAvailable) {
          outcomes.push("idle");
          return false;
        }
        outcomes.push("published");
        publicMessages.push("wake");
        return false;
      },
    );
    scheduler.request("assetDecode");
    callbacks.shift()!();
    expect(callbacks).toEqual([]);
    interactionAvailable = true;
    scheduler.requestRuntimeWake();
    for (let callback = callbacks.shift(); callback; callback = callbacks.shift()) {
      callback();
      expect(callbacks.length).toBeLessThanOrEqual(1);
    }
    expect(outcomes).toEqual([fixture.runtimeWakeRetry.beforeHandback, fixture.runtimeWakeRetry.afterHandback]);
    expect(owners).toEqual(["assetDecode", ...fixture.runtimeWakeRetry.wakeOwners]);
    expect(publicMessages).toEqual(fixture.runtimeWakeRetry.publicMessages);
    expect(frameMessages).toHaveLength(fixture.runtimeWakeRetry.frameMessages);
  });
  it("parks a live presenter checkout and preserves runtime wakes outside and inside its frame callback", () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8")).presenterWait as {
      continueFrame: false;
      callbacksWhileWaiting: number;
      wakeTiming: ("afterPark" | "insideFrame")[];
      callbackOwners: WorkerTurnOwner[];
      frameOutcomes: string[];
    };
    for (const timing of fixture.wakeTiming) {
      const callbacks: (() => void)[] = [];
      const owners: WorkerTurnOwner[] = [];
      const outcomes: string[] = [];
      const scheduler = new FrameTurnScheduler(
        (callback) => callbacks.push(callback),
        () => {
          owners.push("frame");
          outcomes.push(outcomes.length === 0 ? "awaitingRuntime" : "presented");
          if (outcomes.length === 1 && timing === "insideFrame") scheduler.requestRuntimeWake();
          return fixture.continueFrame;
        },
        () => true,
        () => { owners.push("assetDecode"); return false; },
      );
      scheduler.request();
      callbacks.shift()!();
      if (timing === "afterPark") {
        expect(callbacks).toHaveLength(fixture.callbacksWhileWaiting);
        scheduler.requestRuntimeWake();
      }
      for (let callback = callbacks.shift(); callback; callback = callbacks.shift()) {
        callback();
        expect(callbacks.length).toBeLessThanOrEqual(1);
        expect(owners.length).toBeLessThanOrEqual(fixture.callbackOwners.length);
      }
      expect(owners).toEqual(fixture.callbackOwners);
      expect(outcomes).toEqual(fixture.frameOutcomes);
    }
    const source = readFileSync(BROWSER_WORKER_RS, "utf8").replace(/\s+/gu, "");
    const start = source.indexOf("letcontinue_frame=");
    const continuation = source.slice(start, source.indexOf("letoutput=BrowserTickOutput{", start));
    expect(continuation).toMatch(/^letcontinue_frame=!host\.presenter\.awaiting_runtime\(\)&&\(host\.runtime\.has_pending_text_work\(\)\|\|/u);
    expect(continuation).toMatch(/\|\|host\.presenter\.has_pending_presentation\(\)\);$/u);
  });
  it("matches Chromium task delivery for parked presenter completion and reentrant wake", async () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8")).presenterWait;
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      const observed = await page.evaluate(async (timings: string[]) => {
        const results = [];
        for (const timing of timings) {
          results.push(await new Promise<{ owners: string[]; outcomes: string[]; callbacksWhileWaiting: number }>((complete) => {
            const channel = new MessageChannel();
            const owners: string[] = [];
            const outcomes: string[] = [];
            let callbacksWhileWaiting = -1;
            let handback!: () => void;
            const completion = new Promise<void>((resolve) => { handback = resolve; });
            void completion.then(() => {
              callbacksWhileWaiting = owners.length - 1;
              channel.port2.postMessage("assetDecode");
              channel.port2.postMessage("frame");
            });
            channel.port1.onmessage = (event: MessageEvent<string>) => {
              owners.push(event.data);
              if (event.data !== "frame") return;
              outcomes.push(outcomes.length === 0 ? "awaitingRuntime" : "presented");
              if (outcomes.length === 1) {
                if (timing === "insideFrame") handback();
                else setTimeout(handback, 0);
              } else {
                channel.port1.close();
                channel.port2.close();
                complete({ owners, outcomes, callbacksWhileWaiting });
              }
            };
            channel.port2.postMessage("frame");
          }));
        }
        return results;
      }, fixture.wakeTiming);
      for (const result of observed) {
        expect(result.owners).toEqual(fixture.callbackOwners);
        expect(result.outcomes).toEqual(fixture.frameOutcomes);
        expect(result.callbacksWhileWaiting).toBe(fixture.callbacksWhileWaiting);
      }
    } finally {
      await browser.close();
    }
  }, 15_000);
}
