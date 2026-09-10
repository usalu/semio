//#region 🫀️BootLivenessLaw
/** @emoji 🫀️ The wgpu browser-boot liveness law, declared once for BOTH browser isolates.
 *
 * 🩸️ What this replaces: a lone `setTimeout(FRAME_WORKER_BOOT_STALL_TIMEOUT_MS)` in the UI isolate whose
 * only proof of life was a message arriving from the frame Worker. That predicate is FALSE for exactly the
 * phases that take the longest. The Worker's liveness ticker is a `setInterval` on the Worker's own event
 * loop, so it cannot fire while that loop is blocked — and a `WebAssembly` compile of the 76 MB renderer,
 * the synchronous prologue of `semioWgpuWorkerBootstrap`, or one Rust bootstrap phase all block it by
 * construction. A healthy Worker doing the single most expensive thing the boot asks of it therefore reads
 * exactly like a dead one, and the watchdog terminated it — taking the transferred `OffscreenCanvas`, the
 * surface's only frame path, with it (`worker-boot-timeout: Worker reported no boot progress for 60000 ms`
 * at `renderer-runtime 65 %`, boot #5).
 *
 * The law here separates the two states the old rule conflated. A Worker DECLARES a long phase before it
 * enters it — the declaration is posted while the loop still runs, so it always arrives — and the watchdog
 * then measures that phase against its own schema-owned ceiling instead of against silence. Silence only
 * terminates a Worker with NO phase in flight, which is the one case that really is a wedged event loop.
 * This is the shape `🎭️actor/📮️shard-client/🟦️.ts` ratified for shards (`evaluateShardLiveness`'s
 * `firstTurnTimeoutMs` half + `describeShardSilence`); this module is its frame-Worker twin. */

/** @emoji 🫀️ THE wgpu boot liveness policy — one record every boot clock in the two browser isolates
 * reads, so a value can never drift between the UI-isolate watchdog (`../🚚️browser-frame-transport/🟦️.ts`)
 * and the Worker's own phase declarations and liveness ticker (`../🎞️frame-worker/🟦️.ts`).
 *
 * `silenceTimeoutMs` is the bound on a Worker with NOTHING declared: it has neither spoken nor named a long
 * phase, so its loop is wedged. `livenessIntervalMs` is the cadence the Worker re-proves liveness at while
 * its loop DOES run. `phaseCeilingMs` prices the declared phases, keyed by full phase name first and by the
 * family before the first `:` second, so `plugin:generation3d` inherits `plugin`. `defaultPhaseCeilingMs`
 * catches a phase no row names, which must still be bounded — an undeclared ceiling is not a licence. */
export const FRAME_WORKER_BOOT_LIVENESS_POLICY = Object.freeze({
  silenceTimeoutMs: 60_000,
  livenessIntervalMs: 1_000,
  defaultPhaseCeilingMs: 300_000,
  phaseCeilingMs: Object.freeze({
    "renderer-module": 300_000,
    "wasm-artifact": 60_000,
    "wasm-cache-read": 120_000,
    "wasm-compile": 900_000,
    "wasm-instantiate": 300_000,
    "plugin": 300_000,
    "gpu-platform": 900_000,
    "shell-boot": 900_000,
    "renderer-bootstrap": 900_000,
  } as Readonly<Record<string, number>>),
});

/** @emoji 🧭️ One long phase a Worker has declared and not yet left. */
export type BrowserBootPhase = {
  readonly phase: string;
  readonly ceilingMs: number;
  readonly enteredAtMs: number;
};

/** @emoji 📏️ The ceiling {@link FRAME_WORKER_BOOT_LIVENESS_POLICY} prices one phase name against. */
export function bootPhaseCeilingMs(phase: string): number {
  const table = FRAME_WORKER_BOOT_LIVENESS_POLICY.phaseCeilingMs;
  const family = phase.slice(0, phase.indexOf(":") < 0 ? phase.length : phase.indexOf(":"));
  return table[phase] ?? table[family] ?? FRAME_WORKER_BOOT_LIVENESS_POLICY.defaultPhaseCeilingMs;
}

/** @emoji 🫀️ One watchdog window's worth of input — every field the rule below reads and nothing else, so
 * the same decision replays from a JSON timeline with no transport in the picture. `lastLivenessAtMs` is
 * `Number.NEGATIVE_INFINITY` for a Worker that has never sent a single message. */
export type BrowserBootLivenessWindow = {
  readonly nowMs: number;
  readonly lastLivenessAtMs: number;
  readonly silenceTimeoutMs: number;
  readonly phase: BrowserBootPhase | undefined;
};

/** @emoji ⚖️ What one window decided, plus the two measurements a fault card must quote. `rearmInMs` is
 * always the exact remaining time, so a re-armed watchdog fires at the deadline rather than one whole
 * window past it. */
export type BrowserBootLivenessDecision = {
  readonly terminate: boolean;
  readonly rearmInMs: number;
  readonly silentForMs: number;
  readonly phaseElapsedMs: number;
};

