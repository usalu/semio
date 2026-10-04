import { createContinuationScheduler, type ContinuationPorts, type ContinuationCancel } from "../../🟦️.ts";

//#region 🧪️VirtualHost
/** 🕰️ A deterministic event loop: a macrotask FIFO, a timer wheel and a clock that only advances when
 * nothing else can run. It models the ONE property the browser gives us and we depend on — a posted
 * macrotask runs before a timer whose deadline has not arrived — without any real waiting. */
export type VirtualContinuationHost = Readonly<{
  ports: ContinuationPorts;
  nowMs: () => number;
  /** ⏩️ Advances the virtual clock to `untilMs` (default: until nothing is left), running everything
   * that comes due. Returns the number of callbacks run. Throws past `stepBudget` so a runaway chain
   * is a test failure rather than a hang. */
  drain: (untilMs?: number, stepBudget?: number) => number;
  /** 📏️ Whether anything at all is still queued. */
  idle: () => boolean;
}>;

/** 🏭️ Builds a {@link VirtualContinuationHost}. */
export const createVirtualContinuationHost = (): VirtualContinuationHost => {
  let nowMs = 0;
  let handleSeq = 0;
  const macrotasks: (() => void)[] = [];
  const timers = new Map<number, { readonly runAtMs: number; readonly run: () => void }>();
  const earliestTimerMs = (): number => {
    let earliest = Number.POSITIVE_INFINITY;
    for (const timer of timers.values()) if (timer.runAtMs < earliest) earliest = timer.runAtMs;
    return earliest;
  };
  const ports: ContinuationPorts = {
    postMacrotask: (run) => {
      macrotasks.push(run);
    },
    setTimer: (run, delayMs) => {
      handleSeq += 1;
      timers.set(handleSeq, { runAtMs: nowMs + Math.max(0, delayMs), run });
      return handleSeq;
    },
    clearTimer: (handle) => {
      timers.delete(handle as number);
    },
    now: () => nowMs,
  };
  return Object.freeze({
    ports,
    nowMs: () => nowMs,
    idle: () => macrotasks.length === 0 && timers.size === 0,
    drain: (untilMs = Number.POSITIVE_INFINITY, stepBudget = 100_000) => {
      let steps = 0;
      for (;;) {
        if (steps > stepBudget) throw new Error(`virtual continuation host exceeded its ${stepBudget}-step budget at ${nowMs} ms`);
        if (macrotasks.length > 0) {
          steps += 1;
          macrotasks.shift()!();
          continue;
        }
        const nextMs = earliestTimerMs();
        if (!Number.isFinite(nextMs) || nextMs > untilMs) {
          if (Number.isFinite(untilMs) && untilMs > nowMs) nowMs = untilMs;
          return steps;
        }
        nowMs = Math.max(nowMs, nextMs);
        for (const [handle, timer] of [...timers]) {
          if (timer.runAtMs > nowMs) continue;
          timers.delete(handle);
          steps += 1;
          timer.run();
          break;
        }
      }
    },
  });
};
//#endregion 🧪️VirtualHost

//#region 🧫️Fixture
/** 🧫️ One scheduling request in a fixture case. `repeat` models a self-re-arming continuation chain —
 * exactly the guest's `Effect::DispatchAction { delay_ms: 0 }` re-arm — by re-scheduling itself with
 * the same delay until it has run `repeat` times; each run is recorded as `label#n`. */
export type ContinuationRequestCase = Readonly<{
  label: string;
  atMs: number;
  delayMs: number;
  key?: string;
  repeat?: number;
  cancelAtMs?: number;
}>;

/** 🧫️ One language-agnostic scheduling law: what was asked for, in what order it must run, and at
 * what millisecond the last callback must land. */
export type ContinuationCase = Readonly<{
  name: string;
  law: string;
  requests: readonly ContinuationRequestCase[];
  expectedOrder: readonly string[];
  expectedCompletionMs: number;
}>;

export type ContinuationSuite = Readonly<{ law: string; provenance: string; fairness: Readonly<{ deadlineMs: number; maximumContinuations: number; expectedDeadlineObserved: boolean }>; cases: readonly ContinuationCase[] }>;

export type ContinuationCaseRun = Readonly<{ order: readonly string[]; completionMs: number }>;

/** ▶️ Runs one case on a virtual clock — no real timers, so the result is a pure function of the
 * fixture. This is the oracle the vitest suite and the node twin both compare against. */
