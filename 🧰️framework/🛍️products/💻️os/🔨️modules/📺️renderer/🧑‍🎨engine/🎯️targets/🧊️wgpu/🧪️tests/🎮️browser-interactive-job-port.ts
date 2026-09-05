import { describe, expect, it } from "vitest";
import {
  BrowserInteractiveJobPort,
  INTERACTIVE_JOB_INPUT_BYTE_CAPACITY,
  INTERACTIVE_JOB_SLOT_CAPACITY,
  type InteractiveJobDescriptor,
  type InteractiveJobPage,
  type InteractiveJobUiMessage,
  type InteractiveJobWorkerMessage,
} from "../🔌️browser-interactive-job-port/🟦️.ts";
import { InteractiveWorkerScheduler, type InteractiveWorkerDescriptor, type InteractiveWorkerJob, type InteractiveWorkerStep } from "../📇️interactive-job-registry/🟦️.ts";

const page = (complete = true): InteractiveJobPage => ({ itemCount: 1, byteLength: 8, payload: { value: 1 }, complete });
const descriptor = (operation: number, kind = "alpha", generation = 1): InteractiveJobDescriptor => ({ kind, operation, generation, inputItems: 1, inputBytes: 32, outputItems: 1, outputBytes: 32, inputPageItems: 1, outputPageItems: 1, pageBytes: 16, payload: { kind, generation } });
const closeable = <T extends { readInputPage(cursor: number, maxItems: number): InteractiveJobPage; onOutputPage(page: InteractiveJobPage): void; onTerminal(terminal: unknown): void }>(consumer: T) => ({ ...consumer, closeStep: () => true, terminalIsEmpty: () => true });

