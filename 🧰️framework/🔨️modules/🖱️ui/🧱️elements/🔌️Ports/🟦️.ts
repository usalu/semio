// #region 🔌️InteractiveJobPort
export interface InteractiveJobDescriptor {
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
}

export interface InteractiveJobPage {
  readonly itemCount: number;
  readonly byteLength: number;
  readonly payload: unknown;
  readonly complete: boolean;
}

export interface InteractiveJobTerminal {
  readonly operation: number;
  readonly generation: number;
  readonly status: "complete" | "cancelled" | "fault";
  readonly detail?: string;
}

export interface InteractiveJobLease {
  readonly operation: number;
  readonly generation: number;
  cancel(): boolean;
}

export interface InteractiveJobPortSnapshot {
  readonly status: "unavailable" | "ready" | "quarantined" | "closed";
  readonly revision: number;
}

export interface InteractiveJobPort {
  readonly status: "unavailable" | "ready" | "quarantined" | "closed";
  getSnapshot(): InteractiveJobPortSnapshot;
  subscribe(listener: () => void): () => void;
  observeConsumerTurn(site: string, durationMs: number): boolean;
  submit(
    descriptor: InteractiveJobDescriptor,
    consumer: {
      readInputPage(cursor: number, maxItems: number): InteractiveJobPage;
      onOutputPage(page: InteractiveJobPage): void;
      onTerminal(terminal: InteractiveJobTerminal): void;
      closeStep(): boolean;
      terminalIsEmpty(): boolean;
    },
  ): InteractiveJobLease | undefined;
}

const unavailableInteractiveJobPort: InteractiveJobPort = {
  status: "unavailable",
  getSnapshot: () => ({ status: "unavailable", revision: 0 }),
  subscribe: () => () => {},
  observeConsumerTurn: () => true,
  submit: () => undefined,
};

const INTERACTIVE_JOB_OBSERVER_CAPACITY = 32;
const interactiveJobObservers = new Array<(() => void) | undefined>(INTERACTIVE_JOB_OBSERVER_CAPACITY);
let installedInteractiveJobPort = unavailableInteractiveJobPort;
let unsubscribeInstalled = () => {};
let interactiveJobRevision = 0;
let interactiveJobSnapshot: InteractiveJobPortSnapshot = { status: "unavailable", revision: 0 };
let observerCursor = 0;
let observerNotifyScheduled = false;

export const interactiveJobPort: InteractiveJobPort = {
  get status() { return installedInteractiveJobPort.status; },
  getSnapshot: () => interactiveJobSnapshot,
  observeConsumerTurn: (site, durationMs) => installedInteractiveJobPort.observeConsumerTurn(site, durationMs),
  subscribe(listener) {
    const slot = interactiveJobObservers.findIndex((entry) => entry === undefined);
    if (slot < 0) throw new Error(`interactive job observer slots exceeded ${INTERACTIVE_JOB_OBSERVER_CAPACITY}`);
    interactiveJobObservers[slot] = listener;
    return () => { interactiveJobObservers[slot] = undefined; };
  },
  submit: (descriptor, consumer) => installedInteractiveJobPort.submit(descriptor, consumer),
};

export function setInteractiveJobPort(port: InteractiveJobPort): InteractiveJobPort {
  const previous = installedInteractiveJobPort;
  unsubscribeInstalled();
  installedInteractiveJobPort = port;
  unsubscribeInstalled = port.subscribe(publishInteractiveJobSnapshot);
  publishInteractiveJobSnapshot();
  return previous;
}

function publishInteractiveJobSnapshot(): void {
  interactiveJobRevision += 1;
  interactiveJobSnapshot = { status: installedInteractiveJobPort.status, revision: interactiveJobRevision };
  observerCursor = 0;
  if (observerNotifyScheduled) return;
  observerNotifyScheduled = true;
  setTimeout(notifyOneInteractiveJobObserver, 0);
}

function notifyOneInteractiveJobObserver(): void {
  observerNotifyScheduled = false;
  while (observerCursor < interactiveJobObservers.length && !interactiveJobObservers[observerCursor]) observerCursor++;
  if (observerCursor === interactiveJobObservers.length) return;
  const observer = interactiveJobObservers[observerCursor++]!;
  const startedAt = typeof performance === "undefined" ? Date.now() : performance.now();
  try {
    observer();
  } catch {
    installedInteractiveJobPort.observeConsumerTurn("status observer threw", Number.POSITIVE_INFINITY);
    return;
  }
  const finishedAt = typeof performance === "undefined" ? Date.now() : performance.now();
  if (!installedInteractiveJobPort.observeConsumerTurn("status observer", finishedAt - startedAt)) return;
  observerNotifyScheduled = true;
  setTimeout(notifyOneInteractiveJobObserver, 0);
}
// #endregion 🔌️InteractiveJobPort
