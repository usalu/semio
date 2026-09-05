import {
  DIAGRAM_LAYOUT_CODEC_KIND,
  createDiagramLayoutWorkerJob,
  type DiagramLayoutDescriptor,
  type DiagramLayoutIngressPage,
  type DiagramLayoutWorkerJob,
} from "../../../../../../../../🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/📐️layout.ts";
import {
  INTERACTIVE_JOB_INPUT_BYTE_CAPACITY,
  INTERACTIVE_JOB_INPUT_ITEM_CAPACITY,
  INTERACTIVE_JOB_PAGE_BYTE_CAPACITY,
  INTERACTIVE_JOB_PAGE_ITEM_CAPACITY,
  INTERACTIVE_JOB_SLOT_CAPACITY,
  INTERACTIVE_JOB_PORT_BYTE_CAPACITY,
  INTERACTIVE_JOB_PORT_ITEM_CAPACITY,
  type InteractiveJobDescriptor,
  type InteractiveJobPage,
  type InteractiveJobUiMessage,
  type InteractiveJobWorkerMessage,
} from "../🔌️browser-interactive-job-port/🟦️.ts";

// #region 🧬️Registry
export type InteractiveWorkerStep = { readonly deadlineMs: number; readonly fuel: number };

export interface InteractiveWorkerJob {
  acceptInput(payload: unknown): boolean;
  cancel(): void;
  close(step: InteractiveWorkerStep): boolean;
  step(step: InteractiveWorkerStep): "running" | "complete" | "cancelled" | "fault";
  takeOutput(): InteractiveJobPage | undefined;
  terminal(): { readonly status: "complete" | "cancelled" | "fault"; readonly detail?: string } | undefined;
}

export interface InteractiveWorkerDescriptor {
  readonly kind: string;
  readonly inputPageItems: number;
  readonly outputPageItems: number;
  readonly pageBytes: number;
  create(descriptor: InteractiveJobDescriptor): InteractiveWorkerJob | undefined;
}

const DIAGRAM_DESCRIPTOR: InteractiveWorkerDescriptor = {
  kind: DIAGRAM_LAYOUT_CODEC_KIND,
  inputPageItems: 64,
  outputPageItems: 128,
  pageBytes: 16 * 1024,
  create(descriptor) {
    const payload = descriptor.payload as Partial<DiagramLayoutDescriptor>;
    if (payload.kind !== DIAGRAM_LAYOUT_CODEC_KIND || payload.generation !== descriptor.generation) return undefined;
    return new DiagramInteractiveWorkerJob(createDiagramLayoutWorkerJob(payload as DiagramLayoutDescriptor), descriptor.generation);
  },
};

export const INTERACTIVE_WORKER_DESCRIPTORS: readonly InteractiveWorkerDescriptor[] = Object.freeze([DIAGRAM_DESCRIPTOR]);

class DiagramInteractiveWorkerJob implements InteractiveWorkerJob {
  constructor(private readonly job: DiagramLayoutWorkerJob, private readonly generation: number) {}

  acceptInput(payload: unknown): boolean {
    return this.job.ingest(payload as DiagramLayoutIngressPage);
  }

  cancel(): void {
    this.job.cancel(this.generation);
  }

  close(step: InteractiveWorkerStep): boolean {
    return this.job.close({ deadline: step.deadlineMs, fuel: step.fuel });
  }

  step(step: InteractiveWorkerStep): "running" | "complete" | "cancelled" | "fault" {
    return this.job.step({ deadline: step.deadlineMs, fuel: step.fuel, generation: this.generation }).status;
  }

  takeOutput(): InteractiveJobPage | undefined {
    const page = this.job.takePreviewPage() ?? this.job.takeResultPage();
    if (!page) return undefined;
    return { itemCount: page.values.length, byteLength: page.values.length * 32, payload: page, complete: page.complete };
  }

