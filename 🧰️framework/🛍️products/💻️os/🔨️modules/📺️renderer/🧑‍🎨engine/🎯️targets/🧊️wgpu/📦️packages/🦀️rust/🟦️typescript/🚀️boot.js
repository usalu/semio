/* 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⏱️turn-budget/🟦️.ts */
var UI_TURN_BUDGET_MS = 2;
var SUSTAINED_TURN_OVERRUN_TURNS = 4;
var TURN_SAMPLE_CAPACITY = 64;
var TURN_DIAGNOSTICS_KEY = "SEMIO_RUNTIME_DIAGNOSTICS";
var diagnosticsOverride;
var diagnosticsResolved;
function diagnosticsArmed(value) {
  return typeof value === "string" && ["1", "true", "on", "yes"].includes(value.trim().toLowerCase());
}
function setTurnDiagnostics(enabled) {
  diagnosticsOverride = enabled;
  diagnosticsResolved = undefined;
}
function turnDiagnosticsEnabled() {
  if (diagnosticsOverride !== undefined)
    return diagnosticsOverride;
  if (diagnosticsResolved !== undefined)
    return diagnosticsResolved;
  let armed = false;
  try {
    armed = diagnosticsArmed(import.meta.env?.[`VITE_${TURN_DIAGNOSTICS_KEY}`]);
  } catch {
    armed = false;
  }
  diagnosticsResolved = armed;
  return armed;
}

class TurnClock {
  now;
  depth = 0;
  spanStartedAt = 0;
  charged = 0;
  clockLost = false;
  constructor(now) {
    this.now = now;
  }
  enter() {
    if (this.depth === 0) {
      this.charged = 0;
      this.clockLost = false;
      this.spanStartedAt = this.reading();
    } else {
      this.chargeSpan();
    }
    this.depth++;
  }
  suspend() {
    if (this.depth === 0)
      return;
    this.chargeSpan();
  }
  resume() {
    if (this.depth === 0)
      return;
    this.spanStartedAt = this.reading();
  }
  leave() {
    if (this.depth === 0)
      return;
    this.depth--;
    if (this.depth > 0)
      return;
    this.chargeSpan();
    return this.clockLost ? undefined : this.charged;
  }
  chargeSpan() {
    const now = this.reading();
    const started = this.spanStartedAt;
    if (!Number.isFinite(now) || !Number.isFinite(started) || now < started) {
      this.clockLost = true;
      return;
    }
    this.charged += now - started;
    this.spanStartedAt = now;
  }
  reading() {
    try {
      const value = this.now();
      return typeof value === "number" ? value : Number.NaN;
    } catch {
      return Number.NaN;
    }
  }
}

class TurnLedger {
  budgetMs;
  scope;
  samples = new Float64Array(TURN_SAMPLE_CAPACITY);
  sampleCount = 0;
  consecutive = 0;
  longestRun = 0;
  recorded = 0;
  sustained = 0;
  worstExecutingMs = 0;
  worstSite = "";
  degradedUntilAdmitted = false;
  constructor(budgetMs = UI_TURN_BUDGET_MS, scope = "ui-turn") {
    this.budgetMs = budgetMs;
    this.scope = scope;
  }
  admit(site, executingMs) {
    if (executingMs === undefined || !Number.isFinite(executingMs) || executingMs < 0) {
      this.consecutive = 0;
      return { site, verdict: "clock-fault", executingMs: 0, consecutive: 0 };
    }
    this.samples[this.sampleCount % TURN_SAMPLE_CAPACITY] = executingMs;
    this.sampleCount++;
    if (executingMs < this.budgetMs) {
      this.consecutive = 0;
      this.degradedUntilAdmitted = false;
      return { site, verdict: "admitted", executingMs, consecutive: 0 };
    }
    this.consecutive++;
    this.longestRun = Math.max(this.longestRun, this.consecutive);
    this.recorded++;
    if (executingMs > this.worstExecutingMs) {
      this.worstExecutingMs = executingMs;
      this.worstSite = site;
    }
    if (this.consecutive < SUSTAINED_TURN_OVERRUN_TURNS) {
      this.trace(site, "recorded-overrun", executingMs);
      return { site, verdict: "recorded-overrun", executingMs, consecutive: this.consecutive };
    }
    this.sustained++;
    this.degradedUntilAdmitted = true;
    this.trace(site, "sustained-overrun", executingMs);
    return { site, verdict: "sustained-overrun", executingMs, consecutive: this.consecutive };
  }
  degraded() {
    return this.degradedUntilAdmitted;
  }
  snapshot() {
    return {
      recordedOverruns: this.recorded,
      sustainedOverruns: this.sustained,
      consecutive: this.consecutive,
      longestRun: this.longestRun,
      worstExecutingMs: this.worstExecutingMs,
      worstSite: this.worstSite,
      degraded: this.degradedUntilAdmitted,
      p99Ms: this.p99Ms()
    };
  }
  p99Ms() {
    const count = Math.min(this.sampleCount, TURN_SAMPLE_CAPACITY);
    if (count === 0)
      return 0;
    const ordered = Array.from(this.samples.subarray(0, count)).sort((left, right) => left - right);
    return ordered[Math.min(count - 1, Math.ceil(count * 0.99) - 1)];
  }
  trace(site, verdict, executingMs) {
    if (!turnDiagnosticsEnabled())
      return;
    console.debug(`[DEBUG] ${this.scope} ${verdict} site=${site} executing=${executingMs.toFixed(3)}ms budget=${this.budgetMs}ms consecutive=${this.consecutive}/${SUSTAINED_TURN_OVERRUN_TURNS}`);
  }
}

/* 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔌️browser-interactive-job-port/🟦️.ts */
var INTERACTIVE_JOB_SLOT_CAPACITY = 16;
var INTERACTIVE_JOB_INPUT_ITEM_CAPACITY = 65536;
var INTERACTIVE_JOB_INPUT_BYTE_CAPACITY = 256 * 1024 * 1024;
var INTERACTIVE_JOB_PAGE_ITEM_CAPACITY = 128;
var INTERACTIVE_JOB_PAGE_BYTE_CAPACITY = 16 * 1024;
var INTERACTIVE_JOB_OBSERVER_CAPACITY = 32;
var INTERACTIVE_JOB_PORT_ITEM_CAPACITY = 262144;
var INTERACTIVE_JOB_PORT_BYTE_CAPACITY = 256 * 1024 * 1024;