describe("browser interactive job port", () => {
  it("fails closed before payload ownership on duplicate, slot, and byte saturation", () => {
    const sent: InteractiveJobUiMessage[] = [];
    const port = new BrowserInteractiveJobPort(1, (message) => sent.push(message), () => 0, () => {});
    port.ready();
    const consumer = closeable({ readInputPage: () => page(), onOutputPage: () => {}, onTerminal: () => {} });
    expect(port.submit({ ...descriptor(0), inputBytes: INTERACTIVE_JOB_INPUT_BYTE_CAPACITY + 1 }, consumer)).toBeUndefined();
    for (let operation = 0; operation < INTERACTIVE_JOB_SLOT_CAPACITY; operation++) expect(port.submit(descriptor(operation), consumer)).toBeDefined();
    expect(port.submit(descriptor(0, "beta", 2), consumer)).toBeUndefined();
    expect(port.submit(descriptor(99), consumer)).toBeUndefined();
    expect(sent).toHaveLength(INTERACTIVE_JOB_SLOT_CAPACITY);
  });

  it("serves two concurrent instances and rejects late and future messages", () => {
    const sent: InteractiveJobUiMessage[] = [];
    const outputs: number[] = [];
    const terminals: number[] = [];
    const quarantines: string[] = [];
    const port = new BrowserInteractiveJobPort(1, (message) => sent.push(message), () => 0, (detail) => quarantines.push(detail));
    port.ready();
    for (const operation of [7, 8]) {
      port.submit(descriptor(operation, operation === 7 ? "alpha" : "beta"), closeable({
        readInputPage: () => page(),
        onOutputPage: () => outputs.push(operation),
        onTerminal: () => terminals.push(operation),
      }));
      port.receive({ kind: "job-input-pull", lifecycle: 1, operation, generation: 1, cursor: 0, maxItems: 1 });
      port.receive({ kind: "job-output-page", lifecycle: 1, operation, generation: 1, page: page() });
      port.receive({ kind: "job-terminal", lifecycle: 1, operation, generation: 1, status: "complete" });
      port.receive({ kind: "job-output-page", lifecycle: 1, operation, generation: 1, page: page() });
    }
    expect(sent.filter((message) => message.kind === "job-input-page")).toHaveLength(2);
    expect(outputs).toEqual([7, 8]);
    expect(terminals).toEqual([7, 8]);
    port.submit(descriptor(9), closeable({ readInputPage: () => page(), onOutputPage: () => {}, onTerminal: () => {} }));
    port.receive({ kind: "job-output-page", lifecycle: 1, operation: 9, generation: 2, page: page() });
    expect(port.status).toBe("quarantined");
    expect(quarantines).toEqual(["interactive job returned future generation 2"]);
  });

  it("drains one completed consumer without closing another live job", () => {
    const sent: InteractiveJobUiMessage[] = [];
    const turns: Array<() => void> = [];
    const terminals: number[] = [];
    const port = new BrowserInteractiveJobPort(1, (message) => sent.push(message), () => 0, () => {}, (callback) => turns.push(callback));
    port.ready();
    for (const operation of [1, 2]) {
      port.submit(descriptor(operation), closeable({ readInputPage: () => page(), onOutputPage: () => {}, onTerminal: () => terminals.push(operation) }));
    }
    port.receive({ kind: "job-terminal", lifecycle: 1, operation: 1, generation: 1, status: "complete" });
    port.receive({ kind: "job-terminal", lifecycle: 1, operation: 1, generation: 1, status: "complete" });
    while (turns.length > 0) turns.shift()!();
    expect(port.status).toBe("ready");
    port.receive({ kind: "job-input-pull", lifecycle: 1, operation: 2, generation: 1, cursor: 0, maxItems: 1 });
    expect(sent).toContainEqual(expect.objectContaining({ kind: "job-input-page", operation: 2 }));
    expect(terminals).toEqual([1]);
  });

  it("retains a terminal consumer through bounded close after its callback throws", () => {
    const turns: Array<() => void> = [];
    let closeTurns = 0;
    let terminalEmpty = false;
    const port = new BrowserInteractiveJobPort(1, () => {}, () => 0, () => {}, (callback) => turns.push(callback));
    port.ready();
    port.submit(descriptor(1), {
      readInputPage: () => page(),
      onOutputPage: () => {},
      onTerminal: () => { throw new Error("consumer fault"); },
      closeStep: () => { closeTurns++; terminalEmpty = closeTurns === 2; return terminalEmpty; },
      terminalIsEmpty: () => terminalEmpty,
    });
    port.receive({ kind: "job-terminal", lifecycle: 1, operation: 1, generation: 1, status: "complete" });
    while (turns.length > 0) turns.shift()!();
    expect(port.status).toBe("quarantined");
    expect(closeTurns).toBe(2);
  });

  it("rejects invalid pull/page counts and catches cancel transfer faults", () => {
    const quarantines: string[] = [];
    const port = new BrowserInteractiveJobPort(1, (message) => { if (message.kind === "job-cancel") throw new Error("clone"); }, () => 0, (detail) => quarantines.push(detail));
    port.ready();
    const lease = port.submit(descriptor(1), closeable({ readInputPage: () => page(), onOutputPage: () => {}, onTerminal: () => {} }))!;
    port.receive({ kind: "job-input-pull", lifecycle: 1, operation: 1, generation: 1, cursor: 0, maxItems: Number.NaN });
    expect(port.status).toBe("quarantined");
    expect(quarantines).toEqual(["interactive job pull exceeded fixed credits"]);
    expect(lease.cancel()).toBe(false);

    const cancelFaults: string[] = [];
    const cancelPort = new BrowserInteractiveJobPort(1, (message) => { if (message.kind === "job-cancel") throw new Error("clone"); }, () => 0, (detail) => cancelFaults.push(detail));
    cancelPort.ready();
    const cancelLease = cancelPort.submit(descriptor(2), closeable({ readInputPage: () => page(), onOutputPage: () => {}, onTerminal: () => {} }))!;
    expect(cancelLease.cancel()).toBe(false);
    expect(cancelFaults).toEqual(["cancel transfer threw: clone"]);
  });

  it("rejects repeated input cursors and non-boolean completion flags", () => {
    const cursorFaults: string[] = [];
    const cursorPort = new BrowserInteractiveJobPort(1, () => {}, () => 0, (detail) => cursorFaults.push(detail));
    cursorPort.ready();
    cursorPort.submit(descriptor(1), closeable({ readInputPage: () => page(), onOutputPage: () => {}, onTerminal: () => {} }));
    cursorPort.receive({ kind: "job-input-pull", lifecycle: 1, operation: 1, generation: 1, cursor: 0, maxItems: 1 });
    cursorPort.receive({ kind: "job-input-pull", lifecycle: 1, operation: 1, generation: 1, cursor: 0, maxItems: 1 });
    expect(cursorFaults).toEqual(["interactive job pull exceeded fixed credits"]);

    const pageFaults: string[] = [];
    const pagePort = new BrowserInteractiveJobPort(1, () => {}, () => 0, (detail) => pageFaults.push(detail));
    pagePort.ready();
    pagePort.submit(descriptor(2), closeable({ readInputPage: () => ({ ...page(), complete: 1 as unknown as boolean }), onOutputPage: () => {}, onTerminal: () => {} }));
    pagePort.receive({ kind: "job-input-pull", lifecycle: 1, operation: 2, generation: 1, cursor: 0, maxItems: 1 });
    expect(pageFaults).toEqual(["interactive job page exceeded fixed credits"]);
  });

  it("quarantines an exact consumer overrun and cursor-drains close", () => {
    let now = 0;
    const port = new BrowserInteractiveJobPort(1, () => {}, () => now, () => {});
    port.ready();
    port.submit(descriptor(1), closeable({ readInputPage: () => { now = 2; return page(); }, onOutputPage: () => {}, onTerminal: () => {} }));
    port.receive({ kind: "job-input-pull", lifecycle: 1, operation: 1, generation: 1, cursor: 0, maxItems: 1 });
    expect(port.status).toBe("quarantined");
    let turns = 0;
    while (!port.closeStep()) turns++;
    expect(turns).toBe(1);
  });

  it("publishes bounded readiness snapshots to pre-boot subscribers", () => {
    const snapshots: string[] = [];
    const turns: Array<() => void> = [];
    const port = new BrowserInteractiveJobPort(1, () => {}, () => 0, () => {}, (callback) => turns.push(callback));
    const unsubscribe = port.subscribe(() => snapshots.push(port.getSnapshot().status));
    port.ready();
    while (turns.length > 0) turns.shift()!();
    port.close();
    while (turns.length > 0) turns.shift()!();
    unsubscribe();
    expect(snapshots).toEqual(["ready", "closed"]);
  });
});

