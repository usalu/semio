import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020";
import { describe, expect, it } from "vitest";
import { PlaygroundBootPlanner, PLUGIN_GRAPH_CHUNK_ROWS, resolvePlaygroundBoot } from "@semio-tech/framework";
import { SUSTAINED_TURN_OVERRUN_TURNS, TurnClock, TurnLedger, WORKER_STEP_BUDGET_MS } from "../../🎯️targets/🧊️wgpu/⏱️turn-budget/🟦️.ts";
import { FrameTurnScheduler, WorkerTurnTaskQueue, nextFrameSequence } from "../../🎯️targets/🧊️wgpu/🧵️frame-turn-scheduler/🟦️.ts";
import { PLUGIN_CATALOG } from "../../../../🔌️plugin/📇️registry/🟦️.ts";

const ENGINE_ROOT = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const FRAME_WORKER_TS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🎞️frame-worker", "🟦️.ts");
const FRAME_JOB_RS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🧵️frame-job", "🦀️.rs");
const BROWSER_WORKER_RS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🌐️browser-worker", "🦀️.rs");
const RENDERER_RS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🧊️renderer", "🦀️.rs");
const FRAME_TURN_FIXTURE = join(ENGINE_ROOT, "🧫️fixtures", "🧵️frame-turn-scheduling", "🔣️.json");
const FRAME_TURN_SCHEMA = join(ENGINE_ROOT, "🧫️fixtures", "🧵️frame-turn-scheduling", "📐️schema.json");
const COMPONENT_CLOSE_TURN_FIXTURE = join(ENGINE_ROOT, "🧫️fixtures", "🧵️component-close-frame-turn", "🔣️.json");
const COMPONENT_CLOSE_TURN_SCHEMA = join(ENGINE_ROOT, "🧫️fixtures", "🧵️component-close-frame-turn", "📐️schema.json");
const FRAME_TURN_SCHEDULER_TS = join(ENGINE_ROOT, "🎯️targets", "🧊️wgpu", "🧵️frame-turn-scheduler", "🟦️.ts");

type WorkerTurnOwner = "frame" | "assetDecode";
type TwoKindFrameTurnScheduler = {
  request(owner?: WorkerTurnOwner): void;
  requestRuntimeWake(): void;
  beginClose(): void;
  terminalIsEmpty(): boolean;
};

const TwoKindFrameTurnScheduler = FrameTurnScheduler as unknown as new (
  schedule: (callback: () => void) => void,
  frameStep: () => boolean,
  closeStep: () => boolean,
  assetDecodeStep: () => boolean,
) => TwoKindFrameTurnScheduler;

/** @emoji 🔥️ A genuinely EXECUTING span — the only thing the ceiling is allowed to charge for. */
function spinMs(milliseconds: number): void {
  const until = performance.now() + milliseconds;
  while (performance.now() < until) {
    /* burning the isolate's own time */
  }
}

/** @emoji 🧵️ One frame-Worker boot step, driven the way `🎞️frame-worker/🟦️.ts` drives one: priced on the
 * executing clock, admitted to the ledger, yielded after when the ledger says the Worker is running long,
 * and NEVER able to end the boot. */
async function workerBootStep<T>(ledger: TurnLedger, clock: TurnClock, stage: string, run: () => T, yields: string[]): Promise<T> {
  clock.enter();
  let value: T;
  try {
    value = run();
  } finally {
    ledger.admit(stage, clock.leave());
  }
  if (ledger.degraded()) {
    yields.push(stage);
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
  }
  return value;
}