class BrowserInteractiveJobPort {
  lifecycle;
  send;
  quarantineConsumer;
  schedule;
  status = "unavailable";
  slots = new Array(INTERACTIVE_JOB_SLOT_CAPACITY);
  closeCursor = 0;
  closeScheduled = false;
  reservedItems = 0;
  reservedBytes = 0;
  observers = new Array(INTERACTIVE_JOB_OBSERVER_CAPACITY);
  observerCursor = 0;
  observerNotifyScheduled = false;
  statusRevision = 0;
  statusSnapshot = { status: "unavailable", revision: 0 };
  uiTurns = new TurnLedger;
  uiTurnClock;
  constructor(lifecycle, send, now, quarantineConsumer, schedule = (callback) => setTimeout(callback, 0)) {
    this.lifecycle = lifecycle;
    this.send = send;
    this.quarantineConsumer = quarantineConsumer;
    this.schedule = schedule;
    this.uiTurnClock = new TurnClock(now);
  }
  ready() {
    if (this.status === "unavailable") {
      this.status = "ready";
      this.publishStatus();
    }
  }
  getSnapshot() {
    return this.statusSnapshot;
  }
  observeConsumerTurn(site, durationMs) {
    return this.uiTurns.admit(site, durationMs).verdict !== "sustained-overrun";
  }
  reportConsumerFault(site, detail) {
    this.quarantine(`${site} threw: ${detail}`);
  }
  uiTurnSnapshot() {
    return this.uiTurns.snapshot();
  }
  subscribe(listener) {
    const slot = this.observers.findIndex((entry) => entry === undefined);
    if (slot < 0)
      throw new Error(`interactive job observer slots exceeded ${INTERACTIVE_JOB_OBSERVER_CAPACITY}`);
    this.observers[slot] = listener;
    return () => {
      this.observers[slot] = undefined;
    };
  }
  submit(descriptor, consumer) {
    if (this.status !== "ready" || descriptor.kind.length === 0 || descriptor.kind.length > 64)
      return;
    if (!admittedCount(descriptor.operation) || !admittedCount(descriptor.generation) || !admittedCount(descriptor.inputItems) || !admittedCount(descriptor.inputBytes) || !admittedCount(descriptor.outputItems) || !admittedCount(descriptor.outputBytes) || !admittedCount(descriptor.inputPageItems) || !admittedCount(descriptor.outputPageItems) || !admittedCount(descriptor.pageBytes))
      return;
    if (descriptor.inputItems > INTERACTIVE_JOB_INPUT_ITEM_CAPACITY || descriptor.inputBytes > INTERACTIVE_JOB_INPUT_BYTE_CAPACITY || descriptor.outputItems > INTERACTIVE_JOB_INPUT_ITEM_CAPACITY || descriptor.outputBytes > INTERACTIVE_JOB_INPUT_BYTE_CAPACITY)
      return;
    if (descriptor.inputPageItems > INTERACTIVE_JOB_PAGE_ITEM_CAPACITY || descriptor.outputPageItems > INTERACTIVE_JOB_PAGE_ITEM_CAPACITY || descriptor.pageBytes > INTERACTIVE_JOB_PAGE_BYTE_CAPACITY)
      return;
    const reservedItems = descriptor.inputItems + descriptor.outputItems;
    const reservedBytes = descriptor.inputBytes + descriptor.outputBytes;
    if (this.reservedItems + reservedItems > INTERACTIVE_JOB_PORT_ITEM_CAPACITY || this.reservedBytes + reservedBytes > INTERACTIVE_JOB_PORT_BYTE_CAPACITY)
      return;
    if (this.slots.some((slot) => slot?.descriptor.operation === descriptor.operation))
      return;
    const index = this.slots.findIndex((slot) => slot === undefined);
    if (index < 0)
      return;
    this.slots[index] = { descriptor, consumer, inputCursor: 0, inputItems: 0, inputBytes: 0, outputItems: 0, outputBytes: 0, closing: false };
    this.reservedItems += reservedItems;
    this.reservedBytes += reservedBytes;
    try {
      this.send({ kind: "job-submit", lifecycle: this.lifecycle, descriptor });
    } catch {
      this.slots[index] = undefined;
      this.reservedItems -= reservedItems;
      this.reservedBytes -= reservedBytes;
      return;
    }
    return { operation: descriptor.operation, generation: descriptor.generation, cancel: () => this.cancel(descriptor.operation, descriptor.generation) };
  }
  receive(message) {
    if (!message.kind.startsWith("job-"))
      return false;
    if (message.lifecycle !== this.lifecycle || this.status !== "ready")
      return true;
    if (!admittedCount(message.operation) || !admittedCount(message.generation)) {
      this.quarantine("interactive job message identity was invalid");
      return true;
    }
    const index = this.slots.findIndex((slot2) => slot2?.descriptor.operation === message.operation);
    if (index < 0)
      return true;
    const slot = this.slots[index];
    if (message.generation > slot.descriptor.generation) {
      this.quarantine(`interactive job returned future generation ${message.generation}`);
      return true;
    }
    if (message.generation < slot.descriptor.generation)
      return true;
    if (slot.closing)
      return true;
    if (message.kind === "job-input-pull") {
      if (!admittedCount(message.cursor) || message.cursor !== slot.inputCursor || !admittedCount(message.maxItems) || message.maxItems === 0 || message.maxItems > slot.descriptor.inputPageItems) {
        this.quarantine("interactive job pull exceeded fixed credits");
        return true;
      }
      this.uiTurnClock.enter();
      let page;
      try {
        page = slot.consumer.readInputPage(message.cursor, Math.min(message.maxItems, slot.descriptor.inputPageItems));
      } catch (error) {
        this.uiTurnClock.leave();
        this.quarantine(`input consumer threw: ${error instanceof Error ? error.message : String(error)}`);
        return true;
      }
      this.observe("input consumer");
      if (!this.admitPage(slot, page, true))
        return true;
      slot.inputCursor += page.itemCount;
      try {
        this.send({ kind: "job-input-page", lifecycle: this.lifecycle, operation: message.operation, generation: message.generation, cursor: message.cursor, page });
      } catch (error) {
        this.quarantine(`input page transfer threw: ${error instanceof Error ? error.message : String(error)}`);
      }
      return true;
    }
    if (message.kind === "job-output-page") {
      if (!this.admitPage(slot, message.page, false))
        return true;
      this.uiTurnClock.enter();
      try {
        slot.consumer.onOutputPage(message.page);
      } catch (error) {
        this.uiTurnClock.leave();
        this.quarantine(`output consumer threw: ${error instanceof Error ? error.message : String(error)}`);
        return true;
      }
      this.observe("output consumer");
      return true;
    }
    if (message.status !== "complete" && message.status !== "cancelled" && message.status !== "fault") {
      this.quarantine("interactive job returned invalid terminal status");
      return true;
    }
    const terminal = { operation: message.operation, generation: message.generation, status: message.status, ...message.detail === undefined ? {} : { detail: message.detail } };
    this.uiTurnClock.enter();
    try {
      slot.consumer.onTerminal(terminal);
    } catch (error) {
      this.uiTurnClock.leave();
      this.quarantine(`terminal consumer threw: ${error instanceof Error ? error.message : String(error)}`);
      slot.closing = true;
      this.scheduleClose();
      return true;
    }
    slot.closing = true;
    this.observe("terminal consumer");
    this.scheduleClose();
    return true;
  }
  close() {
    if (this.status === "closed")
      return;
    this.status = "closed";
    this.closeCursor = 0;
    for (let index = 0;index < this.slots.length; index++)
      if (this.slots[index])
        this.slots[index].closing = true;
    this.publishStatus();
    this.scheduleClose();
  }
  closeStep() {
    if (this.status !== "closed" && this.status !== "quarantined")
      return false;
    return this.drainClosingStep();
  }
  drainClosingStep() {
    while (this.closeCursor < this.slots.length && (!this.slots[this.closeCursor] || !this.slots[this.closeCursor].closing))
      this.closeCursor++;
    if (this.closeCursor === this.slots.length)
      return true;
    const slot = this.slots[this.closeCursor];
    this.uiTurnClock.enter();
    let complete = false;
    try {
      complete = slot.consumer.closeStep();
      if (complete)
        complete = slot.consumer.terminalIsEmpty();
    } catch (error) {
      this.uiTurnClock.leave();
      this.quarantine(`consumer close threw: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
    this.observe("consumer close");
    if (complete) {
      this.releaseSlot(this.closeCursor);
      this.closeCursor++;
    }
    return false;
  }
  quarantineFromOwner() {
    if (this.status === "closed")
      return;
    this.status = "quarantined";
    this.closeCursor = 0;
    for (let index = 0;index < this.slots.length; index++)
      if (this.slots[index])
        this.slots[index].closing = true;
    this.publishStatus();
    this.scheduleClose();
  }
  cancel(operation, generation) {
    if (this.status !== "ready")
      return false;
    const slot = this.slots.find((candidate) => candidate?.descriptor.operation === operation);
    if (!slot || slot.descriptor.generation !== generation)
      return false;
    try {
      this.send({ kind: "job-cancel", lifecycle: this.lifecycle, operation, generation });
    } catch (error) {
      this.quarantine(`cancel transfer threw: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
    return true;
  }
  admitPage(slot, page, input) {
    const pageItemLimit = input ? slot.descriptor.inputPageItems : slot.descriptor.outputPageItems;
    if (!admittedCount(page.itemCount) || !admittedCount(page.byteLength) || typeof page.complete !== "boolean" || page.itemCount === 0 && !page.complete || page.itemCount > pageItemLimit || page.byteLength > slot.descriptor.pageBytes) {
      this.quarantine("interactive job page exceeded fixed credits");
      return false;
    }
    const items = (input ? slot.inputItems : slot.outputItems) + page.itemCount;
    const bytes = (input ? slot.inputBytes : slot.outputBytes) + page.byteLength;
    const itemLimit = input ? slot.descriptor.inputItems : slot.descriptor.outputItems;
    const byteLimit = input ? slot.descriptor.inputBytes : slot.descriptor.outputBytes;
    if (items > itemLimit || bytes > byteLimit) {
      this.quarantine("interactive job aggregate credits exhausted");
      return false;
    }
    if (page.complete && items !== itemLimit || !page.complete && items >= itemLimit) {
      this.quarantine("interactive job page completion violated declared item credits");
      return false;
    }
    if (input) {
      slot.inputItems = items;
      slot.inputBytes = bytes;
    } else {
      slot.outputItems = items;
      slot.outputBytes = bytes;
    }
    return true;
  }
  observe(site) {
    return this.uiTurns.admit(site, this.uiTurnClock.leave()).verdict !== "sustained-overrun";
  }
  quarantine(detail) {
    if (this.status !== "ready")
      return;
    this.status = "quarantined";
    this.closeCursor = 0;
    for (let index = 0;index < this.slots.length; index++)
      if (this.slots[index])
        this.slots[index].closing = true;
    this.publishStatus();
    this.scheduleClose();
    this.quarantineConsumer(detail);
  }
  notifyObservers() {
    this.observerCursor = 0;
    if (this.observerNotifyScheduled)
      return;
    this.observerNotifyScheduled = true;
    this.schedule(() => this.notifyOneObserver());
  }
  publishStatus() {
    this.statusRevision += 1;
    this.statusSnapshot = { status: this.status, revision: this.statusRevision };
    this.notifyObservers();
  }
  notifyOneObserver() {
    this.observerNotifyScheduled = false;
    while (this.observerCursor < this.observers.length && !this.observers[this.observerCursor])
      this.observerCursor++;
    if (this.observerCursor === this.observers.length)
      return;
    const observer = this.observers[this.observerCursor++];
    this.uiTurnClock.enter();
    try {
      observer();
    } catch (error) {
      this.uiTurnClock.leave();
      this.quarantine(`status observer threw: ${error instanceof Error ? error.message : String(error)}`);
      return;
    }
    this.observe("status observer");
    this.observerNotifyScheduled = true;
    this.schedule(() => this.notifyOneObserver());
  }
  releaseSlot(index) {
    const slot = this.slots[index];
    if (!slot)
      return;
    this.reservedItems -= slot.descriptor.inputItems + slot.descriptor.outputItems;
    this.reservedBytes -= slot.descriptor.inputBytes + slot.descriptor.outputBytes;
    this.slots[index] = undefined;
  }
  scheduleClose() {
    if (this.closeScheduled)
      return;
    this.closeScheduled = true;
    this.schedule(() => {
      this.closeScheduled = false;
      this.closeCursor = 0;
      if (!this.drainClosingStep())
        this.scheduleClose();
    });
  }
}
function admittedCount(value) {
  return Number.isSafeInteger(value) && value >= 0;
}

/* 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🫀️boot-liveness/🟦️.ts */
var FRAME_WORKER_BOOT_LIVENESS_POLICY = Object.freeze({
  silenceTimeoutMs: 60000,
  livenessIntervalMs: 1000,
  defaultPhaseCeilingMs: 300000,
  phaseCeilingMs: Object.freeze({
    "renderer-module": 300000,
    "wasm-artifact": 60000,
    "wasm-cache-read": 120000,
    "wasm-compile": 900000,
    "wasm-instantiate": 300000,
    plugin: 300000,
    "gpu-platform": 900000,
    "shell-boot": 900000,
    "renderer-bootstrap": 900000
  })
});
function bootPhaseCeilingMs(phase) {
  const table = FRAME_WORKER_BOOT_LIVENESS_POLICY.phaseCeilingMs;
  const family = phase.slice(0, phase.indexOf(":") < 0 ? phase.length : phase.indexOf(":"));
  return table[phase] ?? table[family] ?? FRAME_WORKER_BOOT_LIVENESS_POLICY.defaultPhaseCeilingMs;
}
function evaluateBrowserBootLiveness(window2) {
  const silentForMs = Number.isFinite(window2.lastLivenessAtMs) ? Math.max(0, window2.nowMs - window2.lastLivenessAtMs) : Number.POSITIVE_INFINITY;
  if (window2.phase) {
    const phaseElapsedMs = Math.max(0, window2.nowMs - window2.phase.enteredAtMs);
    const remainingMs2 = window2.phase.ceilingMs - phaseElapsedMs;
    if (remainingMs2 > 0)
      return { terminate: false, rearmInMs: Math.min(window2.silenceTimeoutMs, remainingMs2), silentForMs, phaseElapsedMs };
    return { terminate: true, rearmInMs: 0, silentForMs, phaseElapsedMs };
  }
  const remainingMs = window2.silenceTimeoutMs - silentForMs;
  if (remainingMs > 0)
    return { terminate: false, rearmInMs: remainingMs, silentForMs, phaseElapsedMs: 0 };
  return { terminate: true, rearmInMs: 0, silentForMs, phaseElapsedMs: 0 };
}
function roundedMs(value) {
  return Number.isFinite(value) ? String(Math.max(0, Math.round(value))) : "∞";
}
function describeBrowserBootSilence(report, tongue) {
  const stage = report.lastStage || (tongue === "de" ? "—" : "—");
  if (report.phase) {
    return tongue === "de" ? `Die erklärte lange Phase „${report.phase.phase}“ des Frame-Workers lief ${roundedMs(report.phaseElapsedMs)} ms gegen ihre Obergrenze von ${roundedMs(report.phase.ceilingMs)} ms (letzte gemeldete Stufe „${stage}“, still seit ${roundedMs(report.silentForMs)} ms).` : `The frame Worker's declared long phase "${report.phase.phase}" ran ${roundedMs(report.phaseElapsedMs)} ms against its ${roundedMs(report.phase.ceilingMs)} ms ceiling (last reported stage "${stage}", silent for ${roundedMs(report.silentForMs)} ms).`;
  }
  const silence = report.heard ? tongue === "de" ? `war ${roundedMs(report.silentForMs)} ms still` : `was silent for ${roundedMs(report.silentForMs)} ms` : tongue === "de" ? "hat nie eine einzige Nachricht gesendet" : "never sent a single message";
  return tongue === "de" ? `Der Frame-Worker ${silence} (Obergrenze ${roundedMs(report.silenceTimeoutMs)} ms, letzte gemeldete Stufe „${stage}“) und hatte keine lange Phase erklärt — seine Ereignisschleife hängt.` : `The frame Worker ${silence} (ceiling ${roundedMs(report.silenceTimeoutMs)} ms, last reported stage "${stage}") and had declared no long phase — its event loop is wedged.`;
}
function describeBrowserBootPhase(phase, elapsedMs, tongue) {
  if (!phase)
    return tongue === "de" ? "Lange Boot-Phase: keine erklärt" : "Long boot phase: none declared";
  return tongue === "de" ? `Lange Boot-Phase: „${phase.phase}“ seit ${roundedMs(elapsedMs)} ms (Obergrenze ${roundedMs(phase.ceilingMs)} ms)` : `Long boot phase: "${phase.phase}" for ${roundedMs(elapsedMs)} ms (ceiling ${roundedMs(phase.ceilingMs)} ms)`;
}

/* 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts */
var FRAME_WORKER_LOSSLESS_ITEM_CAPACITY = 64;
var FRAME_WORKER_BYTE_CAPACITY = 256 * 1024;
var FRAME_WORKER_BOOT_STALL_TIMEOUT_MS = FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs;
var FRAME_WORKER_POINTER_CAPACITY = 16;
var FRAME_WORKER_MESSAGE_BYTE_CAPACITY = 4 * 1024;
var FRAME_WORKER_TEXT_CHUNK_CODE_UNITS = 1024;
var FRAME_WORKER_INTROSPECTION_CAPACITY = 4;
var FRAME_WORKER_INTROSPECTION_TIMEOUT_MS = 1e4;

class BrowserFrameTransport {
  lifecycle = 1;
  interactiveJobs;
  status = "booting";
  fault;
  worker;
  shardWorkers = new Map;
  now;
  clearTimer;
  setTimer;
  onReady;
  onProgress;
  workerSteps = { degraded: false, recordedOverruns: 0, sustainedOverruns: 0, worstStepMs: 0, worstStepSite: "" };
  workerStepOverruns = 0;
  onDirectives;
  onFault;
  onUiTurn;
  requestRaf;
  cancelRaf;
  pointerIds = new Array(FRAME_WORKER_POINTER_CAPACITY);
  pointerMoves = new Array(FRAME_WORKER_POINTER_CAPACITY);
  pointerCount = 0;
  wheel;
  resize;
  lossless = [];
  losslessBytes = 0;
  nextStreamId = 1;
  generation = 0;
  sequence = 0;
  acceptedSequence = 0;
  inFlight = false;
  frameRequested = false;
  rafHandle;
  bootTimer;
  lastLivenessAtMs = Number.NEGATIVE_INFINITY;
  bootStage = "";
  bootPhase;
  bootStartedAtMs = 0;
  locale;
  closeRequested = false;
  uiTurns = new TurnLedger;
  uiTurnClock;
  deferredWork = [];
  deferredScheduled = false;
  introspections = new Map;
  nextIntrospectionId = 1;
  constructor(options) {
    this.worker = options.worker;
    this.now = options.now ?? (() => performance.now());
    this.locale = options.boot.locale === "de" ? "de" : "en";
    this.bootStartedAtMs = this.now();
    const setTimer = options.setTimer ?? ((callback, delayMs) => window.setTimeout(callback, delayMs));
    this.setTimer = setTimer;
    this.clearTimer = options.clearTimer ?? ((handle) => window.clearTimeout(handle));
    this.onReady = options.onReady;
    this.onProgress = options.onProgress;
    this.onDirectives = options.onDirectives;
    this.onFault = options.onFault;
    this.onUiTurn = options.onUiTurn;
    this.uiTurnClock = new TurnClock(this.now);
    this.requestRaf = options.requestAnimationFrame;
    this.cancelRaf = options.cancelAnimationFrame;
    this.interactiveJobs = new BrowserInteractiveJobPort(this.lifecycle, (message) => this.worker.postMessage(message), this.now, (detail) => this.quarantine("interactive-job-violation", detail), (callback) => void this.setTimer(callback, 0));
    this.worker.onmessage = (event) => this.receive(event.data);
    this.worker.onerror = (event) => this.fail("worker-message-failed", event.message || "Worker error");
    this.worker.onmessageerror = () => this.fail("worker-message-failed", "Worker message could not be decoded");
    this.armBootWatchdog(FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs);
    try {
      this.worker.postMessage({ kind: "boot", lifecycle: this.lifecycle, ...options.boot }, [options.boot.canvas]);
    } catch (error) {
      this.fail("worker-boot-failed", error instanceof Error ? error.message : String(error));
    }
  }
  enqueueReplaceable(event) {
    if (!this.accepting())
      return false;
    this.generation++;
    if (event.kind === "pointer-move") {
      let slot = -1;
      for (let index = 0;index < this.pointerCount; index++) {
        if (this.pointerIds[index] === event.pointerId) {
          slot = index;
          break;
        }
      }
      if (slot < 0) {
        if (this.pointerCount === FRAME_WORKER_POINTER_CAPACITY) {
          this.fail("replaceable-overflow", `pointer lane exceeded ${FRAME_WORKER_POINTER_CAPACITY} active identities`);
          return false;
        }
        slot = this.pointerCount++;
        this.pointerIds[slot] = event.pointerId;
      }
      this.pointerMoves[slot] = { ...event, timestampMs: this.now() };
    }
    if (event.kind === "wheel") {
      const prior = this.wheel?.kind === "wheel" ? this.wheel : undefined;
      this.wheel = prior ? { ...event, deltaX: prior.deltaX + event.deltaX, deltaY: prior.deltaY + event.deltaY, timestampMs: this.now() } : { ...event, timestampMs: this.now() };
    }
    if (event.kind === "resize")
      this.resize = { ...event, timestampMs: this.now() };
    this.requestFrame();
    return true;
  }
  enqueueLossless(event) {
    if (!this.accepting())
      return false;
    if ((event.kind === "key-down" || event.kind === "key-up") && event.key.length > FRAME_WORKER_TEXT_CHUNK_CODE_UNITS) {
      this.fail("lossless-overflow", `key payload exceeded ${FRAME_WORKER_TEXT_CHUNK_CODE_UNITS} code units`);
      return false;
    }
    if ((event.kind === "ime-update" || event.kind === "ime-commit") && event.text.length > FRAME_WORKER_TEXT_CHUNK_CODE_UNITS) {
      this.fail("lossless-overflow", `IME payload exceeded ${FRAME_WORKER_TEXT_CHUNK_CODE_UNITS} code units`);
      return false;
    }
    const bytes = admittedBytes(event);
    if (this.lossless.length >= FRAME_WORKER_LOSSLESS_ITEM_CAPACITY || this.losslessBytes + bytes > FRAME_WORKER_BYTE_CAPACITY) {
      this.fail("lossless-overflow", `lossless lane exceeded ${FRAME_WORKER_LOSSLESS_ITEM_CAPACITY} items or ${FRAME_WORKER_BYTE_CAPACITY} bytes`);
      return false;
    }
    this.lossless.push({ event, bytes, streamId: this.nextStreamId++, timestampMs: this.now(), cursor: 0 });
    this.losslessBytes += bytes;
    this.generation++;
    this.requestFrame();
    return true;
  }
  requestFrame() {
    if (!this.accepting())
      return;
    this.frameRequested = true;
    if (this.requestRaf && this.rafHandle === undefined) {
      this.rafHandle = this.requestRaf((timestampMs) => {
        this.rafHandle = undefined;
        this.flush(timestampMs);
      });
    }
  }
  flush(timestampMs = this.now()) {
    if (this.status !== "ready" || this.inFlight || !this.frameRequested)
      return false;
    const replaceable = [];
    for (let index = 0;index < this.pointerCount; index++) {
      const event = this.pointerMoves[index];
      if (event)
        replaceable.push(event);
    }
    if (this.wheel)
      replaceable.push(this.wheel);
    if (this.resize)
      replaceable.push(this.resize);
    const lossless = this.takeLosslessWireBatch();
    this.pointerMoves.fill(undefined);
    this.pointerCount = 0;
    this.wheel = undefined;
    this.resize = undefined;
    this.frameRequested = this.lossless.length > 0;
    const sequence = ++this.sequence;
    this.inFlight = true;
    try {
      this.uiTurnClock.enter();
      this.worker.postMessage({ kind: "batch", lifecycle: this.lifecycle, sequence, generation: this.generation, timestampMs, replaceable, lossless });
      this.observeUiTurn("frame-transfer", this.uiTurnClock.leave());
      return true;
    } catch (error) {
      this.uiTurnClock.leave();
      this.fail("worker-message-failed", error instanceof Error ? error.message : String(error));
      return false;
    }
  }
  introspect(probe) {
    if (this.status !== "ready" || this.introspections.size >= FRAME_WORKER_INTROSPECTION_CAPACITY)
      return Promise.resolve(null);
    const requestId = this.nextIntrospectionId++;
    this.requestFrame();
    this.flush();
    return new Promise((resolve) => {
      const timer = this.setTimer(() => {
        this.introspections.delete(requestId);
        resolve(null);
      }, FRAME_WORKER_INTROSPECTION_TIMEOUT_MS);
      this.introspections.set(requestId, { resolve, timer });
      try {
        this.worker.postMessage({ kind: "introspect", lifecycle: this.lifecycle, requestId, probe });
      } catch {
        this.introspections.delete(requestId);
        this.clearTimer(timer);
        resolve(null);
      }
    });
  }
  close() {
    if (this.status === "closed")
      return;
    if (this.bootTimer !== undefined)
      this.clearTimer(this.bootTimer);
    if (this.rafHandle !== undefined)
      this.cancelRaf?.(this.rafHandle);
    this.requestWorkerClose();
    this.interactiveJobs.close();
    this.drainInteractiveJobs();
    this.clearQueues();
    this.status = "closed";
  }
  observeUiTurn(site, executingMs) {
    const outcome = this.uiTurns.admit(site, executingMs);
    if (outcome.verdict !== "admitted" && outcome.verdict !== "clock-fault")
      this.onUiTurn?.(outcome);
    return outcome.verdict === "admitted" || outcome.verdict === "clock-fault";
  }
  degraded() {
    return this.uiTurns.degraded();
  }
  fallbackState() {
    return {
      surface: this.status,
      uiThreadFrames: "unavailable-offscreen-transferred",
      workerTerminated: this.status === "faulted" || this.status === "closed",
      inputAccepted: this.accepting(),
      deferredCadence: this.uiTurns.degraded(),
      uiTurns: this.uiTurns.snapshot(),
      workerSteps: { ...this.workerSteps, sustainedOverruns: this.workerSteps.sustainedOverruns + this.workerStepOverruns },
      bootPhase: this.bootPhase,
      bootPhaseElapsedMs: this.bootPhase ? Math.max(0, this.now() - this.bootPhase.enteredAtMs) : 0,
      bootStage: this.bootStage,
      bootSilentForMs: Math.max(0, this.now() - (Number.isFinite(this.lastLivenessAtMs) ? this.lastLivenessAtMs : this.bootStartedAtMs))
    };
  }
  uiTurnP99Ms() {
    return this.uiTurns.p99Ms();
  }
  deferToNextTurn(work) {
    this.deferredWork.push(work);
    if (this.deferredScheduled)
      return;
    this.deferredScheduled = true;
    this.setTimer(() => {
      this.deferredScheduled = false;
      const pending = this.deferredWork;
      this.deferredWork = [];
      for (const item of pending) {
        if (this.status === "closed")
          return;
        this.uiTurnClock.enter();
        try {
          item();
        } catch (error) {
          this.uiTurnClock.leave();
          this.fail("ui-hook-failed", `deferred UI turn threw: ${error instanceof Error ? error.message : String(error)}`);
          return;
        }
        this.observeUiTurn("deferred-hook", this.uiTurnClock.leave());
      }
    }, 0);
  }
  armBootWatchdog(delayMs) {
    if (this.bootTimer !== undefined)
      this.clearTimer(this.bootTimer);
    this.bootTimer = this.setTimer(() => this.judgeBootLiveness(), Math.max(0, delayMs));
  }
  judgeBootLiveness() {
    this.bootTimer = undefined;
    if (this.status !== "booting")
      return;
    const nowMs = this.now();
    const window2 = { nowMs, lastLivenessAtMs: this.lastLivenessAtMs, silenceTimeoutMs: FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs, phase: this.bootPhase };
    const decision = evaluateBrowserBootLiveness(window2);
    if (!decision.terminate) {
      this.armBootWatchdog(decision.rearmInMs);
      return;
    }
    const detail = describeBrowserBootSilence({ heard: Number.isFinite(this.lastLivenessAtMs), lastStage: this.bootStage, silentForMs: decision.silentForMs, silenceTimeoutMs: window2.silenceTimeoutMs, phase: this.bootPhase, phaseElapsedMs: decision.phaseElapsedMs }, this.locale);
    this.fail("worker-boot-timeout", detail);
  }
  witnessWorker() {
    this.lastLivenessAtMs = this.now();
  }
  accepting() {
    return this.status === "booting" || this.status === "ready";
  }
  spawnShardWorker(shardIndex, url) {
    this.terminateShardWorker(shardIndex);
    const worker = new Worker(url, { type: "module" });
    const channel = new MessageChannel;
    worker.onmessage = (event) => channel.port1.postMessage(event.data);
    worker.onerror = (event) => channel.port1.postMessage({ kind: "shard-worker-error", message: event.message ?? "", filename: event.filename ?? "", lineno: event.lineno ?? 0 });
    channel.port1.onmessage = (event) => worker.postMessage(event.data);
    channel.port1.start();
    this.shardWorkers.set(shardIndex, worker);
    this.worker.postMessage({ kind: "shard-port", shardIndex, port: channel.port2 }, [channel.port2]);
  }
  terminateShardWorker(shardIndex) {
    const existing = this.shardWorkers.get(shardIndex);
    if (!existing)
      return;
    this.shardWorkers.delete(shardIndex);
    existing.onmessage = null;
    existing.onerror = null;
    existing.terminate();
  }
  receive(message) {
    if (message.kind === "shard-spawn") {
      this.spawnShardWorker(message.shardIndex, message.url);
      return;
    }
    if (message.kind === "shard-terminate") {
      this.terminateShardWorker(message.shardIndex);
      return;
    }
    if (message.lifecycle !== this.lifecycle)
      return;
    if (message.kind === "job-input-pull" || message.kind === "job-output-page" || message.kind === "job-terminal") {
      this.interactiveJobs.receive(message);
      return;
    }
    if (message.kind === "introspection") {
      const pending = this.introspections.get(message.requestId);
      if (!pending)
        return;
      this.introspections.delete(message.requestId);
      this.clearTimer(pending.timer);
      pending.resolve(message.json);
      return;
    }
    if (message.kind === "closed") {
      this.worker.terminate();
      return;
    }
    if (this.status === "closed" || this.status === "faulted" || this.status === "quarantined")
      return;
    if (message.kind === "booted") {
      if (this.bootTimer !== undefined)
        this.clearTimer(this.bootTimer);
      this.bootTimer = undefined;
      this.bootPhase = undefined;
      this.witnessWorker();
      this.status = "ready";
      this.interactiveJobs.ready();
      if (!this.runUiHook("ready-hook", () => this.onReady?.()))
        return;
      this.requestFrame();
      return;
    }
    if (message.kind === "boot-liveness") {
      if (this.status === "booting")
        this.witnessWorker();
      return;
    }
    if (message.kind === "boot-phase") {
      if (this.status !== "booting")
        return;
      this.witnessWorker();
      if (message.state === "enter")
        this.bootPhase = { phase: message.phase, ceilingMs: bootPhaseCeilingMs(message.phase), enteredAtMs: this.now() };
      else if (this.bootPhase?.phase === message.phase)
        this.bootPhase = undefined;
      return;
    }
    if (message.kind === "boot-progress") {
      if (this.status !== "booting")
        return;
      this.witnessWorker();
      this.bootStage = message.stage;
      this.workerSteps = message.worker;
      const report = () => this.onProgress?.(message.stage, message.progress, message.worker);
      if (this.degraded() || message.worker.degraded)
        this.deferToNextTurn(report);
      else
        this.runUiHook("progress-hook", report);
      return;
    }
    if (message.kind === "wake") {
      this.requestFrame();
      return;
    }
    if (message.kind === "fault") {
      this.fail(this.status === "booting" ? "worker-boot-failed" : "worker-runtime-failed", `${message.code}: ${message.detail}`);
      return;
    }
    if (message.generation > this.generation) {
      this.fail("protocol-violation", `Worker returned future generation ${message.generation} while UI generation is ${this.generation}`);
      return;
    }
    if (message.sequence <= this.acceptedSequence)
      return;
    this.inFlight = false;
    if (message.workerStepVerdict === "sustained-overrun")
      this.workerStepOverruns++;
    if (message.quarantined) {
      const code = message.faultCode === "present-failed" ? "worker-present-failed" : message.faultCode === "text-input-failed" ? "worker-input-failed" : "worker-step-overrun";
      this.quarantine(code, message.faultDetail ?? `worker frame step executed ${message.workerExecutingMs.toFixed(3)} ms`);
      return;
    }
    if (message.generation === this.generation) {
      this.acceptedSequence = message.sequence;
      if (!this.runUiHook("directive-hook", () => this.onDirectives?.({ cursor: message.cursor, fullscreen: message.fullscreen, generation: message.generation, workerDurationMs: message.workerDurationMs })))
        return;
    }
    if (message.requestFrame || this.frameRequested || message.generation < this.generation)
      this.requestFrame();
  }
  fail(code, detail) {
    if (this.status === "faulted" || this.status === "closed")
      return;
    if (this.bootTimer !== undefined)
      this.clearTimer(this.bootTimer);
    if (this.rafHandle !== undefined)
      this.cancelRaf?.(this.rafHandle);
    this.requestWorkerClose();
    this.interactiveJobs.close();
    this.drainInteractiveJobs();
    this.clearQueues();
    this.fault = { code, detail };
    this.status = "faulted";
    const fallback = this.fallbackState();
    this.runUiHook("fault-hook", () => this.onFault?.(code, detail, fallback));
  }
  quarantine(code, detail) {
    if (this.status !== "ready")
      return;
    if (this.rafHandle !== undefined)
      this.cancelRaf?.(this.rafHandle);
    this.interactiveJobs.quarantineFromOwner();
    this.drainInteractiveJobs();
    this.requestWorkerClose();
    this.clearQueues();
    this.fault = { code, detail };
    this.status = "quarantined";
    const fallback = this.fallbackState();
    this.runUiHook("fault-hook", () => this.onFault?.(code, detail, fallback));
  }
  runUiHook(site, callback) {
    this.uiTurnClock.enter();
    try {
      callback();
    } catch (error) {
      this.uiTurnClock.leave();
      const detail = `${site} threw: ${error instanceof Error ? error.message : String(error)}`;
      if (this.status !== "faulted" && this.status !== "closed")
        this.fail("ui-hook-failed", detail);
      return false;
    }
    this.observeUiTurn(site, this.uiTurnClock.leave());
    return true;
  }
  clearQueues() {
    for (const pending of this.introspections.values()) {
      this.clearTimer(pending.timer);
      pending.resolve(null);
    }
    this.introspections.clear();
    this.pointerMoves.fill(undefined);
    this.pointerCount = 0;
    this.wheel = undefined;
    this.resize = undefined;
    const retiredLossless = this.lossless;
    this.lossless = [];
    const drain = () => {
      retiredLossless.pop();
      if (retiredLossless.length > 0)
        this.setTimer(drain, 0);
    };
    if (retiredLossless.length > 0)
      this.setTimer(drain, 0);
    this.losslessBytes = 0;
    this.frameRequested = false;
    this.inFlight = false;
  }
  drainInteractiveJobs() {
    if (this.interactiveJobs.closeStep())
      return;
    this.setTimer(() => this.drainInteractiveJobs(), 0);
  }
  requestWorkerClose() {
    if (this.closeRequested)
      return;
    this.closeRequested = true;
    try {
      this.worker.postMessage({ kind: "close", lifecycle: this.lifecycle });
    } catch {}
  }
  takeLosslessWireBatch() {
    const batch = [];
    let budget = FRAME_WORKER_MESSAGE_BYTE_CAPACITY - 2048;
    while (batch.length < 16 && this.lossless.length > 0 && budget > 256) {
      const queued = this.lossless[0];
      const event = queued.event;
      if (event.kind !== "text" && event.kind !== "paste" && event.kind !== "ime-update" && event.kind !== "ime-commit") {
        batch.push({ ...event, timestampMs: queued.timestampMs });
        budget -= Math.min(queued.bytes, 512);
        this.lossless.shift();
        this.losslessBytes -= queued.bytes;
        continue;
      }
      const remaining = event.text.length - queued.cursor;
      const take = Math.min(remaining, FRAME_WORKER_TEXT_CHUNK_CODE_UNITS, Math.max(1, Math.floor((budget - 512) / 6)));
      let end = queued.cursor + take;
      if (end < event.text.length && isHighSurrogate(event.text.charCodeAt(end - 1)) && isLowSurrogate(event.text.charCodeAt(end)))
        end--;
      const final = end === event.text.length;
      batch.push({ kind: "text-chunk", streamId: queued.streamId, target: event.kind, text: event.text.slice(queued.cursor, end), totalBytes: 3 * event.text.length, final, timestampMs: queued.timestampMs, ...event.kind === "ime-update" ? { cursor: event.cursor } : {} });
      queued.cursor = end;
      budget -= 6 * take + 512;
      if (final) {
        this.lossless.shift();
        this.losslessBytes -= queued.bytes;
      }
    }
    return batch;
  }
}
function admittedBytes(event) {
  if (event.kind === "text" || event.kind === "paste" || event.kind === "ime-update" || event.kind === "ime-commit")
    return 3 * event.text.length + 128;
  if (event.kind === "key-down" || event.kind === "key-up")
    return 2 * event.key.length + 128;
  return 128;
}
function isHighSurrogate(value) {
  return value >= 55296 && value <= 56319;
}
function isLowSurrogate(value) {
  return value >= 56320 && value <= 57343;
}

/* 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/📡️interactive-jobs.ts */
var unavailableInteractiveJobPort = {
  status: "unavailable",
  getSnapshot: () => ({ status: "unavailable", revision: 0 }),
  subscribe: () => () => {},
  observeConsumerTurn: () => true,
  reportConsumerFault: () => {},
  submit: () => {
    return;
  }
};
var INTERACTIVE_JOB_OBSERVER_CAPACITY2 = 32;
var interactiveJobObservers = new Array(INTERACTIVE_JOB_OBSERVER_CAPACITY2);
var installedInteractiveJobPort = unavailableInteractiveJobPort;
var unsubscribeInstalled = () => {};
var interactiveJobRevision = 0;
var interactiveJobSnapshot = { status: "unavailable", revision: 0 };
var observerCursor = 0;
var observerNotifyScheduled = false;
function setInteractiveJobPort(port) {
  const previous = installedInteractiveJobPort;
  unsubscribeInstalled();
  installedInteractiveJobPort = port;
  unsubscribeInstalled = port.subscribe(publishInteractiveJobSnapshot);
  publishInteractiveJobSnapshot();
  return previous;
}
function publishInteractiveJobSnapshot() {
  interactiveJobRevision += 1;
  interactiveJobSnapshot = { status: installedInteractiveJobPort.status, revision: interactiveJobRevision };
  observerCursor = 0;
  if (observerNotifyScheduled)
    return;
  observerNotifyScheduled = true;
  setTimeout(notifyOneInteractiveJobObserver, 0);
}
function notifyOneInteractiveJobObserver() {
  observerNotifyScheduled = false;
  while (observerCursor < interactiveJobObservers.length && !interactiveJobObservers[observerCursor])
    observerCursor++;
  if (observerCursor === interactiveJobObservers.length)
    return;
  const observer = interactiveJobObservers[observerCursor++];
  const startedAt = typeof performance === "undefined" ? Date.now() : performance.now();
  try {
    observer();
  } catch (error) {
    installedInteractiveJobPort.reportConsumerFault("status observer", error instanceof Error ? error.message : String(error));
    return;
  }
  const finishedAt = typeof performance === "undefined" ? Date.now() : performance.now();
  installedInteractiveJobPort.observeConsumerTurn("status observer", finishedAt - startedAt);
  observerNotifyScheduled = true;
  setTimeout(notifyOneInteractiveJobObserver, 0);
}

/* 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts */
var DEFAULT_HOST_VARIANT = "s";

/* 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts */
var RENDERER_MODULE_URL = new URL("./semio-framework-os-renderer-wgpu.js", import.meta.url).href;
var RENDERER_WASM_URL = new URL("./semio-framework-os-renderer-wgpu_bg.wasm", import.meta.url).href;
var FRAME_WORKER_URL = new URL("./🎞️frame-worker.js", import.meta.url);
var BOOT_FIELD_CAPACITY = 2048;
var LOCATION_SEARCH_CAPACITY = 8192;
await new Promise((resolve) => {
  if (document.readyState === "loading")
    document.addEventListener("DOMContentLoaded", () => resolve(), { once: true });
  else
    resolve();
});
function armUiTurnDiagnostics() {
  try {
    const stored = globalThis.localStorage?.getItem(TURN_DIAGNOSTICS_KEY);
    if (stored !== null && stored !== undefined)
      setTurnDiagnostics(["1", "true", "on", "yes"].includes(stored.trim().toLowerCase()));
  } catch {
    setTurnDiagnostics(undefined);
  }
}
function locale() {
  return navigator.language.toLowerCase().startsWith("de") ? "de" : "en";
}
function bounded(value, field) {
  if (value.length > BOOT_FIELD_CAPACITY)
    throw new Error(`boot-descriptor-overflow: ${field} exceeds ${BOOT_FIELD_CAPACITY} code units`);
  return value;
}
function bootDescriptor() {
  if (window.location.search.length > LOCATION_SEARCH_CAPACITY)
    throw new Error(`boot-descriptor-overflow: location.search exceeds ${LOCATION_SEARCH_CAPACITY} code units`);
  const params = new URLSearchParams(window.location.search);
  const hubUrl = params.get("hub");
  return {
    pluginVariant: bounded(params.get("plugin") ?? DEFAULT_HOST_VARIANT, "plugin"),
    appRole: params.get("role") === "viewer" ? "viewer" : "editor",
    ...hubUrl ? { hub: { hubUrl: bounded(hubUrl, "hub"), user: bounded(params.get("user") ?? "", "user"), dataDir: bounded(params.get("dataDir") ?? "", "dataDir") } } : {}
  };
}
var WGPU_CANVAS_ID = "semio-wgpu-canvas";
function canvasElement() {
  const canvas = document.createElement("canvas");
  canvas.id = WGPU_CANVAS_ID;
  canvas.tabIndex = 0;
  canvas.setAttribute("aria-label", locale() === "de" ? "Semio Arbeitsfläche" : "Semio workspace");
  canvas.style.cssText = "display:block;width:100%;height:100%;touch-action:none;outline:none;";
  return canvas;
}
var WGPU_INTROSPECTION_GLOBAL = "semioWgpuIntrospection";
function attachIntrospectionBindings(transport) {
  const probe = (kind) => async () => await transport.introspect(kind) ?? "";
  const host = window;
  host.semioWgpuIntrospection = { dumpStructure: probe("structure"), dumpFrameStats: probe("frame-stats") };
  return () => delete host.semioWgpuIntrospection;
}
function statusElement(root) {
  const status = document.createElement("div");
  status.setAttribute("role", "status");
  status.setAttribute("aria-live", "polite");
  status.style.cssText = "position:fixed;left:12px;bottom:12px;padding:6px 9px;background:#001117cc;color:#d7f7ff;font:12px monospace;z-index:9997;";
  root.appendChild(status);
  return status;
}
function fallbackLines(state, tongue) {
  if (!state) {
    return tongue === "de" ? "Oberfläche: vor der Übergabe der Zeichenfläche an den Worker gescheitert. Kein Frame-Pfad war je aktiv." : "Surface: failed before the canvas reached the Worker. No frame path was ever live.";
  }
  const turns = state.uiTurns;
  const steps = state.workerSteps;
  const ledger = `${turns.recordedOverruns}/${turns.sustainedOverruns} (p99 ${turns.p99Ms.toFixed(3)} ms, worst ${turns.worstExecutingMs.toFixed(3)} ms @ ${turns.worstSite || "—"})`;
  const workerLedger = `${steps.recordedOverruns}/${steps.sustainedOverruns} (worst ${steps.worstStepMs.toFixed(3)} ms @ ${steps.worstStepSite || "—"})`;
  const phaseLine = describeBrowserBootPhase(state.bootPhase, state.bootPhaseElapsedMs, tongue);
  if (tongue === "de") {
    return [
      `Oberfläche: ${state.surface}${state.deferredCadence ? " · verzögerte Taktung" : ""}`,
      `Boot-Stufe: ${state.bootStage || "—"} · still seit ${Math.round(state.bootSilentForMs)} ms`,
      phaseLine,
      `UI-Thread-Frames: nicht verfügbar — die Zeichenfläche gehört dem Frame-Worker (OffscreenCanvas übergeben)`,
      `Worker beendet: ${state.workerTerminated ? "ja" : "nein"} · Eingaben angenommen: ${state.inputAccepted ? "ja" : "nein"}`,
      `UI-Takte über dem Budget (erfasst/anhaltend): ${ledger}`,
      `Worker-Schritte über dem Budget (erfasst/anhaltend): ${workerLedger}${steps.degraded ? " · verzögerte Taktung" : ""}`
    ].join(`
`);
  }
  return [
    `Surface: ${state.surface}${state.deferredCadence ? " · deferred cadence" : ""}`,
    `Boot stage: ${state.bootStage || "—"} · silent for ${Math.round(state.bootSilentForMs)} ms`,
    phaseLine,
    `UI-thread frames: unavailable — the canvas belongs to the frame Worker (OffscreenCanvas transferred)`,
    `Worker terminated: ${state.workerTerminated ? "yes" : "no"} · input accepted: ${state.inputAccepted ? "yes" : "no"}`,
    `UI turns over budget (recorded/sustained): ${ledger}`,
    `Worker steps over budget (recorded/sustained): ${workerLedger}${steps.degraded ? " · deferred cadence" : ""}`
  ].join(`
`);
}
function renderFault(root, code, detail, state) {
  const banner = document.createElement("div");
  banner.setAttribute("role", "alert");
  banner.style.cssText = "position:fixed;inset:0;padding:24px;background:#2a0a0acc;color:#ffb4b4;font:14px monospace;white-space:pre-wrap;overflow:auto;z-index:9999;";
  const tongue = locale();
  const title = tongue === "de" ? "wgpu-Renderer-Fehler" : "wgpu renderer fault";
  banner.textContent = `${title}:

${code}: ${detail}

${fallbackLines(state, tongue)}`;
  root.appendChild(banner);
}
function wireInput(canvas, transport) {
  const abort = new AbortController;
  const options = { signal: abort.signal };
  const pointer = (event) => ({
    pointerId: event.pointerId,
    pointerKind: event.pointerType === "touch" || event.pointerType === "pen" ? event.pointerType : "mouse",
    x: event.offsetX * window.devicePixelRatio,
    y: event.offsetY * window.devicePixelRatio,
    pressure: event.pressure || undefined,
    tiltX: event.tiltX || undefined,
    tiltY: event.tiltY || undefined
  });
  const observed = (site, startedAt) => void transport.observeUiTurn(site, performance.now() - startedAt);
  canvas.addEventListener("pointermove", (event) => {
    const startedAt = performance.now();
    transport.enqueueReplaceable({ kind: "pointer-move", ...pointer(event) });
    observed("pointer-move", startedAt);
  }, options);
  canvas.addEventListener("pointerdown", (event) => {
    const startedAt = performance.now();
    canvas.focus({ preventScroll: true });
    canvas.setPointerCapture(event.pointerId);
    transport.enqueueLossless({ kind: "pointer-down", ...pointer(event), button: event.button === 2 ? "secondary" : event.button === 1 ? "middle" : "primary" });
    observed("pointer-down", startedAt);
  }, options);
  canvas.addEventListener("pointerup", (event) => {
    const startedAt = performance.now();
    transport.enqueueLossless({ kind: "pointer-up", ...pointer(event), button: event.button === 2 ? "secondary" : event.button === 1 ? "middle" : "primary" });
    observed("pointer-up", startedAt);
  }, options);
  canvas.addEventListener("wheel", (event) => {
    const startedAt = performance.now();
    event.preventDefault();
    transport.enqueueReplaceable({ kind: "wheel", x: event.offsetX * window.devicePixelRatio, y: event.offsetY * window.devicePixelRatio, deltaX: event.deltaX, deltaY: event.deltaY });
    observed("wheel", startedAt);
  }, { ...options, passive: false });
  const key = (event, kind) => {
    const startedAt = performance.now();
    transport.enqueueLossless({ kind, key: event.key, shift: event.shiftKey, ctrl: event.ctrlKey, alt: event.altKey, meta: event.metaKey });
    observed(kind, startedAt);
  };
  canvas.addEventListener("keydown", (event) => void key(event, "key-down"), options);
  canvas.addEventListener("keyup", (event) => void key(event, "key-up"), options);
  canvas.addEventListener("compositionstart", () => {
    const startedAt = performance.now();
    transport.enqueueLossless({ kind: "ime-start" });
    observed("ime-start", startedAt);
  }, options);
  canvas.addEventListener("compositionupdate", (event) => {
    const startedAt = performance.now();
    transport.enqueueLossless({ kind: "ime-update", text: event.data, cursor: event.data.length });
    observed("ime-update", startedAt);
  }, options);
  canvas.addEventListener("compositionend", (event) => {
    const startedAt = performance.now();
    transport.enqueueLossless({ kind: "ime-commit", text: event.data });
    observed("ime-commit", startedAt);
  }, options);
  canvas.addEventListener("paste", (event) => {
    const startedAt = performance.now();
    const items = event.clipboardData?.items;
    if (items) {
      const count = Math.min(items.length, 16);
      for (let index = 0;index < count; index++) {
        const item = items[index];
        if (item?.kind !== "string" || item.type !== "text/plain")
          continue;
        item.getAsString((text) => {
          const handoffStartedAt = performance.now();
          transport.enqueueLossless({ kind: "paste", text });
          transport.observeUiTurn("paste-handoff", performance.now() - handoffStartedAt);
        });
        break;
      }
    }
    observed("paste", startedAt);
  }, options);
  return () => abort.abort();
}
async function mount(root) {
  armUiTurnDiagnostics();
  const descriptor = bootDescriptor();
  if (typeof Worker === "undefined")
    throw new Error("worker-unavailable: Dedicated Worker is not supported");
  const canvas = canvasElement();
  if (typeof canvas.transferControlToOffscreen !== "function")
    throw new Error("offscreen-canvas-unavailable: OffscreenCanvas transfer is not supported");
  root.replaceChildren(canvas);
  const status = statusElement(root);
  const dpr = window.devicePixelRatio || 1;
  const width = Math.max(1, Math.round(canvas.clientWidth * dpr));
  const height = Math.max(1, Math.round(canvas.clientHeight * dpr));
  canvas.width = width;
  canvas.height = height;
  let offscreen;
  try {
    offscreen = canvas.transferControlToOffscreen();
  } catch (error) {
    throw new Error(`offscreen-transfer-failed: ${error instanceof Error ? error.message : String(error)}`);
  }
  let worker;
  try {
    worker = new Worker(FRAME_WORKER_URL, { type: "module", name: "semio-frame-worker" });
  } catch (error) {
    throw new Error(`worker-construction-failed: ${error instanceof Error ? error.message : String(error)}`);
  }
  let cleanupInput = () => {};
  let detachIntrospection = () => {};
  const transport = new BrowserFrameTransport({
    worker,
    boot: { bindingsModuleUrl: RENDERER_MODULE_URL, bindingsWasmUrl: RENDERER_WASM_URL, canvas: offscreen, width, height, dpr, pluginVariant: descriptor.pluginVariant, locale: locale(), appRole: descriptor.appRole, hub: descriptor.hub },
    requestAnimationFrame: (callback) => window.requestAnimationFrame(callback),
    cancelAnimationFrame: (handle) => window.cancelAnimationFrame(handle),
    onProgress: (stage, progress, worker2) => {
      status.textContent = `${stage} ${Math.round(progress * 100)}%${worker2.degraded ? locale() === "de" ? " · verzögerte Taktung" : " · deferred cadence" : ""}`;
      status.dataset.workerDegraded = worker2.degraded ? "true" : "false";
      status.dataset.workerStepOverruns = String(worker2.recordedOverruns);
    },
    onUiTurn: (outcome) => {
      canvas.dataset.uiTurn = `${outcome.verdict}:${outcome.site}:${outcome.executingMs.toFixed(3)}`;
    },
    onReady: () => {
      status.remove();
      detachIntrospection = attachIntrospectionBindings(transport);
      cleanupInput = wireInput(canvas, transport);
      transport.enqueueReplaceable({ kind: "resize", width, height, dpr });
      canvas.focus({ preventScroll: true });
    },
    onDirectives: ({ cursor, fullscreen }) => {
      canvas.style.cursor = cursor;
      if (fullscreen === true)
        canvas.requestFullscreen().catch(() => {});
      if (fullscreen === false && document.fullscreenElement)
        document.exitFullscreen().catch(() => {});
    },
    onFault: (code, detail, fallback) => {
      cleanupInput();
      detachIntrospection();
      renderFault(root, code, detail, fallback);
    }
  });
  const previousInteractiveJobPort = setInteractiveJobPort(transport.interactiveJobs);
  const resize = new ResizeObserver(() => {
    const startedAt = performance.now();
    const nextDpr = window.devicePixelRatio || 1;
    transport.enqueueReplaceable({ kind: "resize", width: Math.max(1, Math.round(canvas.clientWidth * nextDpr)), height: Math.max(1, Math.round(canvas.clientHeight * nextDpr)), dpr: nextDpr });
    transport.observeUiTurn("resize-observer", performance.now() - startedAt);
  });
  resize.observe(canvas);
  window.addEventListener("pagehide", () => {
    resize.disconnect();
    cleanupInput();
    detachIntrospection();
    setInteractiveJobPort(previousInteractiveJobPort);
    transport.close();
  }, { once: true });
}
var root = document.getElementById("root");
if (!root)
  throw new Error("missing-root: #root is unavailable");
try {
  await mount(root);
} catch (error) {
  const detail = error instanceof Error ? error.message : String(error);
  renderFault(root, "worker-boot-failed", detail);
  throw error;
}
export {
  WGPU_INTROSPECTION_GLOBAL,
  WGPU_CANVAS_ID
};
