/* ../../🔌️browser-interactive-job-port/🟦️.ts */
var INTERACTIVE_JOB_SLOT_CAPACITY = 16;
var INTERACTIVE_JOB_INPUT_ITEM_CAPACITY = 65536;
var INTERACTIVE_JOB_INPUT_BYTE_CAPACITY = 256 * 1024 * 1024;
var INTERACTIVE_JOB_PAGE_ITEM_CAPACITY = 128;
var INTERACTIVE_JOB_PAGE_BYTE_CAPACITY = 16 * 1024;
var INTERACTIVE_JOB_UI_BUDGET_MS = 2;
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
  now;
  constructor(lifecycle, send, now, quarantineConsumer, schedule = (callback) => setTimeout(callback, 0)) {
    this.lifecycle = lifecycle;
    this.send = send;
    this.quarantineConsumer = quarantineConsumer;
    this.schedule = schedule;
    this.now = now;
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
    if (durationMs < INTERACTIVE_JOB_UI_BUDGET_MS)
      return true;
    this.quarantine(`${site} took ${durationMs.toFixed(3)} ms`);
    return false;
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
      const startedAt2 = this.now();
      let page;
      try {
        page = slot.consumer.readInputPage(message.cursor, Math.min(message.maxItems, slot.descriptor.inputPageItems));
      } catch (error) {
        this.quarantine(`input consumer threw: ${error instanceof Error ? error.message : String(error)}`);
        return true;
      }
      if (!this.observe(startedAt2, "input consumer"))
        return true;
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
      const startedAt2 = this.now();
      try {
        slot.consumer.onOutputPage(message.page);
      } catch (error) {
        this.quarantine(`output consumer threw: ${error instanceof Error ? error.message : String(error)}`);
        return true;
      }
      if (!this.observe(startedAt2, "output consumer"))
        return true;
      return true;
    }
    if (message.status !== "complete" && message.status !== "cancelled" && message.status !== "fault") {
      this.quarantine("interactive job returned invalid terminal status");
      return true;
    }
    const terminal = { operation: message.operation, generation: message.generation, status: message.status, ...message.detail === undefined ? {} : { detail: message.detail } };
    const startedAt = this.now();
    try {
      slot.consumer.onTerminal(terminal);
    } catch (error) {
      this.quarantine(`terminal consumer threw: ${error instanceof Error ? error.message : String(error)}`);
      slot.closing = true;
      this.scheduleClose();
      return true;
    }
    slot.closing = true;
    if (!this.observe(startedAt, "terminal consumer"))
      return true;
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
    const startedAt = this.now();
    let complete = false;
    try {
      complete = slot.consumer.closeStep();
      if (complete)
        complete = slot.consumer.terminalIsEmpty();
    } catch (error) {
      this.quarantine(`consumer close threw: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
    if (!this.observe(startedAt, "consumer close"))
      return false;
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
  observe(startedAt, site) {
    const duration = this.now() - startedAt;
    if (duration < INTERACTIVE_JOB_UI_BUDGET_MS)
      return true;
    this.quarantine(`${site} took ${duration.toFixed(3)} ms`);
    return false;
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
    const startedAt = this.now();
    try {
      observer();
    } catch (error) {
      this.quarantine(`status observer threw: ${error instanceof Error ? error.message : String(error)}`);
      return;
    }
    if (!this.observe(startedAt, "status observer"))
      return;
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

/* ../../🚚️browser-frame-transport/🟦️.ts */
var FRAME_WORKER_LOSSLESS_ITEM_CAPACITY = 64;
var FRAME_WORKER_BYTE_CAPACITY = 256 * 1024;
var FRAME_WORKER_BOOT_TIMEOUT_MS = 15000;
var FRAME_WORKER_POINTER_CAPACITY = 16;
var FRAME_WORKER_MESSAGE_BYTE_CAPACITY = 4 * 1024;
var FRAME_WORKER_TEXT_CHUNK_CODE_UNITS = 1024;
var FRAME_UI_TURN_BUDGET_MS = 2;

class BrowserFrameTransport {
  lifecycle = 1;
  interactiveJobs;
  status = "booting";
  fault;
  worker;
  now;
  clearTimer;
  setTimer;
  onReady;
  onProgress;
  onDirectives;
  onFault;
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
  closeRequested = false;
  uiTurnSamples = new Float64Array(64);
  uiTurnSampleCount = 0;
  constructor(options) {
    this.worker = options.worker;
    this.now = options.now ?? (() => performance.now());
    const setTimer = options.setTimer ?? ((callback, delayMs) => window.setTimeout(callback, delayMs));
    this.setTimer = setTimer;
    this.clearTimer = options.clearTimer ?? ((handle) => window.clearTimeout(handle));
    this.onReady = options.onReady;
    this.onProgress = options.onProgress;
    this.onDirectives = options.onDirectives;
    this.onFault = options.onFault;
    this.requestRaf = options.requestAnimationFrame;
    this.cancelRaf = options.cancelAnimationFrame;
    this.interactiveJobs = new BrowserInteractiveJobPort(this.lifecycle, (message) => this.worker.postMessage(message), this.now, (detail) => this.quarantine("ui-turn-overrun", detail), (callback) => void this.setTimer(callback, 0));
    this.worker.onmessage = (event) => this.receive(event.data);
    this.worker.onerror = (event) => this.fail("worker-message-failed", event.message || "Worker error");
    this.worker.onmessageerror = () => this.fail("worker-message-failed", "Worker message could not be decoded");
    this.bootTimer = setTimer(() => this.fail("worker-boot-timeout", `Worker did not boot within ${FRAME_WORKER_BOOT_TIMEOUT_MS} ms`), FRAME_WORKER_BOOT_TIMEOUT_MS);
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
      const startedAt = this.now();
      this.worker.postMessage({ kind: "batch", lifecycle: this.lifecycle, sequence, generation: this.generation, timestampMs, replaceable, lossless });
      const duration = this.now() - startedAt;
      if (!this.observeUiTurn("frame-transfer", duration))
        return false;
      return true;
    } catch (error) {
      this.fail("worker-message-failed", error instanceof Error ? error.message : String(error));
      return false;
    }
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
  observeUiTurn(site, durationMs) {
    this.uiTurnSamples[this.uiTurnSampleCount % this.uiTurnSamples.length] = durationMs;
    this.uiTurnSampleCount++;
    if (durationMs < FRAME_UI_TURN_BUDGET_MS)
      return true;
    if (this.status === "quarantined" || this.status === "faulted" || this.status === "closed")
      return false;
    const detail = `${site} UI turn took ${durationMs.toFixed(3)} ms`;
    if (this.status === "ready")
      this.quarantine("ui-turn-overrun", detail);
    else
      this.fail("ui-turn-overrun", detail);
    return false;
  }
  uiTurnP99Ms() {
    const count = Math.min(this.uiTurnSampleCount, this.uiTurnSamples.length);
    if (count === 0)
      return 0;
    const samples = Array.from(this.uiTurnSamples.subarray(0, count)).sort((left, right) => left - right);
    return samples[Math.min(count - 1, Math.ceil(count * 0.99) - 1)];
  }
  accepting() {
    return this.status === "booting" || this.status === "ready";
  }
  receive(message) {
    if (message.lifecycle !== this.lifecycle)
      return;
    if (message.kind === "job-input-pull" || message.kind === "job-output-page" || message.kind === "job-terminal") {
      this.interactiveJobs.receive(message);
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
      this.status = "ready";
      this.interactiveJobs.ready();
      if (!this.runUiHook("ready-hook", () => this.onReady?.()))
        return;
      this.requestFrame();
      return;
    }
    if (message.kind === "boot-progress") {
      if (this.status === "booting")
        this.runUiHook("progress-hook", () => this.onProgress?.(message.stage, message.progress));
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
    if (message.quarantined || message.workerDurationMs >= 8) {
      const code = message.faultCode === "present-failed" ? "worker-present-failed" : message.faultCode === "text-input-failed" ? "worker-input-failed" : "worker-step-overrun";
      this.quarantine(code, message.faultDetail ?? `worker frame step took ${message.workerDurationMs.toFixed(3)} ms`);
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
    this.runUiHook("fault-hook", () => this.onFault?.(code, detail));
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
    this.runUiHook("fault-hook", () => this.onFault?.(code, detail));
  }
  runUiHook(site, callback) {
    const startedAt = this.now();
    try {
      callback();
    } catch (error) {
      const detail = `${site} threw: ${error instanceof Error ? error.message : String(error)}`;
      if (this.status === "ready")
        this.quarantine("ui-turn-overrun", detail);
      else if (this.status !== "quarantined" && this.status !== "faulted" && this.status !== "closed")
        this.fail("ui-turn-overrun", detail);
      return false;
    }
    return this.observeUiTurn(site, this.now() - startedAt);
  }
  clearQueues() {
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

/* ../../../../../../../../../🔨️modules/🖱️ui/🧱️elements/🔌️Ports/📡️interactive-jobs.ts */
var unavailableInteractiveJobPort = {
  status: "unavailable",
  getSnapshot: () => ({ status: "unavailable", revision: 0 }),
  subscribe: () => () => {},
  observeConsumerTurn: () => true,
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
  } catch {
    installedInteractiveJobPort.observeConsumerTurn("status observer threw", Number.POSITIVE_INFINITY);
    return;
  }
  const finishedAt = typeof performance === "undefined" ? Date.now() : performance.now();
  if (!installedInteractiveJobPort.observeConsumerTurn("status observer", finishedAt - startedAt))
    return;
  observerNotifyScheduled = true;
  setTimeout(notifyOneInteractiveJobObserver, 0);
}

/* ../../🚀️browser-boot/🟦️.ts */
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
    pluginVariant: bounded(params.get("plugin") ?? "s", "plugin"),
    appRole: params.get("role") === "viewer" ? "viewer" : "editor",
    ...hubUrl ? { hub: { hubUrl: bounded(hubUrl, "hub"), user: bounded(params.get("user") ?? "", "user"), dataDir: bounded(params.get("dataDir") ?? "", "dataDir") } } : {}
  };
}
function canvasElement() {
  const canvas = document.createElement("canvas");
  canvas.tabIndex = 0;
  canvas.setAttribute("aria-label", locale() === "de" ? "Semio Arbeitsfläche" : "Semio workspace");
  canvas.style.cssText = "display:block;width:100%;height:100%;touch-action:none;outline:none;";
  return canvas;
}
function statusElement(root) {
  const status = document.createElement("div");
  status.setAttribute("role", "status");
  status.setAttribute("aria-live", "polite");
  status.style.cssText = "position:fixed;left:12px;bottom:12px;padding:6px 9px;background:#001117cc;color:#d7f7ff;font:12px monospace;z-index:9997;";
  root.appendChild(status);
  return status;
}
function renderFault(root, code, detail) {
  const banner = document.createElement("div");
  banner.setAttribute("role", "alert");
  banner.style.cssText = "position:fixed;inset:0;padding:24px;background:#2a0a0acc;color:#ffb4b4;font:14px monospace;white-space:pre-wrap;overflow:auto;z-index:9999;";
  banner.textContent = `wgpu renderer fault:

${code}: ${detail}

No UI-thread frame fallback was attempted.`;
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
  const transport = new BrowserFrameTransport({
    worker,
    boot: { bindingsModuleUrl: RENDERER_MODULE_URL, bindingsWasmUrl: RENDERER_WASM_URL, canvas: offscreen, width, height, dpr, pluginVariant: descriptor.pluginVariant, locale: locale(), appRole: descriptor.appRole, hub: descriptor.hub },
    requestAnimationFrame: (callback) => window.requestAnimationFrame(callback),
    cancelAnimationFrame: (handle) => window.cancelAnimationFrame(handle),
    onProgress: (stage, progress) => {
      status.textContent = `${stage} ${Math.round(progress * 100)}%`;
    },
    onReady: () => {
      status.remove();
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
    onFault: (code, detail) => {
      cleanupInput();
      renderFault(root, code, detail);
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