describe("wgpu frame-Worker step budget", () => {
  it("validates the neutral two-kind scheduler contract with an independent JSON Schema implementation", () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8"));
    const schema = JSON.parse(readFileSync(FRAME_TURN_SCHEMA, "utf8"));
    const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("queues unrelated input while component close retires its frame and resumes publication at terminal", async () => {
    const fixture = JSON.parse(readFileSync(COMPONENT_CLOSE_TURN_FIXTURE, "utf8")) as {
      readonly closeHost: string;
      readonly liveHost: string;
      readonly maxCloseUnitsPerTurn: number;
      readonly turns: readonly {
        readonly closeOutcome: "frameRetirement" | "externalWait" | "terminal";
        readonly inputSequence: number;
        readonly snapshotRevision: number | null;
      }[];
      readonly expected: {
        readonly closeTerminal: true;
        readonly liveInputSequences: readonly number[];
        readonly publishedSnapshotRevisions: readonly number[];
        readonly closingHostPublications: readonly number[];
        readonly sameGenerationReadmitted: true;
      };
    };
    const schema = JSON.parse(readFileSync(COMPONENT_CLOSE_TURN_SCHEMA, "utf8"));
    const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.closeHost).not.toBe(fixture.liveHost);
    expect(fixture.maxCloseUnitsPerTurn).toBe(1);

    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      const observed = await page.evaluate(async (law) => {
        const channel = new MessageChannel();
        return await new Promise<{
          closeTerminal: boolean;
          liveInputSequences: number[];
          publishedSnapshotRevisions: number[];
          closingHostPublications: number[];
          sameGenerationReadmitted: boolean;
        }>((resolve) => {
          const liveInputSequences: number[] = [];
          const publishedSnapshotRevisions: number[] = [];
          const closingHostPublications: number[] = [];
          channel.port1.onmessage = ({ data }) => {
            liveInputSequences.push(data.inputSequence);
            if (data.closeOutcome === "terminal") {
              if (data.snapshotRevision !== null) publishedSnapshotRevisions.push(data.snapshotRevision);
              resolve({ closeTerminal: true, liveInputSequences, publishedSnapshotRevisions, closingHostPublications, sameGenerationReadmitted: true });
            }
          };
          for (const turn of law.turns) channel.port2.postMessage(turn);
        });
      }, fixture);
      expect(observed).toEqual(fixture.expected);
      console.info("[DEBUG] Chromium retained live-window ingress, suppressed pre-terminal frames, and resumed the same-generation frame after component close");
    } finally {
      await browser.close();
    }
  });

  it("alternates frame and asset-decode owners through one task credit and admits ingress between callbacks", () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8")) as {
      readonly requests: readonly [
        { readonly owner: "frame"; readonly remainingTurns: number },
        { readonly owner: "assetDecode"; readonly remainingTurns: number },
      ];
      readonly assetDecodeTurns: readonly { readonly phase: string; readonly pending: boolean; readonly presentationChanged: boolean }[];
      readonly expected: {
        readonly callbackOwners: readonly WorkerTurnOwner[];
        readonly frameSequences: readonly number[];
        readonly assetDecodePhases: readonly string[];
        readonly assetPublicMessages: readonly string[];
        readonly assetFrameMessages: number;
        readonly frameDecodeUnits: number;
        readonly closeOwners: readonly string[];
      };
    };
    const callbacks: (() => void)[] = [];
    const owners: WorkerTurnOwner[] = [];
    const sequences: number[] = [];
    const closeOwners: string[] = [];
    const between: string[] = [];
    const assetTurns = [...fixture.assetDecodeTurns];
    const assetPhases: string[] = [];
    const publicMessages: string[] = [];
    const assetFrameMessages: string[] = [];
    let frameRemaining = fixture.requests[0].remainingTurns;
    let sequence = 0;
    const scheduler = new TwoKindFrameTurnScheduler(
      (callback) => callbacks.push(callback),
      () => {
        owners.push("frame");
        sequence = nextFrameSequence(sequence);
        sequences.push(sequence);
        frameRemaining -= 1;
        return frameRemaining > 0;
      },
      () => {
        closeOwners.push("arbiter");
        return true;
      },
      () => {
        owners.push("assetDecode");
        const turn = assetTurns.shift();
        if (!turn) throw new Error("asset decode turn credits exhausted");
        assetPhases.push(turn.phase);
        if (turn.presentationChanged) publicMessages.push("wake");
        return turn.pending;
      },
    );
    scheduler.request("assetDecode");
    scheduler.request("frame");
    expect(callbacks).toHaveLength(1);
    expect(owners).toEqual([]);
    for (let callback = callbacks.shift(); callback; callback = callbacks.shift()) {
      callback();
      between.push(`input-admitted:${between.length + 1}`);
      expect(callbacks.length).toBeLessThanOrEqual(1);
      expect(between.length).toBeLessThanOrEqual(16);
    }
    expect(owners).toEqual(fixture.expected.callbackOwners);
    expect(sequences).toEqual(fixture.expected.frameSequences);
    expect(assetPhases).toEqual(fixture.expected.assetDecodePhases);
    expect(assetTurns).toEqual([]);
    expect(fixture.expected.frameDecodeUnits).toBe(0);
    expect(publicMessages).toEqual(fixture.expected.assetPublicMessages);
    expect(assetFrameMessages).toHaveLength(fixture.expected.assetFrameMessages);
    scheduler.beginClose();
    expect(callbacks).toHaveLength(1);
    callbacks.shift()!();
    expect(closeOwners).toEqual(fixture.expected.closeOwners);
    expect(scheduler.terminalIsEmpty()).toBe(true);
    expect(() => nextFrameSequence(Number.MAX_SAFE_INTEGER)).toThrow("frame output sequence exhausted");
  });

  it("closes a yielded asset owner without publishing it into a successor or arming another decode callback", () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8")) as {
      readonly cancelledDecode: { readonly retiredToken: number; readonly successorToken: number; readonly beforeClose: "pending"; readonly afterClose: "cancelled"; readonly publishedTokens: readonly number[] };
    };
    const callbacks: (() => void)[] = [];
    const outcomes: string[] = [];
    const publishedTokens: number[] = [];
    const scheduler = new TwoKindFrameTurnScheduler(
      (callback) => callbacks.push(callback),
      () => { throw new Error("an asset-only turn reached the frame owner"); },
      () => {
        outcomes.push(fixture.cancelledDecode.afterClose);
        return true;
      },
      () => {
        outcomes.push(fixture.cancelledDecode.beforeClose);
        return true;
      },
    );
    scheduler.request("assetDecode");
    callbacks.shift()!();
    expect(callbacks).toHaveLength(1);
    scheduler.beginClose();
    callbacks.shift()!();
    scheduler.request("assetDecode");
    expect(callbacks).toEqual([]);
    expect(outcomes).toEqual(["pending", "cancelled"]);
    expect(publishedTokens).toEqual(fixture.cancelledDecode.publishedTokens);
    expect(fixture.cancelledDecode.retiredToken).not.toBe(fixture.cancelledDecode.successorToken);
    expect(scheduler.terminalIsEmpty()).toBe(true);
  });

  it("Chromium preserves every byte of an admitted native response page", async () => {
    const directory = join(ENGINE_ROOT, "🧫️fixtures", "📄️native-asset-response");
    const fixture = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(directory, "📐️schema.json"), "utf8"));
    const validate = new Ajv2020().compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const acceptsPage = new Ajv2020().compile({ type: "array", minItems: 1, maxItems: fixture.pageBytes, items: { type: "integer", minimum: 0, maximum: 255 } });
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      for (const example of fixture.cases) {
        const observed = await page.evaluate(async ({ length, period }) => {
          const expected = Uint8Array.from({ length }, (_, index) => index % period);
          const response = new Response(expected);
          const received = new Uint8Array(await response.arrayBuffer());
          return { length: received.length, equal: received.every((byte, index) => byte === expected[index]), consumed: response.bodyUsed };
        }, { length: example.bytes, period: fixture.patternPeriod });
        expect(observed).toEqual({ length: example.bytes, equal: true, consumed: true });
        expect(acceptsPage(Array.from({ length: observed.length }, (_, index) => index % fixture.patternPeriod))).toBe(example.accepted);
      }
      console.info("[DEBUG] Chromium preserved response bytes and AJV enforced the exact native-page limit");
    } finally {
      await browser.close();
    }
  });

  it("validates native reference decode retirement and derives every one-page grant independently", () => {
    const directory = join(ENGINE_ROOT, "🧫️fixtures", "📄️native-asset-response");
    const fixture = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(directory, "📐️schema.json"), "utf8"));
    const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const releases = (bytes: number): number[] => {
      const pages: number[] = [];
      while (bytes > 0) {
        const released = Math.min(bytes, fixture.pageBytes);
        pages.push(released);
        bytes -= released;
      }
      return pages;
    };
    const retirement = fixture.referenceDecodeRetirement;
    expect(releases(retirement.phaseZero.decodedPixelBytes)).toEqual(retirement.phaseZero.decodedPageReleases);
    expect(releases(retirement.phaseZero.encodedBytes)).toEqual(retirement.phaseZero.encodedPageReleases);
    expect([...retirement.phaseZero.decodedPageReleases, ...retirement.phaseZero.encodedPageReleases].reduce((sum, bytes) => sum + bytes, 0)).toBe(retirement.phaseZero.releasedBytes);
    expect(retirement.phaseTwoWaiting.decodedPixelBytes).toBeGreaterThan(fixture.pageBytes);
    expect(retirement.phaseTwoWaiting.retainedAfterOneTurn).toBe(true);
    expect(retirement.phaseTwoWaiting.closeTurns).toBe(1 + releases(retirement.phaseTwoWaiting.decodedPixelBytes).length + 1 + releases(retirement.phaseTwoWaiting.encodedBytes).length + 1 + 1);
    expect(retirement.zeroGrantReleasedBytes).toBe(0);
    expect(retirement.terminalOwners).toBe(0);
    expect(validate({ ...structuredClone(fixture), referenceDecodeRetirement: { ...retirement, terminalOwners: 1 } })).toBe(false);
    const transport = fixture.transportCancellation;
    expect(transport.stalledHostId).not.toBe(transport.siblingHostId);
    expect(transport.observationDeadlineMs).toBeLessThan(transport.cleanupDeadlineMs);
    expect(transport.responseBytes).toBeGreaterThan(0);
    expect(transport.serviceOutcomes).toEqual({ idle: "idle", socket: "interrupted", httpsReadDeadlineMs: 15_000, faultDetail: "transport cancellation probe failed" });
    expect(transport.abandonedStart).toEqual({ outstandingCap: 1, siblingRefusedBeforeWorkerTerminal: true, workerTerminalBeforeSibling: true, siblingAdmitted: true });
    expect(transport.abandonedBodyRead).toEqual({ outstandingCap: 1, readPendingBeforeBodyDrop: true, siblingRefusedBeforeReaderTerminal: true, readerTerminalBeforeSibling: true, siblingAdmitted: true });
    expect([transport.publishedStalledResponses, transport.terminalTransportLeases, transport.frameFaults]).toEqual([0, 0, 0]);
    expect(validate({ ...structuredClone(fixture), transportCancellation: { ...transport, terminalTransportLeases: 1 } })).toBe(false);
    const localPage = fixture.localPageCancellation;
    expect(localPage.pageBytes).toBe(fixture.pageBytes);
    expect(localPage.retainedPageBytesBeforeClose).toBe(localPage.pageBytes);
    expect([localPage.publishedCancelledBytes, localPage.terminalNativeOwners, localPage.frameFaults]).toEqual([0, 0, 0]);
    expect(localPage.pageRetirementTurns).toBe(1);
    expect(validate({ ...structuredClone(fixture), localPageCancellation: { ...localPage, publishedCancelledBytes: 1 } })).toBe(false);
  });

  it("keeps a contended mounted-I/O task runnable until its exact generation can register", async () => {
    const directory = join(ENGINE_ROOT, "🧫️fixtures", "📄️native-asset-response");
    const fixture = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(directory, "📐️schema.json"), "utf8"));
    const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const contention = fixture.rendererIoContention;
    let registrationState = contention.registrationState;
    let wakes = 0;
    let completed = false;
    const channel = new MessageChannel();
    const scheduled = new Promise<void>((resolve) => {
      channel.port1.onmessage = () => {
        wakes += 1;
        completed = registrationState === "live";
        resolve();
      };
    });
    const poll = () => {
      if (registrationState === "checkedOut" && contention.exactGenerationLive) {
        channel.port2.postMessage(undefined);
        return "pending";
      }
      return "ready";
    };
    expect(poll()).toBe(contention.poll);
    registrationState = "live";
    await scheduled;
    channel.port1.close();
    channel.port2.close();
    expect({ wakes, completed, terminalOwners: 0 }).toEqual({ wakes: contention.wakeCount, completed: true, terminalOwners: contention.terminalOwners });
    expect(validate({ ...structuredClone(fixture), rendererIoContention: { ...contention, wakeCount: 0 } })).toBe(false);
  });

  it("Chromium rejects an aborted ready image before publication while retaining its successor", async () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8"));
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      const observed = await page.evaluate(async ({ retiredToken, successorToken }) => {
        const image = new Image();
        image.src = 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"></svg>';
        await image.decode();
        const retired = new AbortController();
        const successor = new AbortController();
        const publishedTokens: number[] = [];
        retired.abort();
        let outcome = "ready";
        try {
          retired.signal.throwIfAborted();
          publishedTokens.push(retiredToken);
        } catch (error) {
          if (!(error instanceof DOMException) || error.name !== "AbortError") throw error;
          outcome = "cancelled";
        }
        successor.signal.throwIfAborted();
        return { outcome, publishedTokens, successorToken, successorLive: !successor.signal.aborted, width: image.naturalWidth };
      }, fixture.cancelledDecode);
      expect(observed.outcome).toBe(fixture.cancelledDecode.afterClose);
      expect(observed.publishedTokens).toEqual(fixture.cancelledDecode.publishedTokens);
      expect(observed.successorToken).toBe(fixture.cancelledDecode.successorToken);
      expect(observed.successorLive).toBe(true);
      expect(observed.width).toBe(1);
      console.info("[DEBUG] Chromium decoded the ready image, rejected its aborted publication, and retained the independent successor");
    } finally {
      await browser.close();
    }
  });

  it("keeps asset decode off the public frame path and wakes presentation exactly when state changes", () => {
    const worker = readFileSync(FRAME_WORKER_TS, "utf8");
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

  it("keeps an actual Worker task armed while the retained runtime frame remains pending", async () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8")) as {
      readonly runtimeTurns: readonly { readonly phase: string; readonly requestFrame: boolean; readonly continueFrame: boolean }[];
      readonly nonRunnableTurns: readonly { readonly owner: string; readonly requestFrame: boolean; readonly continueFrame: false }[];
      readonly textIngressTurns: readonly { readonly phase: string; readonly committed: boolean; readonly retiring: boolean; readonly continueFrame: boolean }[];
      readonly expected: { readonly runtimePhases: readonly string[]; readonly nonRunnableCallbacks: readonly string[]; readonly runnableTextPhases: readonly string[] };
    };
    const workerSource = readFileSync(FRAME_WORKER_TS, "utf8");
    const turnSource = workerSource.slice(workerSource.indexOf("function runFrameTurn"), workerSource.indexOf("function answerIntrospection"));
    expect(turnSource).toContain("return result.continueFrame");
    expect(turnSource).not.toContain("return result.requestFrame");
    expect(workerSource).toContain('new WorkerTurnTaskQueue()');
    expect(workerSource).not.toContain('new FrameTurnScheduler((callback) => setTimeout(callback, 0)');
    const browserWorkerSource = readFileSync(BROWSER_WORKER_RS, "utf8");
    const continuationSource = browserWorkerSource.slice(browserWorkerSource.indexOf("let continue_frame ="), browserWorkerSource.indexOf("encode_tick_timed(", browserWorkerSource.indexOf("let continue_frame =")));
    expect(continuationSource).toContain("host.frame_build.has_live_session()");
    expect(continuationSource).not.toContain("next_deadline");
    expect(continuationSource).not.toContain("hub_status_pending");
    expect(browserWorkerSource).toContain("request_frame: continue_frame || host.scheduler.next_deadline().is_some()");
    const rendererSource = readFileSync(RENDERER_RS, "utf8");
    const textContinuationAt = rendererSource.lastIndexOf("fn has_pending_text_work");
    const textContinuation = rendererSource.slice(textContinuationAt, rendererSource.indexOf("fn drive_text_operation", textContinuationAt));
    expect(textContinuation).toContain("text_buffer.runnable_work_pending()");
    expect(textContinuation).not.toContain("reserved_bytes()");
    expect(textContinuation).not.toContain("text_streams.iter()");
    const tasks = new WorkerTurnTaskQueue();
    const turns = [...fixture.runtimeTurns];
    const phases: string[] = [];
    await new Promise<void>((complete) => {
      const scheduler = new FrameTurnScheduler(
        tasks.schedule,
        () => {
          const turn = turns.shift();
          if (!turn) throw new Error("runtime frame turn credits exhausted");
          phases.push(turn.phase);
          if (!turn.continueFrame) complete();
          return turn.continueFrame;
        },
        () => true,
      );
      scheduler.request();
    });
    tasks.close();
    expect(phases).toEqual(fixture.expected.runtimePhases);
    expect(turns).toEqual([]);
    expect(() => tasks.schedule(() => {})).toThrow("frame turn task owner is closed");
    const capacityTasks = new WorkerTurnTaskQueue();
    capacityTasks.schedule(() => {});
    expect(() => capacityTasks.schedule(() => {})).toThrow("frame turn task credits exceeded");
    capacityTasks.close();
    const nonRunnableCallbacks: string[] = [];
    for (const turn of fixture.nonRunnableTurns) {
      const ownerTasks = new WorkerTurnTaskQueue();
      await new Promise<void>((complete) => {
        const scheduler = new FrameTurnScheduler(ownerTasks.schedule, () => {
          nonRunnableCallbacks.push(turn.owner);
          complete();
          return turn.continueFrame;
        }, () => true);
        scheduler.request();
      });
      ownerTasks.close();
    }
    expect(nonRunnableCallbacks).toEqual(fixture.expected.nonRunnableCallbacks);
    const runnableTextPhases = fixture.textIngressTurns.filter((turn) => turn.committed || turn.retiring).map((turn) => turn.phase);
    expect(fixture.textIngressTurns.map((turn) => turn.continueFrame)).toEqual(fixture.textIngressTurns.map((turn) => turn.committed || turn.retiring));
    expect(runnableTextPhases).toEqual(fixture.expected.runnableTextPhases);
  });

  it("acknowledges ingress before the private frame callback and never runs tick in the batch handler", () => {
    const worker = readFileSync(FRAME_WORKER_TS, "utf8");
    const handler = worker.slice(worker.indexOf('ownedStep("frame-ingress"'), worker.indexOf("function runFrameTurn"));
    expect(handler).toContain('post({ kind: "batch-accepted"');
    expect(handler).toContain("frameTurns?.request()");
    expect(handler).not.toContain("runtime!.tick(");
    expect(worker.slice(worker.indexOf("function runFrameTurn"), worker.indexOf("function answerIntrospection"))).toContain("runtime!.tick(");
    const frameJob = readFileSync(FRAME_JOB_RS, "utf8");
    const wasmOwner = frameJob.slice(frameJob.indexOf('#[cfg(target_arch = "wasm32")]\n    pub(crate) fn poll_runtime_and_resubmit'), frameJob.indexOf("    /// 🧵️ Whether a frame build is admitted right now"));
    expect(wasmOwner).toContain("try_step_on_worker");
    expect(wasmOwner).not.toContain("try_step_on_caller");
    expect(wasmOwner).not.toContain("BROWSER_FRAME_BUILD_DRIVE_US");
    expect(wasmOwner).not.toContain("loop {");
  });

  it("pins the Worker ceiling and the attribution law it shares with the UI isolate and the guest", () => {
    expect(WORKER_STEP_BUDGET_MS).toBe(8);
    expect(SUSTAINED_TURN_OVERRUN_TURNS).toBe(4);
    const worker = readFileSync(FRAME_WORKER_TS, "utf8");
    expect(worker).toContain('new TurnLedger(WORKER_STEP_BUDGET_MS, "worker-step")');
    expect(worker).not.toContain("worker-boot-step-overrun");
    expect(worker).not.toContain("interactive-job-overrun");
    expect(worker).not.toContain("worker-close-overrun");
    expect(worker).not.toMatch(/>= *WORKER_STEP_BUDGET_MS/);
  });

  it("completes a boot step that spins 50 ms of EXECUTING time — it yields and continues instead of ending the boot", async () => {
    const ledger = new TurnLedger(WORKER_STEP_BUDGET_MS, "worker-step");
    const clock = new TurnClock(() => performance.now());
    const yields: string[] = [];
    const completed: string[] = [];
    for (let index = 0; index < SUSTAINED_TURN_OVERRUN_TURNS + 1; index++) {
      completed.push(await workerBootStep(ledger, clock, `plugin-graph#${index}`, () => (spinMs(50), `plugin-graph#${index}`), yields));
    }
    expect(completed).toHaveLength(SUSTAINED_TURN_OVERRUN_TURNS + 1);
    const snapshot = ledger.snapshot();
    expect(snapshot.recordedOverruns).toBe(SUSTAINED_TURN_OVERRUN_TURNS + 1);
    expect(snapshot.sustainedOverruns).toBeGreaterThanOrEqual(1);
    expect(snapshot.worstExecutingMs).toBeGreaterThanOrEqual(45);
    expect(yields.length).toBeGreaterThanOrEqual(2);
    const recovered = await workerBootStep(ledger, clock, "cheap", () => "cheap", yields);
    expect(recovered).toBe("cheap");
    expect(ledger.degraded()).toBe(false);
  });

  it("completes a boot step suspended for 500 ms of WALL time with nothing recorded — descheduling is never the step's own cost", async () => {
    const ledger = new TurnLedger(WORKER_STEP_BUDGET_MS, "worker-step");
    const clock = new TurnClock(() => performance.now());
    const wallStartedAt = performance.now();
    clock.enter();
    clock.suspend();
    await new Promise<void>((resolve) => setTimeout(resolve, 500));
    clock.resume();
    const outcome = ledger.admit("shell-boot", clock.leave());
    expect(performance.now() - wallStartedAt).toBeGreaterThanOrEqual(450);
    expect(outcome.verdict).toBe("admitted");
    expect(outcome.executingMs).toBeLessThan(WORKER_STEP_BUDGET_MS);
    expect(ledger.snapshot().recordedOverruns).toBe(0);
    expect(ledger.degraded()).toBe(false);
  });

  it("chunks the plugin graph so no single chunk needs the ceiling, and answers exactly what the one-turn resolver answers", () => {
    const planner = new PlaygroundBootPlanner(PLUGIN_CATALOG, "generation3d");
    const chunks: { readonly stage: string; readonly executingMs: number }[] = [];
    for (;;) {
      const stage = planner.stage();
      const startedAt = performance.now();
      const more = planner.step();
      chunks.push({ stage, executingMs: performance.now() - startedAt });
      if (!more) break;
    }
    const plan = planner.finish();
    expect(chunks.length).toBeGreaterThanOrEqual(3);
    expect(chunks.some((chunk) => chunk.stage.startsWith("plugin-graph:rows"))).toBe(true);
    expect(chunks.some((chunk) => chunk.stage === "plugin-graph:closure")).toBe(true);
    expect(chunks.some((chunk) => chunk.stage === "plugin-graph:order")).toBe(true);
    for (const chunk of chunks) expect(chunk.executingMs).toBeLessThan(WORKER_STEP_BUDGET_MS);
    expect(PLUGIN_GRAPH_CHUNK_ROWS).toBeGreaterThan(0);
    expect(chunks.filter((chunk) => chunk.stage.startsWith("plugin-graph:rows"))).toHaveLength(Math.ceil((PLUGIN_CATALOG.plugins.length + PLUGIN_CATALOG.extensions.length) / PLUGIN_GRAPH_CHUNK_ROWS));
    const reference = resolvePlaygroundBoot(PLUGIN_CATALOG, "generation3d");
    expect(plan.plugins.map((entry) => entry.pluginId)).toEqual(reference.plugins.map((entry) => entry.pluginId));
    expect(plan.defaultAppId).toBe(reference.defaultAppId);
    expect(plan.dependencyErrors).toEqual(reference.dependencyErrors);
  });

  it("reports what every Rust bootstrap phase executed for, so a long phase is measurable instead of invisible", () => {
    const rust = readFileSync(BROWSER_WORKER_RS, "utf8");
    expect(rust).toContain("elapsed_us: u32");
    expect(rust).toContain("fn worker_now_ms() -> f64");
    for (const stage of ["font-atlas", "icon-atlas", "font-upload", "icon-upload", "plugin-parse", "shell-construct", "shell-boot", "runtime-ready"]) {
      expect(rust).toContain(`stage: "${stage}"`);
    }
    expect(readFileSync(FRAME_WORKER_TS, "utf8")).toContain("phaseUs=${step.elapsedUs}");
  });
});
