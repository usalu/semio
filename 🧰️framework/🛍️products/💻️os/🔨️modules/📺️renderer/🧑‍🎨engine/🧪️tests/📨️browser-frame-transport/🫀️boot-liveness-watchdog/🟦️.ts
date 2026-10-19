/** 🫀️ Boot liveness watchdog: the virtual clock harness, the re-arm, the phase ceilings, the silence and heartbeat laws and the third-party clock replay. */
import { fixtureOriginalWorkStock } from "@semio-tech/framework-actor/retained-turn/bootstrap/testing/fixtures";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import { BrowserFrameTransport } from "@semio-tech/framework-renderer-wgpu/browser-frame-transport";
import { type BrowserFrameFallbackState } from "@semio-tech/framework-renderer-wgpu/browser-frame-transport/contract";
import { FRAME_WORKER_BOOT_LIVENESS_POLICY, bootPhaseCeilingMs, describeBrowserBootPhase, describeBrowserBootSilence, evaluateBrowserBootLiveness } from "@semio-tech/framework-renderer-wgpu/boot-liveness";
import { FakeWorker, TEST_HOST_APPEARANCE, TEST_HOST_PLATFORM, TEST_HOST_STORAGE, testBootDescriptor } from "../🧰️transport-test-support/🟦️.ts";
import { readFrameWorkerSource } from "../📖️frame-worker-source/🟦️.ts";

type VirtualTimer = { readonly id: number; readonly callback: () => void; readonly dueAtMs: number };

/** ⏱️ A virtual clock + timer queue, so a 120 s boot is decided in microseconds and the watchdog's
 * verdict is a function of the timeline rather than of how loaded the machine running the suite is. */
function bootHarness(tongue: "en" | "de" = "en") {
  const worker = new FakeWorker();
  const timers = new Map<number, VirtualTimer>();
  const faults: { readonly code: string; readonly detail: string; readonly fallback: BrowserFrameFallbackState }[] = [];
  let nextTimerId = 1;
  let nowMs = 0;
  const subject = new BrowserFrameTransport({ originalWorkStock: fixtureOriginalWorkStock(),
    worker,
    boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer_bg.wasm", canvas: {} as OffscreenCanvas, width: 8, height: 8, dpr: 1, locale: tongue, descriptor: testBootDescriptor("generation3d"), appearance: TEST_HOST_APPEARANCE, platform: TEST_HOST_PLATFORM, storage: TEST_HOST_STORAGE, catalog: { version: 1 as const, targets: [], hosts: [], playgrounds: [] } },
    now: () => nowMs,
    setTimer: (callback, delayMs) => {
      const id = nextTimerId++;
      timers.set(id, { id, callback, dueAtMs: nowMs + delayMs });
      return id;
    },
    clearTimer: (handle) => void timers.delete(handle),
    onFault: (code, detail, fallback) => faults.push({ code, detail, fallback }),
  });
  const advance = (deltaMs: number): void => {
    const targetMs = nowMs + deltaMs;
    for (;;) {
      let due: VirtualTimer | undefined;
      for (const timer of timers.values()) if (timer.dueAtMs <= targetMs && (due === undefined || timer.dueAtMs < due.dueAtMs)) due = timer;
      if (due === undefined) break;
      timers.delete(due.id);
      nowMs = Math.max(nowMs, due.dueAtMs);
      due.callback();
    }
    nowMs = targetMs;
  };
  return { advance, faults, subject, worker, now: () => nowMs };
}

