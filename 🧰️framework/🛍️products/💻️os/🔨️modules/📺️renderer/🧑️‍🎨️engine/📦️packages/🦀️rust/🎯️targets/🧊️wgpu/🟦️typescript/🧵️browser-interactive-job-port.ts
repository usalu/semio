// #region 🔖️Protocol
export const INTERACTIVE_JOB_SLOT_CAPACITY = 16;
export const INTERACTIVE_JOB_INPUT_ITEM_CAPACITY = 65_536;
export const INTERACTIVE_JOB_INPUT_BYTE_CAPACITY = 256 * 1024 * 1024;
export const INTERACTIVE_JOB_PAGE_ITEM_CAPACITY = 128;
export const INTERACTIVE_JOB_PAGE_BYTE_CAPACITY = 16 * 1024;
export const INTERACTIVE_JOB_UI_BUDGET_MS = 2;
export const INTERACTIVE_JOB_OBSERVER_CAPACITY = 32;
export const INTERACTIVE_JOB_PORT_ITEM_CAPACITY = 262_144;
export const INTERACTIVE_JOB_PORT_BYTE_CAPACITY = 256 * 1024 * 1024;

export type InteractiveJobDescriptor = {
  readonly kind: string;
  readonly operation: number;
  readonly generation: number;
  readonly inputItems: number;
  readonly inputBytes: number;
  readonly outputItems: number;
  readonly outputBytes: number;
  readonly inputPageItems: number;
  readonly outputPageItems: number;
  readonly pageBytes: number;
  readonly payload: unknown;
};
export type InteractiveJobPage = { readonly itemCount: number; readonly byteLength: number; readonly payload: unknown; readonly complete: boolean };
export type InteractiveJobTerminal = { readonly operation: number; readonly generation: number; readonly status: "complete" | "cancelled" | "fault"; readonly detail?: string };
export type InteractiveJobUiMessage =
  | { readonly kind: "job-submit"; readonly lifecycle: number; readonly descriptor: InteractiveJobDescriptor }
  | { readonly kind: "job-input-page"; readonly lifecycle: number; readonly operation: number; readonly generation: number; readonly cursor: number; readonly page: InteractiveJobPage }
  | { readonly kind: "job-cancel"; readonly lifecycle: number; readonly operation: number; readonly generation: number };
export type InteractiveJobWorkerMessage =
  | { readonly kind: "job-input-pull"; readonly lifecycle: number; readonly operation: number; readonly generation: number; readonly cursor: number; readonly maxItems: number }
  | { readonly kind: "job-output-page"; readonly lifecycle: number; readonly operation: number; readonly generation: number; readonly page: InteractiveJobPage }
  | ({ readonly kind: "job-terminal"; readonly lifecycle: number } & InteractiveJobTerminal);

export type InteractiveJobConsumer = {
  readInputPage(cursor: number, maxItems: number): InteractiveJobPage;
  onOutputPage(page: InteractiveJobPage): void;
  onTerminal(terminal: InteractiveJobTerminal): void;
  closeStep(): boolean;
  terminalIsEmpty(): boolean;
};
export type InteractiveJobLease = { readonly operation: number; readonly generation: number; cancel(): boolean };
export type InteractiveJobPortSnapshot = { readonly status: BrowserInteractiveJobPort["status"]; readonly revision: number };
type Slot = { descriptor: InteractiveJobDescriptor; consumer: InteractiveJobConsumer; inputCursor: number; inputItems: number; inputBytes: number; outputItems: number; outputBytes: number; closing: boolean };
// #endregion 🔖️Protocol

// #region 📮️Port
export class BrowserInteractiveJobPort {
  status: "unavailable" | "ready" | "quarantined" | "closed" = "unavailable";
  private readonly slots = new Array<Slot | undefined>(INTERACTIVE_JOB_SLOT_CAPACITY);
  private closeCursor = 0;
  private closeScheduled = false;
  private reservedItems = 0;
  private reservedBytes = 0;
  private readonly observers = new Array<(() => void) | undefined>(INTERACTIVE_JOB_OBSERVER_CAPACITY);
  private observerCursor = 0;
  private observerNotifyScheduled = false;
  private statusRevision = 0;
  private statusSnapshot: InteractiveJobPortSnapshot = { status: "unavailable", revision: 0 };
  private readonly now: () => number;