  terminal(): { readonly status: "complete" | "cancelled" | "fault"; readonly detail?: string } | undefined {
    const terminal = this.job.terminal();
    if (!terminal) return undefined;
    return terminal.status === "fault" ? { status: "fault", detail: terminal.reason } : { status: terminal.status };
  }
}
// #endregion 🧬️Registry

// #region ⏱️Scheduler
type SchedulerSlot = {
  readonly descriptor: InteractiveJobDescriptor;
  readonly job: InteractiveWorkerJob;
  inputCursor: number;
  inputItems: number;
  inputBytes: number;
  outputItems: number;
  outputBytes: number;
  phase: "ingress" | "running" | "publishing" | "closing";
  afterPublish: "running" | "closing";
  terminalSent: boolean;
};

export class InteractiveWorkerScheduler {
  private readonly slots = new Array<SchedulerSlot | undefined>(INTERACTIVE_JOB_SLOT_CAPACITY);
  private cursor = 0;
  private scheduled = false;
  private closed = false;
  private closeCursor = 0;
  private reservedItems = 0;
  private reservedBytes = 0;

  constructor(
    private readonly lifecycle: number,
    private readonly descriptors: readonly InteractiveWorkerDescriptor[],
    private readonly post: (message: InteractiveJobWorkerMessage) => void,
    private readonly schedule: (callback: () => void) => void,
    private readonly now: () => number,
    private readonly fault: (detail: string) => void,
  ) {}

