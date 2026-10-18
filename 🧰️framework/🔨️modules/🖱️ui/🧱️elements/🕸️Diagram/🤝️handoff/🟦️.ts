export type DiagramHandoffKind = "consumer-publication" | "drag-move" | "drag-start" | "drag-stop" | "edge-publication" | "host-publication" | "state-publication";

export interface DiagramHandoffViolation {
  readonly elapsedMs: number;
  readonly fault?: string;
  readonly generation: number;
  readonly kind: DiagramHandoffKind;
}

export interface DiagramHandoffStatus {
  lastValidPublicationGeneration(): number | undefined;
  violations(): readonly DiagramHandoffViolation[];
}

interface DiagramHandoffTask {
  readonly consumer?: object;
  readonly generation: number;
  readonly sequence: number;
  readonly valid: () => boolean;
  readonly run: () => void;
}

const diagramHandoffKinds: readonly DiagramHandoffKind[] = ["host-publication", "state-publication", "consumer-publication", "edge-publication", "drag-start", "drag-move", "drag-stop"];

export class DiagramHandoffQueue implements DiagramHandoffStatus {
  private generation = 0;
  private handle?: number;
  private lastValidPublication?: DiagramHandoffTask;
  private sequence = 0;
  private readonly quarantined = new WeakSet<object>();
  private readonly tasks: Partial<Record<DiagramHandoffKind, DiagramHandoffTask>> = {};
  private readonly violationLog: DiagramHandoffViolation[] = [];

  lastValidPublicationGeneration(): number | undefined {
    return this.lastValidPublication?.generation;
  }

  violations(): readonly DiagramHandoffViolation[] {
    return this.violationLog;
  }

  enqueue(kind: DiagramHandoffKind, consumer: object | undefined, valid: () => boolean, run: () => void): number | undefined {
    if (consumer && this.quarantined.has(consumer)) return undefined;
    const generation = ++this.generation;
    this.tasks[kind] = Object.freeze({ consumer, generation, run, sequence: ++this.sequence, valid });
    this.schedule();
    return generation;
  }

  invalidate(...kinds: DiagramHandoffKind[]): void {
    for (const kind of kinds) this.tasks[kind] = undefined;
  }

  stop(): void {
    this.invalidate(...diagramHandoffKinds);
    if (this.handle !== undefined && typeof window !== "undefined") window.clearTimeout(this.handle);
    this.handle = undefined;
  }

  private schedule(): void {
    if (this.handle !== undefined || typeof window === "undefined") return;
    this.handle = window.setTimeout(() => this.consume(), 0);
  }

  private consume(): void {
    this.handle = undefined;
    let kind: DiagramHandoffKind | undefined;
    let task: DiagramHandoffTask | undefined;
    for (const candidate of diagramHandoffKinds) {
      const current = this.tasks[candidate];
      if (current && (!task || current.sequence < task.sequence)) {
        kind = candidate;
        task = current;
      }
    }
    if (!kind || !task) return;
    this.tasks[kind] = undefined;
    if (diagramHandoffKinds.some((candidate) => this.tasks[candidate])) this.schedule();
    if (!task.valid() || (task.consumer && this.quarantined.has(task.consumer))) return;
    let failed = false;
    let fault: string | undefined;
    const started = Date.now();
    try {
      task.run();
    } catch (error) {
      failed = true;
      fault = error instanceof Error ? error.message : typeof error === "string" ? error : "Unknown Diagram handoff fault";
    }
    const elapsedMs = Date.now() - started;
    if (failed || elapsedMs >= 8) {
      if (task.consumer) this.quarantined.add(task.consumer);
      this.violationLog.push({ elapsedMs, fault, generation: task.generation, kind });
      if (this.violationLog.length > 16) this.violationLog.shift();
    } else if (kind === "consumer-publication") this.lastValidPublication = task;
  }
}