  constructor(
    private readonly lifecycle: number,
    private readonly send: (message: InteractiveJobUiMessage) => void,
    now: () => number,
    private readonly quarantineConsumer: (detail: string) => void,
    private readonly schedule: (callback: () => void) => void = (callback) => setTimeout(callback, 0),
  ) {
    this.now = now;
  }

  ready(): void {
    if (this.status === "unavailable") {
      this.status = "ready";
      this.publishStatus();
    }
  }

  getSnapshot(): InteractiveJobPortSnapshot {
    return this.statusSnapshot;
  }

  observeConsumerTurn(site: string, durationMs: number): boolean {
    if (durationMs < INTERACTIVE_JOB_UI_BUDGET_MS) return true;
    this.quarantine(`${site} took ${durationMs.toFixed(3)} ms`);
    return false;
  }

  subscribe(listener: () => void): () => void {
    const slot = this.observers.findIndex((entry) => entry === undefined);
    if (slot < 0) throw new Error(`interactive job observer slots exceeded ${INTERACTIVE_JOB_OBSERVER_CAPACITY}`);
    this.observers[slot] = listener;
    return () => { this.observers[slot] = undefined; };
  }

  submit(descriptor: InteractiveJobDescriptor, consumer: InteractiveJobConsumer): InteractiveJobLease | undefined {
    if (this.status !== "ready" || descriptor.kind.length === 0 || descriptor.kind.length > 64) return undefined;
    if (!admittedCount(descriptor.operation) || !admittedCount(descriptor.generation) || !admittedCount(descriptor.inputItems) || !admittedCount(descriptor.inputBytes) || !admittedCount(descriptor.outputItems) || !admittedCount(descriptor.outputBytes) || !admittedCount(descriptor.inputPageItems) || !admittedCount(descriptor.outputPageItems) || !admittedCount(descriptor.pageBytes)) return undefined;
    if (descriptor.inputItems > INTERACTIVE_JOB_INPUT_ITEM_CAPACITY || descriptor.inputBytes > INTERACTIVE_JOB_INPUT_BYTE_CAPACITY || descriptor.outputItems > INTERACTIVE_JOB_INPUT_ITEM_CAPACITY || descriptor.outputBytes > INTERACTIVE_JOB_INPUT_BYTE_CAPACITY) return undefined;
    if (descriptor.inputPageItems > INTERACTIVE_JOB_PAGE_ITEM_CAPACITY || descriptor.outputPageItems > INTERACTIVE_JOB_PAGE_ITEM_CAPACITY || descriptor.pageBytes > INTERACTIVE_JOB_PAGE_BYTE_CAPACITY) return undefined;
    const reservedItems = descriptor.inputItems + descriptor.outputItems;
    const reservedBytes = descriptor.inputBytes + descriptor.outputBytes;
    if (this.reservedItems + reservedItems > INTERACTIVE_JOB_PORT_ITEM_CAPACITY || this.reservedBytes + reservedBytes > INTERACTIVE_JOB_PORT_BYTE_CAPACITY) return undefined;
    if (this.slots.some((slot) => slot?.descriptor.operation === descriptor.operation)) return undefined;
    const index = this.slots.findIndex((slot) => slot === undefined);
    if (index < 0) return undefined;
    this.slots[index] = { descriptor, consumer, inputCursor: 0, inputItems: 0, inputBytes: 0, outputItems: 0, outputBytes: 0, closing: false };
    this.reservedItems += reservedItems;
    this.reservedBytes += reservedBytes;
    try {
      this.send({ kind: "job-submit", lifecycle: this.lifecycle, descriptor });
    } catch {
      this.slots[index] = undefined;
      this.reservedItems -= reservedItems;
      this.reservedBytes -= reservedBytes;
      return undefined;
    }
    return { operation: descriptor.operation, generation: descriptor.generation, cancel: () => this.cancel(descriptor.operation, descriptor.generation) };
  }