  receive(message: InteractiveJobUiMessage): boolean {
    try {
      return this.receiveOwned(message);
    } catch (error) {
      return this.protocolFault(`interactive job callback threw: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  private receiveOwned(message: InteractiveJobUiMessage): boolean {
    if (!message.kind.startsWith("job-")) return false;
    if (this.closed || message.lifecycle !== this.lifecycle) return true;
    if (message.kind === "job-submit") return this.submit(message.descriptor);
    if (!admittedCount(message.operation) || !admittedCount(message.generation)) return this.protocolFault("interactive job message identity was invalid");
    const index = this.find(message.operation);
    if (index < 0) return true;
    const slot = this.slots[index]!;
    if (message.generation > slot.descriptor.generation) return this.protocolFault("interactive job future generation");
    if (message.generation < slot.descriptor.generation) return true;
    if (message.kind === "job-cancel") {
      slot.job.cancel();
      slot.phase = "closing";
      this.scheduleRun();
      return true;
    }
    if (slot.phase !== "ingress" || message.cursor !== slot.inputCursor) return this.protocolFault("interactive job ingress cursor mismatch");
    if (!admitPage(message.page, slot.descriptor.inputPageItems, slot.descriptor.pageBytes) || (message.page.itemCount === 0 && !message.page.complete)) return this.protocolFault("interactive job input page exceeded fixed credits");
    const items = slot.inputItems + message.page.itemCount;
    const bytes = slot.inputBytes + message.page.byteLength;
    if (items > slot.descriptor.inputItems || bytes > slot.descriptor.inputBytes) return this.protocolFault("interactive job input credits exhausted");
    if (!slot.job.acceptInput(message.page.payload)) return this.protocolFault("interactive job rejected input ownership");
    slot.inputItems = items;
    slot.inputBytes = bytes;
    slot.inputCursor += message.page.itemCount;
    if (message.page.complete) {
      slot.phase = "running";
      this.scheduleRun();
    } else {
      this.post({ kind: "job-input-pull", lifecycle: this.lifecycle, operation: message.operation, generation: message.generation, cursor: slot.inputCursor, maxItems: slot.descriptor.inputPageItems });
    }
    return true;
  }

  close(): void {
    if (this.closed) return;
    this.closed = true;
    this.closeCursor = 0;
  }

  closeStep(): boolean {
    try {
      return this.closeOwnedStep();
    } catch (error) {
      this.protocolFault(`interactive job close callback threw: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
  }

  private closeOwnedStep(): boolean {
    for (let scanned = 0; scanned < this.slots.length; scanned++) {
      const index = (this.closeCursor + scanned) % this.slots.length;
      const slot = this.slots[index];
      if (!slot) continue;
      if (slot.phase !== "closing") {
        slot.job.cancel();
        slot.phase = "closing";
        this.closeCursor = (index + 1) % this.slots.length;
        return false;
      }
      if (slot.job.close({ deadlineMs: this.now() + 6, fuel: 1024 })) this.releaseSlot(index);
      this.closeCursor = (index + 1) % this.slots.length;
      return false;
    }
    return true;
  }

  private submit(descriptor: InteractiveJobDescriptor): true {
    if (!admitDescriptor(descriptor) || this.find(descriptor.operation) >= 0) {
      this.postTerminal(descriptor, "fault", "interactive job descriptor unavailable or saturated");
      return true;
    }
    const index = this.slots.findIndex((slot) => slot === undefined);
    const factory = this.descriptors.find((candidate) => candidate.kind === descriptor.kind);
    if (factory && (descriptor.inputPageItems !== factory.inputPageItems || descriptor.outputPageItems !== factory.outputPageItems || descriptor.pageBytes !== factory.pageBytes)) {
      this.postTerminal(descriptor, "fault", "interactive job kind limits do not match the static registry");
      return true;
    }
    const reservedItems = descriptor.inputItems + descriptor.outputItems;
    const reservedBytes = descriptor.inputBytes + descriptor.outputBytes;
    if (this.reservedItems + reservedItems > INTERACTIVE_JOB_PORT_ITEM_CAPACITY || this.reservedBytes + reservedBytes > INTERACTIVE_JOB_PORT_BYTE_CAPACITY) {
      this.postTerminal(descriptor, "fault", "interactive job process credits saturated");
      return true;
    }
    const job = factory?.create(descriptor);
    if (index < 0 || !job) {
      this.postTerminal(descriptor, "fault", "interactive job kind unavailable or slots saturated");
      return true;
    }
    this.slots[index] = { descriptor, job, inputCursor: 0, inputItems: 0, inputBytes: 0, outputItems: 0, outputBytes: 0, phase: "ingress", afterPublish: "running", terminalSent: false };
    this.reservedItems += reservedItems;
    this.reservedBytes += reservedBytes;
    this.post({ kind: "job-input-pull", lifecycle: this.lifecycle, operation: descriptor.operation, generation: descriptor.generation, cursor: 0, maxItems: descriptor.inputPageItems });
    return true;
  }

  private scheduleRun(): void {
    if (this.scheduled || this.closed) return;
    this.scheduled = true;
    this.schedule(() => {
      this.scheduled = false;
      const startedAt = this.now();
      try {
        this.runOne();
      } catch (error) {
        this.protocolFault(`interactive job Worker callback threw: ${error instanceof Error ? error.message : String(error)}`);
      }
      if (this.now() - startedAt >= 8) this.protocolFault("interactive job Worker turn exceeded budget");
    });
  }

  private runOne(): void {
    if (this.closed) return;
    for (let scanned = 0; scanned < this.slots.length; scanned++) {
      const index = (this.cursor + scanned) % this.slots.length;
      const slot = this.slots[index];
      if (!slot || slot.phase === "ingress") continue;
      this.cursor = (index + 1) % this.slots.length;
      if (slot.phase === "running") {
        const status = slot.job.step({ deadlineMs: this.now() + 6, fuel: 16_384 });
        if (status !== "running" && status !== "complete" && status !== "cancelled" && status !== "fault") return void this.protocolFault("interactive job returned invalid step status");
        if (status === "running" || status === "complete") {
          slot.phase = "publishing";
          slot.afterPublish = status === "running" ? "running" : "closing";
        } else slot.phase = "closing";
        this.scheduleRun();
        return;
      }
      if (slot.phase === "publishing") {
        const page = slot.job.takeOutput();
        if (!page) slot.phase = slot.afterPublish;
        else {
          if (!admitPage(page, slot.descriptor.outputPageItems, slot.descriptor.pageBytes)) return void this.protocolFault("interactive job output page exceeded fixed credits");
          slot.outputItems += page.itemCount;
          slot.outputBytes += page.byteLength;
          if (slot.outputItems > slot.descriptor.outputItems || slot.outputBytes > slot.descriptor.outputBytes) return void this.protocolFault("interactive job output credits exhausted");
          this.post({ kind: "job-output-page", lifecycle: this.lifecycle, operation: slot.descriptor.operation, generation: slot.descriptor.generation, page });
          if (page.complete) slot.phase = "closing";
          else if (slot.afterPublish === "running") slot.phase = "running";
        }
        this.scheduleRun();
        return;
      }
      if (slot.phase === "closing") {
        const terminal = slot.job.terminal() ?? { status: "cancelled" as const };
        if (terminal.status !== "complete" && terminal.status !== "cancelled" && terminal.status !== "fault") return void this.protocolFault("interactive job returned invalid terminal status");
        if (!slot.terminalSent) {
          this.postTerminal(slot.descriptor, terminal.status, terminal.detail);
          slot.terminalSent = true;
          this.scheduleRun();
          return;
        }
        if (slot.job.close({ deadlineMs: this.now() + 6, fuel: 1024 })) this.releaseSlot(index);
      }
      this.scheduleRun();
      return;
    }
  }

  private find(operation: number): number {
    return this.slots.findIndex((slot) => slot?.descriptor.operation === operation);
  }

  private postTerminal(descriptor: Pick<InteractiveJobDescriptor, "operation" | "generation">, status: "complete" | "cancelled" | "fault", detail?: string): void {
    this.post({ kind: "job-terminal", lifecycle: this.lifecycle, operation: descriptor.operation, generation: descriptor.generation, status, ...(detail === undefined ? {} : { detail }) });
  }

  private releaseSlot(index: number): void {
    const slot = this.slots[index];
    if (!slot) return;
    this.reservedItems -= slot.descriptor.inputItems + slot.descriptor.outputItems;
    this.reservedBytes -= slot.descriptor.inputBytes + slot.descriptor.outputBytes;
    this.slots[index] = undefined;
  }

  private protocolFault(detail: string): true {
    this.close();
    try {
      this.fault(detail);
    } catch {}
    return true;
  }
}

function admitDescriptor(descriptor: InteractiveJobDescriptor): boolean {
  return descriptor.kind.length > 0 && descriptor.kind.length <= 64 && admittedCount(descriptor.operation) && admittedCount(descriptor.generation) && admittedCount(descriptor.inputItems) && admittedCount(descriptor.inputBytes) && admittedCount(descriptor.outputItems) && admittedCount(descriptor.outputBytes) && admittedCount(descriptor.inputPageItems) && admittedCount(descriptor.outputPageItems) && admittedCount(descriptor.pageBytes) && descriptor.inputItems <= INTERACTIVE_JOB_INPUT_ITEM_CAPACITY && descriptor.outputItems <= INTERACTIVE_JOB_INPUT_ITEM_CAPACITY && descriptor.inputBytes <= INTERACTIVE_JOB_INPUT_BYTE_CAPACITY && descriptor.outputBytes <= INTERACTIVE_JOB_INPUT_BYTE_CAPACITY && descriptor.inputPageItems <= INTERACTIVE_JOB_PAGE_ITEM_CAPACITY && descriptor.outputPageItems <= INTERACTIVE_JOB_PAGE_ITEM_CAPACITY && descriptor.pageBytes <= INTERACTIVE_JOB_PAGE_BYTE_CAPACITY;
}

function admitPage(page: InteractiveJobPage, itemCapacity: number, byteCapacity: number): boolean {
  return admittedCount(page.itemCount) && admittedCount(page.byteLength) && typeof page.complete === "boolean" && page.itemCount <= itemCapacity && page.byteLength <= byteCapacity;
}

function admittedCount(value: number): boolean {
  return Number.isSafeInteger(value) && value >= 0;
}
// #endregion ⏱️Scheduler