/** @emoji ⚖️ THE rule. A declared phase is measured against its OWN ceiling and silence cannot touch it —
 * a Worker blocked inside a browser-owned compile is busy, not dead, and the declaration is the evidence.
 * With no phase in flight, silence past `silenceTimeoutMs` is a wedged event loop and terminates. */
export function evaluateBrowserBootLiveness(window: BrowserBootLivenessWindow): BrowserBootLivenessDecision {
  const silentForMs = Number.isFinite(window.lastLivenessAtMs) ? Math.max(0, window.nowMs - window.lastLivenessAtMs) : Number.POSITIVE_INFINITY;
  if (window.phase) {
    const phaseElapsedMs = Math.max(0, window.nowMs - window.phase.enteredAtMs);
    const remainingMs = window.phase.ceilingMs - phaseElapsedMs;
    if (remainingMs > 0) return { terminate: false, rearmInMs: Math.min(window.silenceTimeoutMs, remainingMs), silentForMs, phaseElapsedMs };
    return { terminate: true, rearmInMs: 0, silentForMs, phaseElapsedMs };
  }
  const remainingMs = window.silenceTimeoutMs - silentForMs;
  if (remainingMs > 0) return { terminate: false, rearmInMs: remainingMs, silentForMs, phaseElapsedMs: 0 };
  return { terminate: true, rearmInMs: 0, silentForMs, phaseElapsedMs: 0 };
}
//#endregion 🫀️BootLivenessLaw

//#region 🩺️BootSilenceDiagnosis
/** @emoji 🩺️ What the frame Worker was doing when the watchdog gave up on it. Every field is state the
 * transport already holds; the point is that `Worker reported no boot progress for 60000 ms` names neither
 * the phase, nor its ceiling, nor how far past it the phase actually ran — which is precisely how a
 * host-side kill reaches a fault banner looking like a spontaneous, causeless Worker death. */
export type BrowserBootSilenceReport = {
  readonly heard: boolean;
  readonly lastStage: string;
  readonly silentForMs: number;
  readonly silenceTimeoutMs: number;
  readonly phase: BrowserBootPhase | undefined;
  readonly phaseElapsedMs: number;
};

function roundedMs(value: number): string {
  return Number.isFinite(value) ? String(Math.max(0, Math.round(value))) : "∞";
}

/** @emoji 🩺️ Composes {@link BrowserBootSilenceReport} into the one line the `worker-boot-timeout` fault
 * carries, in the reader's tongue. A declared phase that blew its ceiling names the phase, its elapsed and
 * its ceiling; an undeclared silence says so explicitly, because those are different defects with
 * different owners — the first is a phase whose ceiling is wrong or whose work truly wedged, the second is
 * a Worker whose event loop never came back. */
export function describeBrowserBootSilence(report: BrowserBootSilenceReport, tongue: "en" | "de"): string {
  const stage = report.lastStage || (tongue === "de" ? "—" : "—");
  if (report.phase) {
    return tongue === "de"
      ? `Die erklärte lange Phase „${report.phase.phase}“ des Frame-Workers lief ${roundedMs(report.phaseElapsedMs)} ms gegen ihre Obergrenze von ${roundedMs(report.phase.ceilingMs)} ms (letzte gemeldete Stufe „${stage}“, still seit ${roundedMs(report.silentForMs)} ms).`
      : `The frame Worker's declared long phase "${report.phase.phase}" ran ${roundedMs(report.phaseElapsedMs)} ms against its ${roundedMs(report.phase.ceilingMs)} ms ceiling (last reported stage "${stage}", silent for ${roundedMs(report.silentForMs)} ms).`;
  }
  const silence = report.heard
    ? tongue === "de"
      ? `war ${roundedMs(report.silentForMs)} ms still`
      : `was silent for ${roundedMs(report.silentForMs)} ms`
    : tongue === "de"
      ? "hat nie eine einzige Nachricht gesendet"
      : "never sent a single message";
  return tongue === "de"
    ? `Der Frame-Worker ${silence} (Obergrenze ${roundedMs(report.silenceTimeoutMs)} ms, letzte gemeldete Stufe „${stage}“) und hatte keine lange Phase erklärt — seine Ereignisschleife hängt.`
    : `The frame Worker ${silence} (ceiling ${roundedMs(report.silenceTimeoutMs)} ms, last reported stage "${stage}") and had declared no long phase — its event loop is wedged.`;
}

/** @emoji 🧭️ The declared-phase line the fallback panel renders beside the fault, so a reader sees WHICH
 * long phase was in flight and for how long even when the fault came from somewhere else entirely. */
export function describeBrowserBootPhase(phase: BrowserBootPhase | undefined, elapsedMs: number, tongue: "en" | "de"): string {
  if (!phase) return tongue === "de" ? "Lange Boot-Phase: keine erklärt" : "Long boot phase: none declared";
  return tongue === "de"
    ? `Lange Boot-Phase: „${phase.phase}“ seit ${roundedMs(elapsedMs)} ms (Obergrenze ${roundedMs(phase.ceilingMs)} ms)`
    : `Long boot phase: "${phase.phase}" for ${roundedMs(elapsedMs)} ms (ceiling ${roundedMs(phase.ceilingMs)} ms)`;
}
//#endregion 🩺️BootSilenceDiagnosis