  receive(message: InteractiveJobWorkerMessage): boolean {
    if (!message.kind.startsWith("job-")) return false;
    if (message.lifecycle !== this.lifecycle || this.status !== "ready") return true;
    if (!admittedCount(message.operation) || !admittedCount(message.generation)) {
      this.quarantine("interactive job message identity was invalid");
      return true;
    }
    const index = this.slots.findIndex((slot) => slot?.descriptor.operation === message.operation);
    if (index < 0) return true;
    const slot = this.slots[index]!;
    if (message.generation > slot.descriptor.generation) {
      this.quarantine(`interactive job returned future generation ${message.generation}`);
      return true;
    }
    if (message.generation < slot.descriptor.generation) return true;
    if (slot.closing) return true;
    if (message.kind === "job-input-pull") {
      if (!admittedCount(message.cursor) || message.cursor !== slot.inputCursor || !admittedCount(message.maxItems) || message.maxItems === 0 || message.maxItems > slot.descriptor.inputPageItems) {
        this.quarantine("interactive job pull exceeded fixed credits");
        return true;
      }
      const startedAt = this.now();
      let page: InteractiveJobPage;
      try {
        page = slot.consumer.readInputPage(message.cursor, Math.min(message.maxItems, slot.descriptor.inputPageItems));
      } catch (error) {
        this.quarantine(`input consumer threw: ${error instanceof Error ? error.message : String(error)}`);
        return true;
      }
      if (!this.observe(startedAt, "input consumer")) return true;
      if (!this.admitPage(slot, page, true)) return true;
      slot.inputCursor += page.itemCount;
      try {
        this.send({ kind: "job-input-page", lifecycle: this.lifecycle, operation: message.operation, generation: message.generation, cursor: message.cursor, page });
      } catch (error) {
        this.quarantine(`input page transfer threw: ${error instanceof Error ? error.message : String(error)}`);
      }
      return true;
    }
    if (message.kind === "job-output-page") {
      if (!this.admitPage(slot, message.page, false)) return true;
      const startedAt = this.now();
      try {
        slot.consumer.onOutputPage(message.page);
      } catch (error) {
        this.quarantine(`output consumer threw: ${error instanceof Error ? error.message : String(error)}`);
        return true;
      }
      if (!this.observe(startedAt, "output consumer")) return true;
      return true;
    }
    if (message.status !== "complete" && message.status !== "cancelled" && message.status !== "fault") {
      this.quarantine("interactive job returned invalid terminal status");
      return true;
    }
    const terminal = { operation: message.operation, generation: message.generation, status: message.status, ...(message.detail === undefined ? {} : { detail: message.detail }) } satisfies InteractiveJobTerminal;
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
    if (!this.observe(startedAt, "terminal consumer")) return true;
    this.scheduleClose();
    return true;
  }

  close(): void {
    if (this.status === "closed") return;
    this.status = "closed";
    this.closeCursor = 0;
    for (let index = 0; index < this.slots.length; index++) if (this.slots[index]) this.slots[index]!.closing = true;
    this.publishStatus();
    this.scheduleClose();
  }

  closeStep(): boolean {
    if (this.status !== "closed" && this.status !== "quarantined") return false;
    return this.drainClosingStep();
  }