describe("wgpu boot liveness watchdog", () => {
  it("re-arms for the exact remaining window and never terminates a declared phase inside its ceiling", () => {
    const silenceTimeoutMs = FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs;
    expect(evaluateBrowserBootLiveness({ nowMs: 40_000, lastLivenessAtMs: 10_000, silenceTimeoutMs, phase: undefined })).toEqual({ terminate: false, rearmInMs: 30_000, silentForMs: 30_000, phaseElapsedMs: 0 });
    expect(evaluateBrowserBootLiveness({ nowMs: 70_000, lastLivenessAtMs: 10_000, silenceTimeoutMs, phase: undefined }).terminate).toBe(true);
    expect(evaluateBrowserBootLiveness({ nowMs: 0, lastLivenessAtMs: Number.NEGATIVE_INFINITY, silenceTimeoutMs, phase: undefined }).silentForMs).toBe(Number.POSITIVE_INFINITY);
    const phase = { phase: "gpu-platform", ceilingMs: bootPhaseCeilingMs("gpu-platform"), enteredAtMs: 1_000 };
    const busy = evaluateBrowserBootLiveness({ nowMs: 601_000, lastLivenessAtMs: 1_000, silenceTimeoutMs, phase });
    expect(busy.terminate).toBe(false);
    expect(busy.phaseElapsedMs).toBe(600_000);
    expect(busy.rearmInMs).toBe(Math.min(silenceTimeoutMs, phase.ceilingMs - 600_000));
    expect(evaluateBrowserBootLiveness({ nowMs: 1_000 + phase.ceilingMs, lastLivenessAtMs: 1_000, silenceTimeoutMs, phase }).terminate).toBe(true);
  });

  it("prices a phase family by the segment before its colon and bounds every unnamed phase", () => {
    expect(bootPhaseCeilingMs("plugin:generation3d")).toBe(FRAME_WORKER_BOOT_LIVENESS_POLICY.phaseCeilingMs["plugin"]);
    expect(bootPhaseCeilingMs("shell-boot")).toBe(FRAME_WORKER_BOOT_LIVENESS_POLICY.phaseCeilingMs["shell-boot"]);
    expect(bootPhaseCeilingMs("a-phase-nobody-declared")).toBe(FRAME_WORKER_BOOT_LIVENESS_POLICY.defaultPhaseCeilingMs);
    expect(bootPhaseCeilingMs("a-phase-nobody-declared")).toBeGreaterThan(0);
  });

  it("keeps a Worker that declared a long phase alive across 120 s of total wall silence", () => {
    const { advance, faults, subject, worker } = bootHarness();
    worker.reply({ kind: "boot-progress", lifecycle: 1, stage: "renderer-runtime", progress: 0.65, worker: { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" } });
    worker.reply({ kind: "boot-phase", lifecycle: 1, phase: "gpu-platform", state: "enter" });
    advance(120_000);
    expect(faults).toEqual([]);
    expect(subject.status).toBe("booting");
    expect(worker.terminated).toBe(false);
    expect(subject.fallbackState().bootPhase?.phase).toBe("gpu-platform");
    expect(subject.fallbackState().bootPhaseElapsedMs).toBe(120_000);
    worker.reply({ kind: "boot-phase", lifecycle: 1, phase: "gpu-platform", state: "leave", elapsedMs: 120_000 });
    worker.reply({ kind: "booted", lifecycle: 1 });
    expect(subject.status).toBe("ready");
  });

  it("keeps a heartbeating Worker alive indefinitely and still terminates one that goes silent with no phase declared", () => {
    const { advance, faults, subject, worker } = bootHarness();
    worker.reply({ kind: "boot-progress", lifecycle: 1, stage: "plugin:generation3d", progress: 0.4, worker: { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" } });
    for (let beat = 0; beat < 240; beat++) {
      advance(FRAME_WORKER_BOOT_LIVENESS_POLICY.livenessIntervalMs);
      worker.reply({ kind: "boot-liveness", lifecycle: 1 });
    }
    expect(faults).toEqual([]);
    expect(subject.status).toBe("booting");
    advance(FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs + 1);
    expect(subject.status).toBe("faulted");
    expect(faults).toHaveLength(1);
    expect(faults[0]?.code).toBe("worker-boot-timeout");
    expect(faults[0]?.detail).toContain("declared no long phase");
    expect(faults[0]?.detail).toContain("plugin:generation3d");
    expect(worker.messages.some((message) => message.kind === "close")).toBe(true);
  });

  it("terminates a declared phase only once it blows its OWN ceiling, naming the phase, its elapsed and that ceiling", () => {
    const { advance, faults, subject, worker } = bootHarness();
    worker.reply({ kind: "boot-progress", lifecycle: 1, stage: "renderer-runtime", progress: 0.65, worker: { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" } });
    worker.reply({ kind: "boot-phase", lifecycle: 1, phase: "gpu-platform", state: "enter" });
    const ceilingMs = bootPhaseCeilingMs("gpu-platform");
    advance(ceilingMs - 1);
    expect(subject.status).toBe("booting");
    advance(2);
    expect(subject.status).toBe("faulted");
    expect(faults[0]?.code).toBe("worker-boot-timeout");
    expect(faults[0]?.detail).toContain('"gpu-platform"');
    expect(faults[0]?.detail).toContain(`${ceilingMs} ms ceiling`);
    expect(faults[0]?.detail).toContain('last reported stage "renderer-runtime"');
    expect(faults[0]?.fallback.bootPhase?.phase).toBe("gpu-platform");
    expect(faults[0]?.fallback.bootStage).toBe("renderer-runtime");
  });

  it("names a Worker that never sent a single message, and says the same thing in German", () => {
    const { advance, faults, subject } = bootHarness("de");
    advance(FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs + 1);
    expect(subject.status).toBe("faulted");
    expect(faults[0]?.detail).toContain("hat nie eine einzige Nachricht gesendet");
    expect(faults[0]?.detail).toContain("Ereignisschleife hängt");
    const report = { heard: true, lastStage: "renderer-runtime", silentForMs: 61_000, silenceTimeoutMs: 60_000, phase: { phase: "wasm-compile", ceilingMs: 900_000, enteredAtMs: 0 }, phaseElapsedMs: 901_000 };
    expect(describeBrowserBootSilence(report, "en")).toBe('The frame Worker\'s declared long phase "wasm-compile" ran 901000 ms against its 900000 ms ceiling (last reported stage "renderer-runtime", silent for 61000 ms).');
    expect(describeBrowserBootSilence(report, "de")).toContain("Die erklärte lange Phase „wasm-compile“");
    expect(describeBrowserBootPhase(report.phase, 901_000, "en")).toBe('Long boot phase: "wasm-compile" for 901000 ms (ceiling 900000 ms)');
    expect(describeBrowserBootPhase(undefined, 0, "de")).toBe("Lange Boot-Phase: keine erklärt");
  });

  it("declares every browser-owned phase BEFORE it blocks, and brings the renderer wasm up in cached, reporting phases", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const workerSource = readFrameWorkerSource(root);
    const bootSource = (readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), "utf8") + "\n" + readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts"), "utf8"));
    const suspension = workerSource.slice(workerSource.indexOf("async function monitoredSuspension"), workerSource.indexOf("async function macrotask"));
    expect(suspension.indexOf('declarePhase(stage, "enter", 0)')).toBeGreaterThan(-1);
    expect(suspension.indexOf('declarePhase(stage, "enter", 0)')).toBeLessThan(suspension.indexOf("stepClock.suspend()"));
    expect(suspension).toContain('declarePhase(stage, "leave"');
    expect(workerSource).toContain('declaredStep("renderer-bootstrap"');
    expect(workerSource).toContain("WebAssembly.compileStreaming");
    expect(workerSource).toContain("readCachedRendererModule");
    expect(workerSource).toContain("writeCachedRendererModule");
    expect(workerSource).not.toContain('monitoredSuspension("wasm-instance"');
    expect(bootSource).toContain("describeBrowserBootPhase(state.bootPhase, state.bootPhaseElapsedMs, tongue)");
    expect(workerSource).toContain("if (!bootDeclarationsOpen || closed || closing || failed) return;");
    expect(workerSource.indexOf("setBootDeclarationsOpen(false);")).toBeLessThan(workerSource.indexOf('post({ kind: "booted", lifecycle })'));
  });
});

describe("wgpu boot liveness watchdog, replayed on a third-party clock", () => {
  /** 🧪️ The same 120 s timeline decided by vitest's own fake timers (`@sinonjs/fake-timers`) driving
   * the real `setTimeout`/`clearTimeout`, instead of this file's hand-rolled queue — an INDEPENDENT clock
   * implementation reaching the same verdicts, so a bug in the harness cannot pass for a passing law. */
  it("survives a declared 120 s phase and dies on 60 s of undeclared silence under vitest's fake timers", () => {
    vi.useFakeTimers();
    try {
      const worker = new FakeWorker();
      const faults: string[] = [];
      const subject = new BrowserFrameTransport({ originalWorkStock: fixtureOriginalWorkStock(),
        worker,
        boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer_bg.wasm", canvas: {} as OffscreenCanvas, width: 8, height: 8, dpr: 1, locale: "en", descriptor: testBootDescriptor("generation3d"), appearance: TEST_HOST_APPEARANCE, platform: TEST_HOST_PLATFORM, storage: TEST_HOST_STORAGE, catalog: { version: 1 as const, targets: [], hosts: [], playgrounds: [] } },
        now: () => Date.now(),
        setTimer: (callback, delayMs) => setTimeout(callback, delayMs) as unknown as number,
        clearTimer: (handle) => clearTimeout(handle),
        onFault: (code, detail) => faults.push(`${code}: ${detail}`),
      });
      worker.reply({ kind: "boot-progress", lifecycle: 1, stage: "renderer-runtime", progress: 0.65, worker: { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" } });
      worker.reply({ kind: "boot-phase", lifecycle: 1, phase: "gpu-platform", state: "enter" });
      vi.advanceTimersByTime(120_000);
      expect(faults).toEqual([]);
      expect(subject.status).toBe("booting");
      worker.reply({ kind: "boot-phase", lifecycle: 1, phase: "gpu-platform", state: "leave", elapsedMs: 120_000 });
      vi.advanceTimersByTime(FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs + 1);
      expect(subject.status).toBe("faulted");
      expect(faults[0]).toContain("worker-boot-timeout");
      expect(faults[0]).toContain("declared no long phase");
    } finally {
      vi.useRealTimers();
    }
  });
});