class TestJob implements InteractiveWorkerJob {
  private state: "ingress" | "running" | "complete" | "cancelled" = "ingress";
  private emitted = false;
  acceptInput(_payload: unknown): boolean { this.state = "running"; return true; }
  cancel(): void { this.state = "cancelled"; }
  close(_step: InteractiveWorkerStep): boolean { return true; }
  step(_step: InteractiveWorkerStep): "running" | "complete" | "cancelled" | "fault" { if (this.state === "running") this.state = "complete"; return this.state === "ingress" ? "running" : this.state; }
  takeOutput(): InteractiveJobPage | undefined { if (this.state !== "complete" || this.emitted) return undefined; this.emitted = true; return page(); }
  terminal(): { readonly status: "complete" | "cancelled" | "fault" } | undefined { return this.state === "complete" || this.state === "cancelled" ? { status: this.state } : undefined; }
}

describe("interactive Worker scheduler", () => {
  it("runs a static two-kind registry and cancels ingress without a UI fallback", () => {
    const posted: InteractiveJobWorkerMessage[] = [];
    const turns: Array<() => void> = [];
    const faults: string[] = [];
    const factories: InteractiveWorkerDescriptor[] = ["alpha", "beta"].map((kind) => ({ kind, inputPageItems: 1, outputPageItems: 1, pageBytes: 16, create: () => new TestJob() }));
    const scheduler = new InteractiveWorkerScheduler(1, factories, (message) => posted.push(message), (callback) => turns.push(callback), () => 0, (detail) => faults.push(detail));
    scheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: descriptor(1, "alpha") });
    scheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: descriptor(2, "beta") });
    scheduler.receive({ kind: "job-input-page", lifecycle: 1, operation: 1, generation: 1, cursor: 0, page: page() });
    scheduler.receive({ kind: "job-cancel", lifecycle: 1, operation: 2, generation: 1 });
    while (turns.length > 0) turns.shift()!();
    expect(posted).toContainEqual(expect.objectContaining({ kind: "job-terminal", operation: 1, status: "complete" }));
    expect(posted).toContainEqual(expect.objectContaining({ kind: "job-terminal", operation: 2, status: "cancelled" }));
    expect(faults).toEqual([]);
    scheduler.receive({ kind: "job-input-page", lifecycle: 1, operation: 1, generation: 1, cursor: 0, page: page() });
    scheduler.close();
    expect(scheduler.closeStep()).toBe(true);
  });

  it("faults on duplicate operations and input credit violations", () => {
    const posted: InteractiveJobWorkerMessage[] = [];
    const faults: string[] = [];
    const scheduler = new InteractiveWorkerScheduler(1, [{ kind: "alpha", inputPageItems: 1, outputPageItems: 1, pageBytes: 16, create: () => new TestJob() }], (message) => posted.push(message), () => {}, () => 0, (detail) => faults.push(detail));
    scheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: { ...descriptor(3), inputBytes: 4 } });
    scheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: descriptor(3) });
    expect(posted).toContainEqual(expect.objectContaining({ kind: "job-terminal", operation: 3, status: "fault" }));
    scheduler.receive({ kind: "job-input-page", lifecycle: 1, operation: 3, generation: 1, cursor: 0, page: page() });
    expect(faults).toEqual(["interactive job input credits exhausted"]);
  });

  it("cancels between output pages without publishing the remainder", () => {
    class MultiPageJob extends TestJob {
      private remaining = 2;
      override takeOutput(): InteractiveJobPage | undefined {
        if (this.remaining === 0) return undefined;
        this.remaining -= 1;
        return { ...page(this.remaining === 0), payload: { page: this.remaining } };
      }
    }
    const posted: InteractiveJobWorkerMessage[] = [];
    const turns: Array<() => void> = [];
    const scheduler = new InteractiveWorkerScheduler(1, [{ kind: "alpha", inputPageItems: 1, outputPageItems: 1, pageBytes: 16, create: () => new MultiPageJob() }], (message) => posted.push(message), (callback) => turns.push(callback), () => 0, () => {});
    scheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: descriptor(6) });
    scheduler.receive({ kind: "job-input-page", lifecycle: 1, operation: 6, generation: 1, cursor: 0, page: page() });
    turns.shift()!();
    turns.shift()!();
    expect(posted.filter((message) => message.kind === "job-output-page")).toHaveLength(1);
    scheduler.receive({ kind: "job-cancel", lifecycle: 1, operation: 6, generation: 1 });
    while (turns.length > 0) turns.shift()!();
    expect(posted.filter((message) => message.kind === "job-output-page")).toHaveLength(1);
    expect(posted).toContainEqual(expect.objectContaining({ kind: "job-terminal", operation: 6, status: "cancelled" }));
  });

  it("terminates a complete zero-output job", () => {
    class ZeroOutputJob extends TestJob {
      override takeOutput(): InteractiveJobPage | undefined { return undefined; }
    }
    const posted: InteractiveJobWorkerMessage[] = [];
    const turns: Array<() => void> = [];
    const scheduler = new InteractiveWorkerScheduler(1, [{ kind: "alpha", inputPageItems: 1, outputPageItems: 1, pageBytes: 16, create: () => new ZeroOutputJob() }], (message) => posted.push(message), (callback) => turns.push(callback), () => 0, () => {});
    scheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: descriptor(7) });
    scheduler.receive({ kind: "job-input-page", lifecycle: 1, operation: 7, generation: 1, cursor: 0, page: page() });
    for (let count = 0; turns.length > 0 && count < 8; count++) turns.shift()!();
    expect(posted.filter((message) => message.kind === "job-terminal" && message.operation === 7)).toEqual([expect.objectContaining({ status: "complete" })]);
    expect(turns).toHaveLength(0);
  });

  it("faults zero nonterminal ingress without advancing job ownership", () => {
    const faults: string[] = [];
    const scheduler = new InteractiveWorkerScheduler(1, [{ kind: "alpha", inputPageItems: 1, outputPageItems: 1, pageBytes: 16, create: () => new TestJob() }], () => {}, () => {}, () => 0, (detail) => faults.push(detail));
    scheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: descriptor(8) });
    scheduler.receive({ kind: "job-input-page", lifecycle: 1, operation: 8, generation: 1, cursor: 0, page: { itemCount: 0, byteLength: 0, payload: {}, complete: false } });
    expect(faults).toEqual(["interactive job input page exceeded fixed credits"]);
    expect(scheduler.closeStep()).toBe(false);
    expect(scheduler.closeStep()).toBe(false);
    expect(scheduler.closeStep()).toBe(true);
  });

  it("releases process credits only after cursorized job close", () => {
    const posted: InteractiveJobWorkerMessage[] = [];
    const turns: Array<() => void> = [];
    const scheduler = new InteractiveWorkerScheduler(1, [{ kind: "alpha", inputPageItems: 1, outputPageItems: 1, pageBytes: 16, create: () => new TestJob() }], (message) => posted.push(message), (callback) => turns.push(callback), () => 0, () => {});
    const large = { ...descriptor(9), inputBytes: 100 * 1024 * 1024, outputBytes: 40 * 1024 * 1024 };
    scheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: large });
    scheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: { ...large, operation: 10 } });
    expect(posted).toContainEqual(expect.objectContaining({ kind: "job-terminal", operation: 10, detail: "interactive job process credits saturated" }));
    scheduler.receive({ kind: "job-cancel", lifecycle: 1, operation: 9, generation: 1 });
    while (turns.length > 0) turns.shift()!();
    scheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: { ...large, operation: 11 } });
    expect(posted).toContainEqual(expect.objectContaining({ kind: "job-input-pull", operation: 11 }));
  });

  it("contains hostile factory, job, and post callbacks in protocol quarantine", () => {
    const factoryFaults: string[] = [];
    const factoryScheduler = new InteractiveWorkerScheduler(1, [{ kind: "alpha", inputPageItems: 1, outputPageItems: 1, pageBytes: 16, create: () => { throw new Error("factory"); } }], () => {}, () => {}, () => 0, (detail) => factoryFaults.push(detail));
    expect(() => factoryScheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: descriptor(20) })).not.toThrow();
    expect(factoryFaults).toEqual(["interactive job callback threw: factory"]);

    class HostileJob extends TestJob {
      override acceptInput(): boolean { throw new Error("ingress"); }
    }
    const jobFaults: string[] = [];
    const jobScheduler = new InteractiveWorkerScheduler(1, [{ kind: "alpha", inputPageItems: 1, outputPageItems: 1, pageBytes: 16, create: () => new HostileJob() }], () => {}, () => {}, () => 0, (detail) => jobFaults.push(detail));
    jobScheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: descriptor(21) });
    expect(() => jobScheduler.receive({ kind: "job-input-page", lifecycle: 1, operation: 21, generation: 1, cursor: 0, page: page() })).not.toThrow();
    expect(jobFaults).toEqual(["interactive job callback threw: ingress"]);

    const postFaults: string[] = [];
    const postScheduler = new InteractiveWorkerScheduler(1, [{ kind: "alpha", inputPageItems: 1, outputPageItems: 1, pageBytes: 16, create: () => new TestJob() }], () => { throw new Error("post"); }, () => {}, () => 0, (detail) => postFaults.push(detail));
    expect(() => postScheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: descriptor(22) })).not.toThrow();
    expect(postFaults).toEqual(["interactive job callback threw: post"]);

    class StepHostileJob extends TestJob {
      override step(): "running" { throw new Error("step"); }
    }
    const stepTurns: Array<() => void> = [];
    const stepFaults: string[] = [];
    const stepScheduler = new InteractiveWorkerScheduler(1, [{ kind: "alpha", inputPageItems: 1, outputPageItems: 1, pageBytes: 16, create: () => new StepHostileJob() }], () => {}, (callback) => stepTurns.push(callback), () => 0, (detail) => stepFaults.push(detail));
    stepScheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: descriptor(23) });
    stepScheduler.receive({ kind: "job-input-page", lifecycle: 1, operation: 23, generation: 1, cursor: 0, page: page() });
    expect(() => stepTurns.shift()!()).not.toThrow();
    expect(stepFaults).toEqual(["interactive job Worker callback threw: step"]);

    class CancelHostileJob extends TestJob {
      override cancel(): void { throw new Error("cancel"); }
    }
    const cancelFaults: string[] = [];
    const cancelScheduler = new InteractiveWorkerScheduler(1, [{ kind: "alpha", inputPageItems: 1, outputPageItems: 1, pageBytes: 16, create: () => new CancelHostileJob() }], () => {}, () => {}, () => 0, (detail) => cancelFaults.push(detail));
    cancelScheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: descriptor(24) });
    expect(() => cancelScheduler.receive({ kind: "job-cancel", lifecycle: 1, operation: 24, generation: 1 })).not.toThrow();
    expect(cancelFaults).toEqual(["interactive job callback threw: cancel"]);

    class CloseHostileJob extends TestJob {
      override close(): boolean { throw new Error("close"); }
    }
    const closeFaults: string[] = [];
    const closeScheduler = new InteractiveWorkerScheduler(1, [{ kind: "alpha", inputPageItems: 1, outputPageItems: 1, pageBytes: 16, create: () => new CloseHostileJob() }], () => {}, () => {}, () => 0, (detail) => closeFaults.push(detail));
    closeScheduler.receive({ kind: "job-submit", lifecycle: 1, descriptor: descriptor(25) });
    closeScheduler.close();
    expect(closeScheduler.closeStep()).toBe(false);
    expect(() => closeScheduler.closeStep()).not.toThrow();
    expect(closeFaults).toEqual(["interactive job close callback threw: close"]);
  });
});
