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
  /** @emoji ⏱️ Prices one consumer turn against the isolate's UI-turn ceiling. Answers `false` only to
   * ask the caller to YIELD the rest of its work to a later macrotask — never that the port has died;
   * a turn over the ceiling is a recorded measurement, not a verdict. */
  observeConsumerTurn(site: string, durationMs: number): boolean;
  /** @emoji 💥️ Reports a consumer that THREW — the only consumer-side condition that may quarantine. */
  reportConsumerFault(site: string, detail: string): void;
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
  reportConsumerFault: () => {},
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
  reportConsumerFault: (site, detail) => installedInteractiveJobPort.reportConsumerFault(site, detail),
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
  } catch (error) {
    installedInteractiveJobPort.reportConsumerFault("status observer", error instanceof Error ? error.message : String(error));
    return;
  }
  const finishedAt = typeof performance === "undefined" ? Date.now() : performance.now();
  installedInteractiveJobPort.observeConsumerTurn("status observer", finishedAt - startedAt);
  observerNotifyScheduled = true;
  setTimeout(notifyOneInteractiveJobObserver, 0);
}
// #endregion 🔌️InteractiveJobPort