  private drainClosingStep(): boolean {
    while (this.closeCursor < this.slots.length && (!this.slots[this.closeCursor] || !this.slots[this.closeCursor]!.closing)) this.closeCursor++;
    if (this.closeCursor === this.slots.length) return true;
    const slot = this.slots[this.closeCursor]!;
    const startedAt = this.now();
    let complete = false;
    try {
      complete = slot.consumer.closeStep();
      if (complete) complete = slot.consumer.terminalIsEmpty();
    } catch (error) {
      this.quarantine(`consumer close threw: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
    if (!this.observe(startedAt, "consumer close")) return false;
    if (complete) {
      this.releaseSlot(this.closeCursor);
      this.closeCursor++;
    }
    return false;
  }

  quarantineFromOwner(): void {
    if (this.status === "closed") return;
    this.status = "quarantined";
    this.closeCursor = 0;
    for (let index = 0; index < this.slots.length; index++) if (this.slots[index]) this.slots[index]!.closing = true;
    this.publishStatus();
    this.scheduleClose();
  }

  private cancel(operation: number, generation: number): boolean {
    if (this.status !== "ready") return false;
    const slot = this.slots.find((candidate) => candidate?.descriptor.operation === operation);
    if (!slot || slot.descriptor.generation !== generation) return false;
    try {
      this.send({ kind: "job-cancel", lifecycle: this.lifecycle, operation, generation });
    } catch (error) {
      this.quarantine(`cancel transfer threw: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
    return true;
  }

  private admitPage(slot: Slot, page: InteractiveJobPage, input: boolean): boolean {
    const pageItemLimit = input ? slot.descriptor.inputPageItems : slot.descriptor.outputPageItems;
    if (!admittedCount(page.itemCount) || !admittedCount(page.byteLength) || typeof page.complete !== "boolean" || (page.itemCount === 0 && !page.complete) || page.itemCount > pageItemLimit || page.byteLength > slot.descriptor.pageBytes) {
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
    if ((page.complete && items !== itemLimit) || (!page.complete && items >= itemLimit)) {
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

  private observe(startedAt: number, site: string): boolean {
    const duration = this.now() - startedAt;
    if (duration < INTERACTIVE_JOB_UI_BUDGET_MS) return true;
    this.quarantine(`${site} took ${duration.toFixed(3)} ms`);
    return false;
  }

  private quarantine(detail: string): void {
    if (this.status !== "ready") return;
    this.status = "quarantined";
    this.closeCursor = 0;
    for (let index = 0; index < this.slots.length; index++) if (this.slots[index]) this.slots[index]!.closing = true;
    this.publishStatus();
    this.scheduleClose();
    this.quarantineConsumer(detail);
  }

  private notifyObservers(): void {
    this.observerCursor = 0;
    if (this.observerNotifyScheduled) return;
    this.observerNotifyScheduled = true;
    this.schedule(() => this.notifyOneObserver());
  }

  private publishStatus(): void {
    this.statusRevision += 1;
    this.statusSnapshot = { status: this.status, revision: this.statusRevision };
    this.notifyObservers();
  }

  private notifyOneObserver(): void {
    this.observerNotifyScheduled = false;
    while (this.observerCursor < this.observers.length && !this.observers[this.observerCursor]) this.observerCursor++;
    if (this.observerCursor === this.observers.length) return;
    const observer = this.observers[this.observerCursor++]!;
    const startedAt = this.now();
    try {
      observer();
    } catch (error) {
      this.quarantine(`status observer threw: ${error instanceof Error ? error.message : String(error)}`);
      return;
    }
    if (!this.observe(startedAt, "status observer")) return;
    this.observerNotifyScheduled = true;
    this.schedule(() => this.notifyOneObserver());
  }

  private releaseSlot(index: number): void {
    const slot = this.slots[index];
    if (!slot) return;
    this.reservedItems -= slot.descriptor.inputItems + slot.descriptor.outputItems;
    this.reservedBytes -= slot.descriptor.inputBytes + slot.descriptor.outputBytes;
    this.slots[index] = undefined;
  }

  private scheduleClose(): void {
    if (this.closeScheduled) return;
    this.closeScheduled = true;
    this.schedule(() => {
      this.closeScheduled = false;
      this.closeCursor = 0;
      if (!this.drainClosingStep()) this.scheduleClose();
    });
  }
}

function admittedCount(value: number): boolean {
  return Number.isSafeInteger(value) && value >= 0;
}
// #endregion 📮️Port