export const runContinuationCase = (testCase: ContinuationCase): ContinuationCaseRun => {
  const host = createVirtualContinuationHost();
  const scheduler = createContinuationScheduler(host.ports);
  const order: string[] = [];
  let completionMs = 0;
  const cancels = new Map<string, ContinuationCancel>();
  const submit = (request: ContinuationRequestCase): void => {
    const total = Math.max(1, request.repeat ?? 1);
    let run = 0;
    const step = (): void => {
      run += 1;
      order.push(total === 1 ? request.label : `${request.label}#${run}`);
      completionMs = host.nowMs();
      if (run < total) cancels.set(request.label, scheduler.schedule(step, request.delayMs, request.key));
    };
    cancels.set(request.label, scheduler.schedule(step, request.delayMs, request.key));
  };
  const milestones = [...new Set(testCase.requests.flatMap((request) => [request.atMs, ...(request.cancelAtMs === undefined ? [] : [request.cancelAtMs])]))].sort((a, b) => a - b);
  for (const milestone of milestones) {
    host.drain(milestone);
    for (const request of testCase.requests) if (request.atMs === milestone) submit(request);
    for (const request of testCase.requests) if (request.cancelAtMs === milestone) cancels.get(request.label)?.();
  }
  host.drain();
  return { order, completionMs };
};

/** ▶️ The same case on the REAL host event loop — the platform's own continuation queue and
 * `setTimeout`, no virtual clock anywhere. It is the independent oracle for
 * {@link runContinuationCase}: if the browser/node event loop and our virtual model disagree about a
 * declared law, the law is wrong. Resolves once every expected callback has run, or rejects at
 * `timeoutMs`. */
export const runContinuationCaseLive = async (testCase: ContinuationCase, timeoutMs = 5_000): Promise<ContinuationCaseRun & { readonly elapsedMs: number }> => {
  const scheduler = createContinuationScheduler();
  const order: string[] = [];
  const startedAtMs = Date.now();
  const cancels = new Map<string, ContinuationCancel>();
  const expected = testCase.expectedOrder.length;
  let settle: (() => void) | null = null;
  const done = new Promise<void>((resolve) => {
    settle = resolve;
  });
  const record = (label: string): void => {
    order.push(label);
    if (order.length >= expected) settle?.();
  };
  const submit = (request: ContinuationRequestCase): void => {
    const total = Math.max(1, request.repeat ?? 1);
    let run = 0;
    const step = (): void => {
      run += 1;
      record(total === 1 ? request.label : `${request.label}#${run}`);
      if (run < total) cancels.set(request.label, scheduler.schedule(step, request.delayMs, request.key));
    };
    cancels.set(request.label, scheduler.schedule(step, request.delayMs, request.key));
  };
  const at = (delayMs: number, run: () => void): void => {
    if (delayMs <= 0) run();
    else setTimeout(run, delayMs);
  };
  for (const request of testCase.requests) at(request.atMs, () => submit(request));
  for (const request of testCase.requests) if (request.cancelAtMs !== undefined) at(request.cancelAtMs, () => cancels.get(request.label)?.());
  const guard = setTimeout(() => settle?.(), timeoutMs);
  await done;
  clearTimeout(guard);
  // 🩺 One extra unthrottled turn so a callback that should NOT have run gets its chance to prove it did.
  await scheduler.yieldContinuation();
  scheduler.dispose();
  return { order, completionMs: Date.now() - startedAtMs, elapsedMs: Date.now() - startedAtMs };
};

/** 🧪️ Checks one case both ways and returns one human-readable fault per violation, empty when the
 * law holds. Shared by the vitest suite and the node twin so the two cannot drift. */
export const continuationCaseFaults = (testCase: ContinuationCase, run: ContinuationCaseRun, toleranceMs = 0): readonly string[] => {
  const faults: string[] = [];
  if (run.order.join(" → ") !== testCase.expectedOrder.join(" → ")) faults.push(`${testCase.name}: ran [${run.order.join(", ")}], expected [${testCase.expectedOrder.join(", ")}]`);
  if (run.completionMs < testCase.expectedCompletionMs - toleranceMs || run.completionMs > testCase.expectedCompletionMs + toleranceMs)
    faults.push(`${testCase.name}: finished at ${Math.round(run.completionMs)} ms, expected ${testCase.expectedCompletionMs} ms ±${toleranceMs}`);
  return faults;
};
//#endregion 🧫️Fixture
